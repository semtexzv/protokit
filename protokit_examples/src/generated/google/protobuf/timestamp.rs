#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Timestamp {
    #[field(1u32, "seconds", varint, optional)]
    pub seconds: Option<i64>,
    #[field(2u32, "nanos", varint, optional)]
    pub nanos: Option<i32>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
