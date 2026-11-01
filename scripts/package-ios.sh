#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
if ! xcodebuild -version >/dev/null 2>&1; then
  echo "Full Xcode is required. Install Xcode, open it once, then run: sudo xcode-select -s /Applications/Xcode.app/Contents/Developer" >&2
  exit 1
fi

export PATH="$HOME/.cargo/bin:$PATH"
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
npm --prefix "$root/apps/desktop" ci
npm --prefix "$root/apps/desktop" test

if [[ ! -d "$root/apps/desktop/src-tauri/gen/apple" ]]; then
  npm --prefix "$root/apps/desktop" run tauri ios init
fi
npm --prefix "$root/apps/desktop" run tauri ios build -- --debug --ci
