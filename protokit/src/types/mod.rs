#![allow(nonstandard_style)]
#![allow(unused_braces)]

pub mod any;
pub use desc::generated::google::protobuf::*;
use textformat::reflect::Registry;

pub fn register_types(reg: &mut Registry) {
    desc::generated::register_types(reg);
}
