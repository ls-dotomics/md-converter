use std::fmt;
use std::path::{Path, PathBuf};

use tokio::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Docx,
    Pdf,
}

impl OutputFormat {
    pub const ALL: [OutputFormat; 2] = [OutputFormat::Docx, OutputFormat::Pdf];

    pub fn label(&self) -> &'static str {
        match self {
            OutputFormat::Docx => "DOCX",
            OutputFormat::Pdf => "PDF",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Docx => "docx",
            OutputFormat::Pdf => "pdf",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug)]
pub enum ConvertError {
    BinaryMissing(&'static str),
    Io(std::io::Error),
    PandocFailed(String),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::BinaryMissing(name) => {
                write!(f, "Bundled {name} binary is missing from the app.")
            }
            ConvertError::Io(e) => write!(f, "I/O error: {e}"),
            ConvertError::PandocFailed(msg) => write!(f, "Conversion failed: {}", msg.trim()),
        }
    }
}

impl std::error::Error for ConvertError {}

impl From<std::io::Error> for ConvertError {
    fn from(e: std::io::Error) -> Self {
        ConvertError::Io(e)
    }
}

pub async fn convert_file(
    input: PathBuf,
    output: PathBuf,
    format: OutputFormat,
) -> Result<PathBuf, ConvertError> {
    let pandoc = resource_path("pandoc").ok_or(ConvertError::BinaryMissing("pandoc"))?;

    let mut cmd = Command::new(&pandoc);
    cmd.arg(&input).arg("-o").arg(&output);

    match format {
        OutputFormat::Docx => {
            cmd.arg("--standalone");
        }
        OutputFormat::Pdf => {
            let typst = resource_path("typst").ok_or(ConvertError::BinaryMissing("typst"))?;
            cmd.arg(format!("--pdf-engine={}", typst.display()));
        }
    }

    if let Some(parent) = input.parent() {
        cmd.current_dir(parent);
    }

    let output_result = cmd.output().await?;
    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr).to_string();
        return Err(ConvertError::PandocFailed(stderr));
    }

    Ok(output)
}

fn resource_path(name: &str) -> Option<PathBuf> {
    // 1. Inside an .app bundle: <bundle>/Contents/Resources/[vendor/]<name>
    if let Ok(exe) = std::env::current_exe() {
        if let Some(resources_dir) = exe.parent().and_then(|p| p.parent()).map(|p| p.join("Resources")) {
            for candidate in [resources_dir.join(name), resources_dir.join("vendor").join(name)] {
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    // 2. Dev fallback: ./vendor/<name> relative to current dir
    let dev = Path::new("vendor").join(name);
    if dev.exists() {
        return Some(dev.canonicalize().unwrap_or(dev));
    }

    None
}
