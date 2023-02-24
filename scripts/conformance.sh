#!/usr/bin/env bash

set -exuo pipefail

git submodule update --init --recursive
cmake -S vendor/protobuf/ -B target/protobuf -Dprotobuf_BUILD_CONFORMANCE=ON

#cd target/protobuf && make conformance_test_runner; cd ../..

#cargo build --package conformance
./target/protobuf/bin/conformance_test_runner ./target/debug/conformance