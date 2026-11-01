#!/usr/bin/env bash
set -euo pipefail

directory="${1:-release}"
cd "$directory"
find . -maxdepth 1 -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 shasum -a 256 > SHA256SUMS
