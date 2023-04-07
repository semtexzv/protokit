#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::super::descriptor::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeGeneratorResponse__Feature(pub u32);
#[protoenum]
impl CodeGeneratorResponse__Feature {
    #[var(0u32, "FEATURE_NONE")]
    pub const FEATURE_NONE: CodeGeneratorResponse__Feature = CodeGeneratorResponse__Feature(
        0u32,
    );
    #[var(1u32, "FEATURE_PROTO3_OPTIONAL")]
    pub const FEATURE_PROTO3_OPTIONAL: CodeGeneratorResponse__Feature = CodeGeneratorResponse__Feature(
        1u32,
    );
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Version {
    #[field(1u32, "major", varint, optional)]
    pub major: Option<i32>,
    #[field(2u32, "minor", varint, optional)]
    pub minor: Option<i32>,
    #[field(3u32, "patch", varint, optional)]
    pub patch: Option<i32>,
    #[field(4u32, "suffix", string, optional)]
    pub suffix: Option<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct CodeGeneratorRequest {
    #[field(1u32, "file_to_generate", string, repeated)]
    pub file_to_generate: Vec<String>,
    #[field(2u32, "parameter", string, optional)]
    pub parameter: Option<String>,
    #[field(15u32, "proto_file", nested, repeated)]
    pub proto_file: Vec<FileDescriptorProto>,
    #[field(3u32, "compiler_version", nested, optional)]
    pub compiler_version: Option<Box<Version>>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct CodeGeneratorResponse {
    #[field(1u32, "error", string, optional)]
    pub error: Option<String>,
    #[field(2u32, "supported_features", varint, optional)]
    pub supported_features: Option<u64>,
    #[field(15u32, "file", nested, repeated)]
    pub file: Vec<CodeGeneratorResponse__File>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct CodeGeneratorResponse__File {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "insertion_point", string, optional)]
    pub insertion_point: Option<String>,
    #[field(15u32, "content", string, optional)]
    pub content: Option<String>,
    #[field(16u32, "generated_code_info", nested, optional)]
    pub generated_code_info: Option<Box<GeneratedCodeInfo>>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}