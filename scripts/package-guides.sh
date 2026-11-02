#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
stage="${1:-$root/release}"
guide_dir="$stage/Universal-Fire-TV-Manager-Guides"

rm -rf "$guide_dir"
mkdir -p "$guide_dir"
cp "$root/README.md" "$guide_dir/README.md"
cp "$root/docs/platform/DEVICE-SETUP.md" "$guide_dir/01-DEVICE-SETUP.md"
cp "$root/docs/platform/MACOS.md" "$guide_dir/02-MACOS.md"
cp "$root/docs/platform/WINDOWS.md" "$guide_dir/03-WINDOWS.md"
cp "$root/docs/platform/LINUX.md" "$guide_dir/04-LINUX.md"
cp "$root/docs/platform/ANDROID.md" "$guide_dir/05-ANDROID.md"
cp "$root/docs/platform/CLI.md" "$guide_dir/06-CLI.md"
cp "$root/docs/BACKUP-RESTORE.md" "$guide_dir/07-BACKUP-RESTORE.md"
cp "$root/docs/UNINSTALL.md" "$guide_dir/08-UNINSTALL.md"
cp "$root/docs/TROUBLESHOOTING.md" "$guide_dir/09-TROUBLESHOOTING.md"
cp "$root/docs/PROFILE-VALIDATION.md" "$guide_dir/10-PROFILE-VALIDATION.md"

(cd "$stage" && zip -qr Universal-Fire-TV-Manager-Guides.zip Universal-Fire-TV-Manager-Guides)
echo "guides staged at $guide_dir"
