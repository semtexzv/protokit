# Protokit

![crates.io](https://img.shields.io/crates/v/protokit.svg)
![img](https://img.shields.io/docsrs/protokit)
![example workflow](https://github.com/semtexzv/protokit/actions/workflows/rust.yml/badge.svg)

Implementation of protocol buffers for rust.

## Why not prost/pb-j/rust-protobuf?

Started with my need for
proper [textformat](https://protobuf.dev/reference/protobuf/textformat-spec/#any)
support.

```prototext
any_value {
  [type.googleapis.com/com.example.SomeType] {
    field1: "hello"
  }
}
```

After I investigated other crates, I found out that they had other issues (required
boxing in rust-protobuf, messy codegen in prost).

## Features
- Binary(100% conformance) + Text(90% conformance) format support
- No message boxing required
- GRPC (usable with tonic)
- Nice derive macro for creating message implementations without proto compiler:
```rust
#[derive(Default, Debug, Clone, BinProto)]
pub struct FieldPath {
    #[field(1, varint, packed)]
    pub fields: Vec<u32>,
}

#[derive(Default, Debug, Clone, BinProto)]
pub struct FieldMask {
    #[field(1, nested, repeated)]
    pub paths: Vec<FieldPath>,
}
```
