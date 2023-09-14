#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use protokit::*;

use crate as protokit;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&Empty::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Empty", package = "google.protobuf")]
pub struct Empty {}
