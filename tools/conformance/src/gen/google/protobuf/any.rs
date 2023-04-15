#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Any {
    #[field(1u32, "type_url", string, singular)]
    pub type_url: String,
    #[field(2u32, "value", bytes, singular)]
    pub value: Vec<u8>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
