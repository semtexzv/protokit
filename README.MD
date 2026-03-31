# Protokit

[![crates.io](https://img.shields.io/crates/v/protokit.svg)](https://crates.io/crates/protokit)
[![img](https://img.shields.io/docsrs/protokit)](https://docs.rs/protokit/latest)
[![example workflow](https://github.com/semtexzv/protokit/actions/workflows/rust.yml/badge.svg)](https://github.com/semtexzv/protokit/actions/workflows/rust.yml)

Protocol Buffers for Rust — binary and text format, with a derive-first API.

The only Rust implementation with full TextFormat support. 100% binary conformance, ~90% text conformance. No message boxing required.

## Define messages in Rust

```rust
#[derive(Default, Debug, Clone, BinProto)]
pub struct Person {
    #[field(1, string)]
    pub name: String,
    #[field(2, varint)]
    pub age: u32,
    #[field(3, nested, repeated)]
    pub addresses: Vec<Address>,
}

#[derive(Default, Debug, Clone, BinProto)]
pub struct Address {
    #[field(1, string)]
    pub street: String,
    #[field(2, string)]
    pub city: String,
}
```

## Encode and decode

```rust
// Binary format
let bytes = protokit::binformat::encode(&person)?;
let decoded: Person = protokit::binformat::decode(&bytes)?;

// Text format
let text = protokit::textformat::encode(&person, &Registry::default())?;
```

## Compile from .proto files

```rust
// build.rs
fn main() {
    protokit_build::Build::new()
        .include("proto/")
        .compile("proto/person.proto")
        .unwrap();
}
```

```rust
// main.rs — use the generated types
pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}
use gen::mypackage::Person;
```

## Enums

```rust
#[derive(Default, Debug, Clone, Copy)]
pub struct Status(pub i32);

#[protoenum]
impl Status {
    #[var(0, "UNKNOWN")]
    pub const UNKNOWN: i32 = 0;
    #[var(1, "ACTIVE")]
    pub const ACTIVE: i32 = 1;
    #[var(2, "INACTIVE")]
    pub const INACTIVE: i32 = 2;
}
```

## TextFormat with Any expansion

```prototext
any_value {
  [type.googleapis.com/com.example.SomeType] {
    field1: "hello"
  }
}
```

This is what started the project — proper TextFormat support that handles `Any` types, which no other Rust library does.

## Crates

| Crate | Purpose |
|-------|---------|
| `protokit` | Core — derive macros, re-exports |
| `protokit_binformat` | Wire format serialization |
| `protokit_textformat` | TextFormat serialization |
| `protokit_build` | `.proto` compiler (hand-written parser, no protoc needed) |
| `protokit_grpc` | gRPC codegen (tonic-compatible) |
| `protokit_desc` | Descriptor handling |

## Why not prost?

- No TextFormat support
- Generated code is harder to read
- Required boxing for recursive types

Protokit stores nested messages inline and gives you both binary and text format from the same derive macro.

## License

MIT
