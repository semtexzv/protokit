#!/usr/bin/env bash

mkdir -p vendor

git clone https://github.com/protocolbuffers/protobuf --no-checkout  vendor/protobuf
git clone https://github.com/googleapis/googleapis --no-checkout vendor/googleapis
git clone https://github.com/envoyproxy/protoc-gen-validate --no-checkout vendor/validate

(cd vendor/protobuf && git fetch && git checkout a8d85ffbc7158660b4247f732371d1b07780510e --force && git submodule update --init --recursive)
(cd vendor/googleapis && git fetch && git checkout master --force && git submodule update --init --recursive)
(cd vendor/validate && git fetch && git checkout main --force && git submodule update --init --recursive)

git submodule update --init --recursive