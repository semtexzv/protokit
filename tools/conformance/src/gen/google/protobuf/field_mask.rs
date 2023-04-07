#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FieldMask {
    #[field(1u32, "paths", string, repeated)]
    pub paths: Vec<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
