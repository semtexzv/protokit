#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct SourceContext {
    #[field(1u32, "file_name", string, singular)]
    pub file_name: String,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
