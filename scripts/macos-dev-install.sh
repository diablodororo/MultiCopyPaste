#!/usr/bin/env bash
# Build, sign with a stable local development certificate, and install to
# /Applications as the single canonical copy.
#
# Why: macOS ties the Accessibility (輔助使用) grant to the app's code
# signature. Ad-hoc signatures change on every rebuild, silently revoking the
# grant and breaking paste injection. Signing with the same local certificate
# keeps the grant valid across rebuilds — authorize once, never again.
set -euo pipefail
cd "$(dirname "$0")/.."

IDENTITY="${APPLE_SIGNING_IDENTITY:-sideBot Local Development 2026}"
APP=src-tauri/target/release/bundle/macos/MultiCopyPaste.app

npx tauri build --bundles app
codesign --force --deep -s "$IDENTITY" "$APP"
codesign --verify --strict "$APP"

pkill -f "MultiCopyPaste.app/Contents/MacOS" || true
sleep 1
rm -rf /Applications/MultiCopyPaste.app
cp -R "$APP" /Applications/MultiCopyPaste.app
open /Applications/MultiCopyPaste.app
echo "Installed and launched /Applications/MultiCopyPaste.app (signed: ${IDENTITY})"
