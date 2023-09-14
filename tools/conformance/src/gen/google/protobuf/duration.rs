#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use protokit::*;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&Duration::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Duration", package = "google.protobuf")]
pub struct Duration {
    #[field(1u32, "seconds", varint, singular)]
    pub seconds: i64,
    #[field(2u32, "nanos", varint, singular)]
    pub nanos: i32,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
