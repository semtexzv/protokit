#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate::*;
pub fn register_types(_registry: &mut crate::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Duration {
    #[field(1u32, "seconds", varint, singular)]
    pub seconds: i64,
    #[field(2u32, "nanos", varint, singular)]
    pub nanos: i32,
}
