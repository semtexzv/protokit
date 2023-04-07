fn main() -> protokit_build::Result<()> {
    protokit_build::Build::new()
        // .borrow()
        .track_unknowns(true)
        .out_dir("src/gen")
        .include("../proto")
        .compile("google/protobuf/descriptor.proto")?
        .compile("google/protobuf/api.proto")?
        .compile("google/protobuf/compiler/plugin.proto")?
        .compile("google/protobuf/struct.proto")?
        .compile("com/book/book.proto")?
        .compile("google/protobuf/timestamp.proto")?
        .generate()
}
