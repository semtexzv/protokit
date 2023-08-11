#!/usr/bin/env bash

set -exuo pipefail

git submodule update --init --recursive
cmake -S . -B target/protobuf -Dprotobuf_BUILD_CONFORMANCE=ON

cd target/protobuf && make rust_conformance_runner; cd ../..

./target/protobuf/bin/rust_conformance_runner

#cargo build --package conformance
#./target/protobuf/bin/conformance_test_runner ./target/debug/conformance