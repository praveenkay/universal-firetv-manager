#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
export JAVA_HOME="${JAVA_HOME:-/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home}"
export ANDROID_HOME="${ANDROID_HOME:-/opt/homebrew/share/android-commandlinetools}"
export NDK_HOME="${NDK_HOME:-$ANDROID_HOME/ndk/27.0.12077973}"

npm --prefix "$root/apps/desktop" ci
npm --prefix "$root/apps/desktop" test
npm --prefix "$root/apps/desktop" run tauri android build -- --debug --apk --target aarch64 armv7 --split-per-abi --ci
