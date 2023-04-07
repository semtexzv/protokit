#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Timestamp {
    #[field(1u32, "seconds", varint, singular)]
    pub seconds: i64,
    #[field(2u32, "nanos", varint, singular)]
    pub nanos: i32,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
