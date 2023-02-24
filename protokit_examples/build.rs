fn main() {
    protokit_build::Build::new()
        .include("../proto")
        .textformat(true)
        .compile("google/protobuf/descriptor.proto")
        .unwrap()
        .compile("google/protobuf/api.proto")
        .unwrap()
        .compile("google/protobuf/compiler/plugin.proto")
        .unwrap()
        .compile("google/protobuf/struct.proto")
        .unwrap()
        .compile("com/book/book.proto")
        .unwrap()
        .compile("google/protobuf/timestamp.proto")
        .unwrap()
        .generate()
        .unwrap()


}
