#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    registry.register(&Any::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Any", package = "google.protobuf")]
pub struct Any {
    #[field(1u32, "type_url", string, singular)]
    pub type_url: String,
    #[field(2u32, "value", bytes, singular)]
    pub value: Vec<u8>,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
