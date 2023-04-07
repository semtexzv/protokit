#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Any {
    #[field(1u32, "type_url", string, optional)]
    pub type_url: Option<String>,
    #[field(2u32, "value", bytes, optional)]
    pub value: Option<Vec<u8>>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
