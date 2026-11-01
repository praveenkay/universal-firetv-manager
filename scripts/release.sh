#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
stage="${1:-$root/release}"
rm -rf "$stage"
mkdir -p "$stage"

cargo build --release -p firetvctl
cp "$root/target/release/firetvctl" "$stage/firetvctl-macos-arm64"

dmg="$root/target/release/bundle/dmg/Universal Fire TV Manager_0.1.0_aarch64.dmg"
arm64="$root/apps/desktop/src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk"
armv7="$root/apps/desktop/src-tauri/gen/android/app/build/outputs/apk/arm/debug/app-arm-debug.apk"
[[ -f "$dmg" ]] && cp "$dmg" "$stage/Universal-Fire-TV-Manager-macOS-arm64.dmg"
[[ -f "$arm64" ]] && cp "$arm64" "$stage/Universal-Fire-TV-Manager-Android-arm64.apk"
[[ -f "$armv7" ]] && cp "$armv7" "$stage/Universal-Fire-TV-Manager-Android-armv7.apk"

bash "$root/scripts/check-repository.sh" "$stage"
bash "$root/scripts/checksums.sh" "$stage"
echo "release staged at $stage"
