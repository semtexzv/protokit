#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::source_context::*;
use super::r#type::*;
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Api {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "methods", nested, packed)]
    pub methods: Vec<Method>,
    #[field(3u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[field(4u32, "version", string, optional)]
    pub version: Option<String>,
    #[field(5u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(6u32, "mixins", nested, packed)]
    pub mixins: Vec<Mixin>,
    #[field(7u32, "syntax", protoenum, optional)]
    pub syntax: Option<Syntax>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Method {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "request_type_url", string, optional)]
    pub request_type_url: Option<String>,
    #[field(3u32, "request_streaming", bool, optional)]
    pub request_streaming: Option<bool>,
    #[field(4u32, "response_type_url", string, optional)]
    pub response_type_url: Option<String>,
    #[field(5u32, "response_streaming", bool, optional)]
    pub response_streaming: Option<bool>,
    #[field(6u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[field(7u32, "syntax", protoenum, optional)]
    pub syntax: Option<Syntax>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Mixin {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "root", string, optional)]
    pub root: Option<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
