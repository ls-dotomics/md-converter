#!/usr/bin/env bash
# Download the pinned pandoc arm64 macOS binary into vendor/pandoc.
# Run once after cloning (then commit via Git LFS).

set -euo pipefail

PANDOC_VERSION="${PANDOC_VERSION:-3.9.0.2}"
ARCH="arm64"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VENDOR_DIR="$ROOT/vendor"
DEST="$VENDOR_DIR/pandoc"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

ARCHIVE="pandoc-${PANDOC_VERSION}-${ARCH}-macOS.zip"
URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/${ARCHIVE}"

echo "Fetching $URL"
mkdir -p "$VENDOR_DIR"
curl -fL --progress-bar -o "$TMP/$ARCHIVE" "$URL"

echo "Extracting…"
unzip -q "$TMP/$ARCHIVE" -d "$TMP"

# Archive layout: pandoc-<ver>-<arch>/bin/pandoc
EXTRACTED="$(find "$TMP" -type f -name pandoc -perm -u+x | head -n1)"
if [[ -z "$EXTRACTED" ]]; then
  echo "Could not find pandoc binary in archive." >&2
  exit 1
fi

cp "$EXTRACTED" "$DEST"
chmod +x "$DEST"

echo "Installed $($DEST --version | head -n1) at $DEST"
echo "Now: git add .gitattributes vendor/pandoc && git commit"
