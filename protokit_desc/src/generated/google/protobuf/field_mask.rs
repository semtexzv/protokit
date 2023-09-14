#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate::*;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    registry.register(&FieldMask::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FieldMask", package = "google.protobuf")]
pub struct FieldMask {
    #[field(1u32, "paths", string, repeated)]
    pub paths: Vec<String>,
}
