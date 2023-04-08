#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate::*;
pub fn register_types(_registry: &mut crate::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Empty {}
