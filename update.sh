#!/usr/bin/env bash

mkdir -p vendor

git clone https://github.com/protocolbuffers/protobuf --no-checkout --depth 1 vendor/protobuf
git clone https://github.com/googleapis/googleapis --no-checkout --depth 1 vendor/googleapis
git clone https://github.com/envoyproxy/protoc-gen-validate --no-checkout --depth 1 vendor/validate

(cd vendor/protobuf && git fetch && git checkout main --force)
(cd vendor/googleapis && git fetch && git checkout master --force)
(cd vendor/validate && git fetch && git checkout main --force)

