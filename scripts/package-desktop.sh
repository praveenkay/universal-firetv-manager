#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
npm --prefix "$root/apps/desktop" ci
npm --prefix "$root/apps/desktop" test
npm --prefix "$root/apps/desktop" run tauri build
