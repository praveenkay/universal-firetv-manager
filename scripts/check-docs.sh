#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
required=("Quick Start" "Enable ADB" "Compatibility" "Install" "Demo" "Features" "Backup" "Restore" "Uninstall" "Updates" "Package Profiles" "Limitations" "Security" "Privacy" "Troubleshooting")
for heading in "${required[@]}"; do
  grep -Fq "## $heading" "$root/README.md" || { echo "missing README section: $heading" >&2; exit 1; }
done
for file in INSTALL UNINSTALL BACKUP-RESTORE PROFILE-VALIDATION TROUBLESHOOTING PRIVACY DEMO; do
  test -s "$root/docs/$file.md" || { echo "missing docs/$file.md" >&2; exit 1; }
done
echo "documentation checks passed"
