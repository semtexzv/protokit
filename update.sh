#!/usr/bin/env bash

git clone https://github.com/protocolbuf#fers/protobuf --no-checkout --depth 1 vendor/protobuf
git clone https://github.com/googleapis/googleapis --no-checkout --depth 1 vendor/googleapis
git clone https://github.com/envoyproxy/protoc-gen-validate --no-checkout --depth 1 vendor/validate

(cd vendor/protobuf && git checkout main -- '*.proto')
(cd vendor/googleapis && git checkout master -- '*.proto')
(cd vendor/validate && git checkout main -- '*.proto')
