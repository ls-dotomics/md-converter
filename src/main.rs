#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod convert;
mod theme;

use std::path::PathBuf;

use iced::widget::{button, column, container, pick_list, row, svg, text, Space};
use iced::{event, window, Alignment, Element, Event, Font, Length, Subscription, Task};

use convert::{convert_file, OutputFormat};

const ALT_RIVIERA_BYTES: &[u8] = include_bytes!("../assets/fonts/ALTRiviera-Regular.otf");
const LOGO_SVG: &[u8] = include_bytes!("../assets/logo-living-models.svg");
const ALT_RIVIERA: Font = Font::with_name("ALT Riviera");

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(|_state: &App| "MD Converter".to_string())
        .subscription(App::subscription)
        .theme(|_state: &App| theme::living_models_theme())
        .font(ALT_RIVIERA_BYTES)
        .default_font(ALT_RIVIERA)
        .window_size((560.0, 460.0))
        .run()
}

#[derive(Debug, Default)]
struct App {
    input: Option<PathBuf>,
    format: OutputFormat,
    status: Status,
    is_hovered: bool,
}

#[derive(Debug, Default, Clone)]
enum Status {
    #[default]
    Idle,
    Converting,
    Done(PathBuf),
    Failed(String),
}

#[derive(Debug, Clone)]
enum Message {
    PickFile,
    FilePicked(Option<PathBuf>),
    FileDropped(PathBuf),
    FileHovered(bool),
    FormatChanged(OutputFormat),
    Convert,
    OutputPicked(Option<PathBuf>),
    Converted(Result<PathBuf, String>),
    RevealInFinder(PathBuf),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PickFile => Task::perform(pick_input(), Message::FilePicked),
            Message::FilePicked(path) => {
                if let Some(path) = path {
                    self.input = Some(path);
                    self.status = Status::Idle;
                }
                Task::none()
            }
            Message::FileDropped(path) => {
                if looks_like_markdown(&path) {
                    self.input = Some(path);
                    self.status = Status::Idle;
                }
                self.is_hovered = false;
                Task::none()
            }
            Message::FileHovered(hovered) => {
                self.is_hovered = hovered;
                Task::none()
            }
            Message::FormatChanged(format) => {
                self.format = format;
                Task::none()
            }
            Message::Convert => {
                let Some(input) = self.input.clone() else {
                    return Task::none();
                };
                let format = self.format;
                Task::perform(pick_output(input, format), Message::OutputPicked)
            }
            Message::OutputPicked(Some(output)) => {
                let Some(input) = self.input.clone() else {
                    return Task::none();
                };
                let format = self.format;
                self.status = Status::Converting;
                Task::perform(
                    async move { convert_file(input, output, format).await.map_err(|e| e.to_string()) },
                    Message::Converted,
                )
            }
            Message::OutputPicked(None) => Task::none(),
            Message::Converted(Ok(path)) => {
                self.status = Status::Done(path);
                Task::none()
            }
            Message::Converted(Err(message)) => {
                self.status = Status::Failed(message);
                Task::none()
            }
            Message::RevealInFinder(path) => {
                let _ = std::process::Command::new("open")
                    .arg("-R")
                    .arg(&path)
                    .spawn();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = self.header();
        let drop_zone = self.drop_zone();
        let controls = self.controls();
        let status = self.status_view();

        container(
            column![header, drop_zone, controls, status]
                .spacing(20)
                .align_x(Alignment::Center),
        )
        .padding(28)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::canvas)
        .into()
    }

    fn header(&self) -> Element<'_, Message> {
        let handle = svg::Handle::from_memory(LOGO_SVG);
        container(svg(handle).width(Length::Fixed(300.0)).height(Length::Fixed(52.0)))
            .center_x(Length::Fill)
            .into()
    }

    fn drop_zone(&self) -> Element<'_, Message> {
        let label: Element<Message> = if let Some(input) = &self.input {
            column![
                text(file_name(input)).size(18),
                text(parent_path(input)).size(11).color(theme::GREY),
            ]
            .spacing(4)
            .align_x(Alignment::Center)
            .into()
        } else {
            column![
                text("drop a markdown file here").size(16),
                text("or click choose file…").size(12).color(theme::GREY),
            ]
            .spacing(4)
            .align_x(Alignment::Center)
            .into()
        };

        container(label)
            .width(Length::Fill)
            .height(170.0)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(if self.is_hovered {
                theme::drop_zone_hovered
            } else {
                theme::drop_zone_idle
            })
            .into()
    }

    fn controls(&self) -> Element<'_, Message> {
        let format_picker = pick_list(
            OutputFormat::ALL,
            Some(self.format),
            Message::FormatChanged,
        )
        .text_size(14);

        let convert_disabled = self.input.is_none() || matches!(self.status, Status::Converting);
        let convert_btn = {
            let b = button(text("convert").size(14))
                .padding([8, 22])
                .style(theme::primary_button);
            if convert_disabled {
                b
            } else {
                b.on_press(Message::Convert)
            }
        };

        row![
            button(text("choose file…").size(14))
                .padding([8, 16])
                .style(theme::secondary_button)
                .on_press(Message::PickFile),
            format_picker,
            Space::new().width(Length::Fill),
            convert_btn,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    }

    fn status_view(&self) -> Element<'_, Message> {
        match &self.status {
            Status::Idle => Space::new().height(Length::Fixed(20.0)).into(),
            Status::Converting => row![text("converting…").size(12).color(theme::GREY)].into(),
            Status::Done(path) => row![
                text(format!("✓ saved to {}", file_name(path))).size(12),
                Space::new().width(Length::Fixed(8.0)),
                button(text("show in finder").size(11))
                    .padding([4, 10])
                    .style(theme::secondary_button)
                    .on_press(Message::RevealInFinder(path.clone())),
            ]
            .align_y(Alignment::Center)
            .spacing(6)
            .into(),
            Status::Failed(message) => text(format!("⚠ {message}")).size(12).into(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Window(window::Event::FileDropped(path)) => Some(Message::FileDropped(path)),
            Event::Window(window::Event::FileHovered(_)) => Some(Message::FileHovered(true)),
            Event::Window(window::Event::FilesHoveredLeft) => Some(Message::FileHovered(false)),
            _ => None,
        })
    }
}

async fn pick_input() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd", "txt"])
        .pick_file()
        .await
        .map(|h| h.path().to_path_buf())
}

async fn pick_output(input: PathBuf, format: OutputFormat) -> Option<PathBuf> {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();
    let parent = input
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    rfd::AsyncFileDialog::new()
        .set_file_name(format!("{stem}.{}", format.extension()))
        .set_directory(parent)
        .add_filter(format.label(), &[format.extension()])
        .save_file()
        .await
        .map(|h| h.path().to_path_buf())
}

fn looks_like_markdown(path: &std::path::Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()).map(str::to_lowercase).as_deref(),
        Some("md" | "markdown" | "mdown" | "mkd" | "txt")
    )
}

fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

fn parent_path(path: &std::path::Path) -> String {
    path.parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string()
}
