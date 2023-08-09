#!/usr/bin/env bash

(cd protokit_derive && cargo publish)
(cd protokit_binformat && cargo publish)
(cd protokit_textformat && cargo publish)
(cd protokit_desc && cargo publish)
(cd protokit_proto && cargo publish)
(cd protokit_build && cargo publish)
(cd protokit && cargo publish)