#!/usr/bin/env bash
# Download the pinned typst arm64 macOS binary into vendor/typst.

set -euo pipefail

TYPST_VERSION="${TYPST_VERSION:-v0.14.2}"
ARCH="aarch64-apple-darwin"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VENDOR_DIR="$ROOT/vendor"
DEST="$VENDOR_DIR/typst"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

ARCHIVE="typst-${ARCH}.tar.xz"
URL="https://github.com/typst/typst/releases/download/${TYPST_VERSION}/${ARCHIVE}"

echo "Fetching $URL"
mkdir -p "$VENDOR_DIR"
curl -fL --progress-bar -o "$TMP/$ARCHIVE" "$URL"

echo "Extracting…"
tar -xf "$TMP/$ARCHIVE" -C "$TMP"

EXTRACTED="$(find "$TMP" -type f -name typst -perm -u+x | head -n1)"
if [[ -z "$EXTRACTED" ]]; then
  echo "Could not find typst binary in archive." >&2
  exit 1
fi

cp "$EXTRACTED" "$DEST"
chmod +x "$DEST"

echo "Installed $($DEST --version) at $DEST"
