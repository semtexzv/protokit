#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    registry.register(&FieldMask::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FieldMask {
    #[field(1u32, "paths", string, repeated)]
    pub paths: Vec<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
