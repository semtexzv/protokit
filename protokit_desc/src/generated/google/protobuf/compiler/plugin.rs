#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate as protokit;
use protokit::*;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&Version::default());
    registry.register(&CodeGeneratorRequest::default());
    registry.register(&CodeGeneratorResponseFile::default());
    registry.register(&CodeGeneratorResponse::default());
}
use super::super::descriptor::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeGeneratorResponseFeature(pub i32);
#[protoenum]
impl CodeGeneratorResponseFeature {
    #[var(0i32, "FEATURE_NONE")]
    pub const FEATURE_NONE: CodeGeneratorResponseFeature = CodeGeneratorResponseFeature(
        0i32,
    );
    #[var(1i32, "FEATURE_PROTO3_OPTIONAL")]
    pub const FEATURE_PROTO3_OPTIONAL: CodeGeneratorResponseFeature = CodeGeneratorResponseFeature(
        1i32,
    );
    #[var(2i32, "FEATURE_SUPPORTS_EDITIONS")]
    pub const FEATURE_SUPPORTS_EDITIONS: CodeGeneratorResponseFeature = CodeGeneratorResponseFeature(
        2i32,
    );
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Version", package = "google.protobuf.compiler")]
pub struct Version {
    #[field(1u32, "major", varint, optional)]
    pub major: Option<i32>,
    #[field(2u32, "minor", varint, optional)]
    pub minor: Option<i32>,
    #[field(3u32, "patch", varint, optional)]
    pub patch: Option<i32>,
    #[field(4u32, "suffix", string, optional)]
    pub suffix: Option<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "CodeGeneratorRequest", package = "google.protobuf.compiler")]
pub struct CodeGeneratorRequest {
    #[field(1u32, "file_to_generate", string, repeated)]
    pub file_to_generate: Vec<String>,
    #[field(2u32, "parameter", string, optional)]
    pub parameter: Option<String>,
    #[field(15u32, "proto_file", nested, repeated)]
    pub proto_file: Vec<FileDescriptorProto>,
    #[field(17u32, "source_file_descriptors", nested, repeated)]
    pub source_file_descriptors: Vec<FileDescriptorProto>,
    #[field(3u32, "compiler_version", nested, optional)]
    pub compiler_version: Option<Version>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "CodeGeneratorResponse.File", package = "google.protobuf.compiler")]
pub struct CodeGeneratorResponseFile {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "insertion_point", string, optional)]
    pub insertion_point: Option<String>,
    #[field(15u32, "content", string, optional)]
    pub content: Option<String>,
    #[field(16u32, "generated_code_info", nested, optional)]
    pub generated_code_info: Option<Box<GeneratedCodeInfo>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "CodeGeneratorResponse", package = "google.protobuf.compiler")]
pub struct CodeGeneratorResponse {
    #[field(1u32, "error", string, optional)]
    pub error: Option<String>,
    #[field(2u32, "supported_features", varint, optional)]
    pub supported_features: Option<u64>,
    #[field(3u32, "minimum_edition", varint, optional)]
    pub minimum_edition: Option<i32>,
    #[field(4u32, "maximum_edition", varint, optional)]
    pub maximum_edition: Option<i32>,
    #[field(15u32, "file", nested, repeated)]
    pub file: Vec<CodeGeneratorResponseFile>,
}
