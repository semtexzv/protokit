#!/usr/bin/env bash

git submodule update --init --recursive
#cmake . -Dprotobuf_BUILD_CONFORMANCE=ON && cmake --build .
cargo build --package conformance
./vendor/protobuf/conformance_test_runner ./target/debug/conformance