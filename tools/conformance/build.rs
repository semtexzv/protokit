fn main() {
    protokit_build::Build::new()
        .track_unknowns(true)

        .out_dir("src/gen")
        .include("../../vendor/protobuf/conformance")
        .include("../../vendor/protobuf/src/")
        .include("../../vendor/protobuf/src/google/protobuf")
        .compile("conformance.proto")
        .unwrap()
        .compile("test_messages_proto3.proto")
        .unwrap()
        .compile("test_messages_proto2.proto")
        .unwrap()
        .generate()
        .unwrap();
}
