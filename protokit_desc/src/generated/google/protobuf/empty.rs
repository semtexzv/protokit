#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate::*;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    registry.register(&Empty::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Empty {}
