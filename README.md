# MD Converter

A small macOS app that converts Markdown into DOCX or PDF. Built for Claude Code users handling generated MD files who'd rather not touch `pandoc` on the command line.

- **Stack:** Rust + [iced](https://iced.rs/), macOS 14+ (Apple Silicon)
- **Engine:** bundled [pandoc](https://pandoc.org) for DOCX; [`pandoc --pdf-engine=typst`](https://typst.app) for PDF (vector output, no LaTeX, ~40KB for a typical doc)
- **License:** TBD

## Install

> Public v1 is build-from-source, like [wezmux](https://github.com/vcabeli/wezmux). A signed DMG release is on the roadmap once distribution graduates to non-technical users.

### Prerequisites

- **Rust toolchain** (no full Xcode needed):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Xcode Command Line Tools** (for the system linker — much smaller than full Xcode):
  ```sh
  xcode-select --install
  ```
- **cargo-bundle** for `.app` packaging (auto-installed by `make` if missing):
  ```sh
  cargo install cargo-bundle
  ```

### Clone and install

```sh
git clone https://github.com/ls-dotomics/md-converter.git
cd md-converter
make install
```

`make install` fetches the bundled pandoc + typst binaries, compiles the Rust binary in release mode, runs `cargo bundle` to assemble `MD Converter.app`, ad-hoc codesigns it, and copies the result to `/Applications`.

Custom location:
```sh
APP_DIR=~/Applications make install
```

Other targets:
```sh
make bundle     # build the .app into target/release/bundle/osx/ without installing
make build      # compile the binary only (no .app bundle)
make binaries   # fetch the bundled pandoc + typst binaries
make run        # build and launch the .app
make clean      # remove build artifacts
```

## Usage

1. Launch **MD Converter** from Spotlight (or your Applications folder).
2. Drag a `.md` file onto the drop zone — or click **Choose File…**
3. Pick **DOCX** or **PDF**.
4. Click **Convert** and choose where to save.

## Project layout

```
src/
├── main.rs               iced UI (drop zone, format picker, status)
└── convert.rs            pandoc + typst process spawning
vendor/
├── pandoc                bundled binary (fetched, not committed)
└── typst                 bundled binary (fetched, not committed)
scripts/
├── fetch-pandoc.sh       downloads pinned pandoc release
└── fetch-typst.sh        downloads pinned typst release
Cargo.toml                Rust deps + cargo-bundle metadata
Makefile                  developer entry point
```

## Roadmap

- **v1 (current):** MD → DOCX, MD → PDF, drag-drop UX. Build-from-source distribution.
- **v2:** Marp / presentation export.
- **Distribution:** Signed + notarized DMG via GitHub Actions once Apple Developer enrolled, enabling one-click install for non-technical users.

## Acknowledgements

- [pandoc](https://pandoc.org) and [typst](https://typst.app) — the document conversion engines doing the actual work.
- [iced](https://iced.rs/) — Rust GUI framework.
- [wezmux](https://github.com/vcabeli/wezmux) — install/distribution pattern inspiration.
