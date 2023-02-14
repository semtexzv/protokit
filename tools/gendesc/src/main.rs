fn main() -> protokit_build::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    let mut config = protokit_build::Build::without_replacements()
        .out_dir("./protokit_desc/src/generated")
        .include("proto")
        .include("../proto")
        .include("vendor/validate")
        .include("../vendor/validate")
        .textformat(true)
        .root("crate");

    config
        .compile("google/protobuf/descriptor.proto")?
        .compile("google/protobuf/empty.proto")?
        .compile("google/protobuf/field_mask.proto")?
        .compile("google/protobuf/timestamp.proto")?
        .compile("validate/validate.proto")?
        .generate()
}
