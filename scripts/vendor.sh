#!/usr/bin/env bash

mkdir -p vendor

git clone https://github.com/protocolbuffers/protobuf --no-checkout  vendor/protobuf
git clone https://github.com/googleapis/googleapis --no-checkout vendor/googleapis
git clone https://github.com/envoyproxy/protoc-gen-validate --no-checkout vendor/validate

(cd vendor/protobuf && git fetch && git checkout a847a8dc4ba1d99e7ba917146c84438b4de7d085 --force && git submodule update --init --recursive)
(cd vendor/googleapis && git fetch && git checkout master --force && git submodule update --init --recursive)
(cd vendor/validate && git fetch && git checkout main --force && git submodule update --init --recursive)

git submodule update --init --recursive