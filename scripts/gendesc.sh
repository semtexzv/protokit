#!/usr/bin/env bash

set -exuo pipefail

export OUT_DIR=$(mktemp -d)
echo "OUT_DIR is $OUT_DIR"
cargo run --manifest-path=./tools/gendesc/Cargo.toml
rm -rf $OUT_DIR