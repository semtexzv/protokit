#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::source_context::*;
use super::r#type::*;
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Api {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "methods", nested, repeated)]
    pub methods: Vec<Method>,
    #[field(3u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[field(4u32, "version", string, singular)]
    pub version: String,
    #[field(5u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(6u32, "mixins", nested, repeated)]
    pub mixins: Vec<Mixin>,
    #[field(7u32, "syntax", protoenum, singular)]
    pub syntax: Syntax,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Method {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "request_type_url", string, singular)]
    pub request_type_url: String,
    #[field(3u32, "request_streaming", bool, singular)]
    pub request_streaming: bool,
    #[field(4u32, "response_type_url", string, singular)]
    pub response_type_url: String,
    #[field(5u32, "response_streaming", bool, singular)]
    pub response_streaming: bool,
    #[field(6u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[field(7u32, "syntax", protoenum, singular)]
    pub syntax: Syntax,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Mixin {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "root", string, singular)]
    pub root: String,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
