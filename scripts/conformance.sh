#!/usr/bin/env bash

set -exuo pipefail

git submodule update --init --recursive
cmake -S . -B target/protobuf -Dprotobuf_BUILD_CONFORMANCE=ON -Dprotobuf_BUILD_JAVA=OFF -Dprotobuf_BUILD_KOTLIN=OFF -Dprotobuf_BUILD_OBJC=OFF -Dprotobuf_BUILD_CSHARP=OFF

cd target/protobuf && make VERBOSE=1 rust_conformance_runner; cd ../..

./target/protobuf/rust_conformance_runner --failure_list tools/conformance/failure_list_rust.txt --text_format_failure_list tools/conformance/text_format_failure_list_rust.txt

#cargo build --package conformance
#./target/protobuf/bin/conformance_test_runner ./target/debug/conformance