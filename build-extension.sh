#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_dir=$(cd -- "$project_dir/../.." && pwd)
terrane=${TERRANE_BIN:-$repo_dir/target/debug/terrane}
if [[ ! -x "$terrane" ]]; then
    terrane=$repo_dir/target/release/terrane
fi
artifact=$($terrane build "$project_dir")
install -Dm755 "$artifact" "$project_dir/godot/bin/libterrane_nbody.so"
printf 'Installed %s\n' "$project_dir/godot/bin/libterrane_nbody.so"
