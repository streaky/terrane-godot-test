#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_dir=$(cd -- "$project_dir/../.." && pwd)
if [[ -n ${TERRANE_BIN:-} ]]; then
    terrane=$TERRANE_BIN
elif [[ -x "$repo_dir/target/debug/terrane" ]]; then
    terrane=$repo_dir/target/debug/terrane
else
    terrane=$repo_dir/target/release/terrane
fi
if [[ ! -x "$terrane" ]]; then
    printf 'Terrane executable not found; build it or set TERRANE_BIN\n' >&2
    exit 1
fi
artifact=$($terrane build "$project_dir")
install -Dm755 "$artifact" "$project_dir/godot/bin/libterrane_nbody.so"
printf 'Installed %s\n' "$project_dir/godot/bin/libterrane_nbody.so"
