#![feature(exit_status_error)]

use std::env::var;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    protokit_build::Build::new()
        .track_unknowns(true)
        .include("../../vendor/protobuf/conformance")
        .include("../../vendor/protobuf/src/")
        .include("../../vendor/protobuf/src/google/protobuf")
        .textformat(true)
        .compile("conformance.proto")
        .unwrap()
        .compile("test_messages_proto3.proto")
        .unwrap()
        .compile("test_messages_proto2.proto")
        .unwrap()
        .generate()
        .unwrap();
}
