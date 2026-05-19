APP_NAME := MD Converter
APP_DIR ?= /Applications
APP_PATH := $(APP_DIR)/$(APP_NAME).app

BUNDLED_APP := target/release/bundle/osx/$(APP_NAME).app

PANDOC_BIN := vendor/pandoc
TYPST_BIN  := vendor/typst

.PHONY: help install bundle build pandoc typst binaries clean run check

help:
	@echo "Targets:"
	@echo "  make install    Build, ad-hoc sign, copy to $(APP_DIR)"
	@echo "  make bundle     Build the .app into ./target/release/bundle/osx/"
	@echo "  make build      Compile the binary only (no .app bundle)"
	@echo "  make binaries   Fetch the bundled pandoc + typst binaries"
	@echo "  make run        Build and launch the .app"
	@echo "  make clean      Remove build artifacts"
	@echo ""
	@echo "Override install location:"
	@echo "  APP_DIR=~/Applications make install"

check:
	@command -v cargo >/dev/null || { echo "cargo not found. Install Rust: https://rustup.rs"; exit 1; }
	@command -v cargo-bundle >/dev/null || { echo "Installing cargo-bundle…"; cargo install cargo-bundle; }
	@xcode-select -p >/dev/null 2>&1 || { echo "Xcode Command Line Tools required: xcode-select --install"; exit 1; }

$(PANDOC_BIN):
	./scripts/fetch-pandoc.sh

$(TYPST_BIN):
	./scripts/fetch-typst.sh

pandoc: $(PANDOC_BIN)
typst: $(TYPST_BIN)
binaries: $(PANDOC_BIN) $(TYPST_BIN)

build: check
	cargo build --release

bundle: check binaries
	cargo bundle --release
	@echo "Ad-hoc signing $(BUNDLED_APP)…"
	codesign --force --deep --sign - "$(BUNDLED_APP)"
	@echo ""
	@echo "Built: $(BUNDLED_APP)"

install: bundle
	@if [ -d "$(APP_PATH)" ]; then \
		echo "Removing existing $(APP_PATH)"; \
		rm -rf "$(APP_PATH)"; \
	fi
	@echo "Installing to $(APP_PATH)"
	@mkdir -p "$(APP_DIR)"
	@cp -R "$(BUNDLED_APP)" "$(APP_PATH)"
	@echo ""
	@echo "Installed $(APP_NAME) to $(APP_PATH)"
	@echo "Launch it from Spotlight or run: open \"$(APP_PATH)\""

run: bundle
	open "$(BUNDLED_APP)"

clean:
	cargo clean
	rm -rf target
