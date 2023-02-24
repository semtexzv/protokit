fn main() {
    cmake::Config::new("../../vendor/protobuf")
        .define("protobuf_BUILD_CONFORMANCE", "ON")
        .very_verbose(true)
        .target("conformance_cpp")
        .no_build_target(true)
        .build();

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
