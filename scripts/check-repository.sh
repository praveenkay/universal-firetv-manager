#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

required=(
  Cargo.toml
  rust-toolchain.toml
  LICENSE-APACHE
  LICENSE-MIT
  NOTICE.md
  SECURITY.md
  README.md
  core/Cargo.toml
  core/src/lib.rs
)

for path in "${required[@]}"; do
  [[ -f "$path" ]] || {
    echo "repository policy: missing required file: $path" >&2
    exit 1
  }
done

tracked=$(git ls-files --cached --others --exclude-standard)

if grep -Eiq '(^|/)(backups?|artifacts?|installers?)/|\.apk$|(^|/)adbkey(\.pub)?$' <<<"$tracked"; then
  echo "repository policy: private backup, installer, APK, artifact, or ADB key path detected" >&2
  exit 1
fi

content_files=$(grep -vxF 'scripts/check-repository.sh' <<<"$tracked" || true)
if [[ -n "$content_files" ]] && xargs grep -Il '' <<<"$content_files" | xargs grep -En \
  '192\.168\.4\.44|(^|[^[:alnum:]])AFTR([^[:alnum:]]|$)|0035334210304' >/dev/null 2>&1; then
  echo "repository policy: unredacted device identifier detected" >&2
  exit 1
fi

echo "repository policy: OK"
