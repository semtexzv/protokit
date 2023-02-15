fn main() {
    protokit_build::Build::new()
        .track_unknowns(true)
        .include("../../vendor/protobuf/conformance")
        .include("../../vendor/protobuf/src/")
        .include("../../vendor/protobuf/src/google/protobuf")
        .out_dir("src/gen")
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
