fn main() -> protokit_build::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    let mut config = protokit_build::Build::without_replacements()

        .out_dir("./protokit_desc/src/generated")
        .include("proto")
        .include("../proto")
        .include("vendor/validate")
        .include("../vendor/validate")
        .root("crate");

    config
        .compile("google/protobuf/descriptor.proto")?
        .compile("google/protobuf/empty.proto")?
        .compile("google/protobuf/field_mask.proto")?
        .compile("google/protobuf/timestamp.proto")?
        .compile("google/protobuf/compiler/plugin.proto")?
        .compile("google/protobuf/any.proto")?
        .generate()
}
