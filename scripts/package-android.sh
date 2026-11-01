#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
export JAVA_HOME="${JAVA_HOME:-/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home}"
export ANDROID_HOME="${ANDROID_HOME:-/opt/homebrew/share/android-commandlinetools}"
export NDK_HOME="${NDK_HOME:-$ANDROID_HOME/ndk/27.0.12077973}"
keystore="${ANDROID_KEYSTORE:-$HOME/.android/debug.keystore}"
store_pass="${ANDROID_KEYSTORE_PASSWORD:-android}"
key_pass="${ANDROID_KEY_PASSWORD:-$store_pass}"
apksigner="${APKSIGNER:-$ANDROID_HOME/build-tools/36.0.0/apksigner}"
output="$root/artifacts/android"

npm --prefix "$root/apps/desktop" ci
npm --prefix "$root/apps/desktop" test
npm --prefix "$root/apps/desktop" run tauri android build -- --apk --target aarch64 armv7 --split-per-abi --ci

mkdir -p "$output"
"$apksigner" sign --ks "$keystore" --ks-pass "pass:$store_pass" --key-pass "pass:$key_pass" \
  --out "$output/Universal-Fire-TV-Manager-Android-arm64.apk" \
  "$root/apps/desktop/src-tauri/gen/android/app/build/outputs/apk/arm64/release/app-arm64-release-unsigned.apk"
"$apksigner" sign --ks "$keystore" --ks-pass "pass:$store_pass" --key-pass "pass:$key_pass" \
  --out "$output/Universal-Fire-TV-Manager-Android-armv7.apk" \
  "$root/apps/desktop/src-tauri/gen/android/app/build/outputs/apk/arm/release/app-arm-release-unsigned.apk"
"$apksigner" verify --verbose "$output/Universal-Fire-TV-Manager-Android-arm64.apk"
"$apksigner" verify --verbose "$output/Universal-Fire-TV-Manager-Android-armv7.apk"
echo "signed test APKs written to $output"
