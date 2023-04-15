#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate::*;
pub fn register_types(_registry: &mut crate::textformat::reflect::Registry) {}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldDescriptorProtoType(pub u32);
#[protoenum(closed)]
impl FieldDescriptorProtoType {
    #[var(1u32, "TYPE_DOUBLE")]
    pub const TYPE_DOUBLE: FieldDescriptorProtoType = FieldDescriptorProtoType(1u32);
    #[var(2u32, "TYPE_FLOAT")]
    pub const TYPE_FLOAT: FieldDescriptorProtoType = FieldDescriptorProtoType(2u32);
    #[var(3u32, "TYPE_INT64")]
    pub const TYPE_INT64: FieldDescriptorProtoType = FieldDescriptorProtoType(3u32);
    #[var(4u32, "TYPE_UINT64")]
    pub const TYPE_UINT64: FieldDescriptorProtoType = FieldDescriptorProtoType(4u32);
    #[var(5u32, "TYPE_INT32")]
    pub const TYPE_INT32: FieldDescriptorProtoType = FieldDescriptorProtoType(5u32);
    #[var(6u32, "TYPE_FIXED64")]
    pub const TYPE_FIXED64: FieldDescriptorProtoType = FieldDescriptorProtoType(6u32);
    #[var(7u32, "TYPE_FIXED32")]
    pub const TYPE_FIXED32: FieldDescriptorProtoType = FieldDescriptorProtoType(7u32);
    #[var(8u32, "TYPE_BOOL")]
    pub const TYPE_BOOL: FieldDescriptorProtoType = FieldDescriptorProtoType(8u32);
    #[var(9u32, "TYPE_STRING")]
    pub const TYPE_STRING: FieldDescriptorProtoType = FieldDescriptorProtoType(9u32);
    #[var(10u32, "TYPE_GROUP")]
    pub const TYPE_GROUP: FieldDescriptorProtoType = FieldDescriptorProtoType(10u32);
    #[var(11u32, "TYPE_MESSAGE")]
    pub const TYPE_MESSAGE: FieldDescriptorProtoType = FieldDescriptorProtoType(11u32);
    #[var(12u32, "TYPE_BYTES")]
    pub const TYPE_BYTES: FieldDescriptorProtoType = FieldDescriptorProtoType(12u32);
    #[var(13u32, "TYPE_UINT32")]
    pub const TYPE_UINT32: FieldDescriptorProtoType = FieldDescriptorProtoType(13u32);
    #[var(14u32, "TYPE_ENUM")]
    pub const TYPE_ENUM: FieldDescriptorProtoType = FieldDescriptorProtoType(14u32);
    #[var(15u32, "TYPE_SFIXED32")]
    pub const TYPE_SFIXED32: FieldDescriptorProtoType = FieldDescriptorProtoType(15u32);
    #[var(16u32, "TYPE_SFIXED64")]
    pub const TYPE_SFIXED64: FieldDescriptorProtoType = FieldDescriptorProtoType(16u32);
    #[var(17u32, "TYPE_SINT32")]
    pub const TYPE_SINT32: FieldDescriptorProtoType = FieldDescriptorProtoType(17u32);
    #[var(18u32, "TYPE_SINT64")]
    pub const TYPE_SINT64: FieldDescriptorProtoType = FieldDescriptorProtoType(18u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldDescriptorProtoLabel(pub u32);
#[protoenum(closed)]
impl FieldDescriptorProtoLabel {
    #[var(1u32, "LABEL_OPTIONAL")]
    pub const LABEL_OPTIONAL: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(1u32);
    #[var(2u32, "LABEL_REQUIRED")]
    pub const LABEL_REQUIRED: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(2u32);
    #[var(3u32, "LABEL_REPEATED")]
    pub const LABEL_REPEATED: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(3u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileOptionsOptimizeMode(pub u32);
#[protoenum(closed)]
impl FileOptionsOptimizeMode {
    #[var(1u32, "SPEED")]
    pub const SPEED: FileOptionsOptimizeMode = FileOptionsOptimizeMode(1u32);
    #[var(2u32, "CODE_SIZE")]
    pub const CODE_SIZE: FileOptionsOptimizeMode = FileOptionsOptimizeMode(2u32);
    #[var(3u32, "LITE_RUNTIME")]
    pub const LITE_RUNTIME: FileOptionsOptimizeMode = FileOptionsOptimizeMode(3u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsCType(pub u32);
#[protoenum(closed)]
impl FieldOptionsCType {
    #[var(0u32, "STRING")]
    pub const STRING: FieldOptionsCType = FieldOptionsCType(0u32);
    #[var(1u32, "CORD")]
    pub const CORD: FieldOptionsCType = FieldOptionsCType(1u32);
    #[var(2u32, "STRING_PIECE")]
    pub const STRING_PIECE: FieldOptionsCType = FieldOptionsCType(2u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsJSType(pub u32);
#[protoenum(closed)]
impl FieldOptionsJSType {
    #[var(0u32, "JS_NORMAL")]
    pub const JS_NORMAL: FieldOptionsJSType = FieldOptionsJSType(0u32);
    #[var(1u32, "JS_STRING")]
    pub const JS_STRING: FieldOptionsJSType = FieldOptionsJSType(1u32);
    #[var(2u32, "JS_NUMBER")]
    pub const JS_NUMBER: FieldOptionsJSType = FieldOptionsJSType(2u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodOptionsIdempotencyLevel(pub u32);
#[protoenum(closed)]
impl MethodOptionsIdempotencyLevel {
    #[var(0u32, "IDEMPOTENCY_UNKNOWN")]
    pub const IDEMPOTENCY_UNKNOWN: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(0u32);
    #[var(1u32, "NO_SIDE_EFFECTS")]
    pub const NO_SIDE_EFFECTS: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(1u32);
    #[var(2u32, "IDEMPOTENT")]
    pub const IDEMPOTENT: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(2u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FileDescriptorSet {
    #[field(1u32, "file", nested, repeated)]
    pub file: Vec<FileDescriptorProto>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FileDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "package", string, optional)]
    pub package: Option<String>,
    #[field(3u32, "dependency", string, repeated)]
    pub dependency: Vec<String>,
    #[field(10u32, "public_dependency", varint, repeated)]
    pub public_dependency: Vec<i32>,
    #[field(11u32, "weak_dependency", varint, repeated)]
    pub weak_dependency: Vec<i32>,
    #[field(4u32, "message_type", nested, repeated)]
    pub message_type: Vec<DescriptorProto>,
    #[field(5u32, "enum_type", nested, repeated)]
    pub enum_type: Vec<EnumDescriptorProto>,
    #[field(6u32, "service", nested, repeated)]
    pub service: Vec<ServiceDescriptorProto>,
    #[field(7u32, "extension", nested, repeated)]
    pub extension: Vec<FieldDescriptorProto>,
    #[field(8u32, "options", nested, optional)]
    pub options: Option<Box<FileOptions>>,
    #[field(9u32, "source_code_info", nested, optional)]
    pub source_code_info: Option<Box<SourceCodeInfo>>,
    #[field(12u32, "syntax", string, optional)]
    pub syntax: Option<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct DescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "field", nested, repeated)]
    pub field: Vec<FieldDescriptorProto>,
    #[field(6u32, "extension", nested, repeated)]
    pub extension: Vec<FieldDescriptorProto>,
    #[field(3u32, "nested_type", nested, repeated)]
    pub nested_type: Vec<DescriptorProto>,
    #[field(4u32, "enum_type", nested, repeated)]
    pub enum_type: Vec<EnumDescriptorProto>,
    #[field(5u32, "extension_range", nested, repeated)]
    pub extension_range: Vec<DescriptorProtoExtensionRange>,
    #[field(8u32, "oneof_decl", nested, repeated)]
    pub oneof_decl: Vec<OneofDescriptorProto>,
    #[field(7u32, "options", nested, optional)]
    pub options: Option<Box<MessageOptions>>,
    #[field(9u32, "reserved_range", nested, repeated)]
    pub reserved_range: Vec<DescriptorProtoReservedRange>,
    #[field(10u32, "reserved_name", string, repeated)]
    pub reserved_name: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct DescriptorProtoExtensionRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<ExtensionRangeOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct DescriptorProtoReservedRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ExtensionRangeOptions {
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FieldDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(3u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(4u32, "label", protoenum, optional)]
    pub label: Option<FieldDescriptorProtoLabel>,
    #[field(5u32, "type", protoenum, optional)]
    pub r#type: Option<FieldDescriptorProtoType>,
    #[field(6u32, "type_name", string, optional)]
    pub type_name: Option<String>,
    #[field(2u32, "extendee", string, optional)]
    pub extendee: Option<String>,
    #[field(7u32, "default_value", string, optional)]
    pub default_value: Option<String>,
    #[field(9u32, "oneof_index", varint, optional)]
    pub oneof_index: Option<i32>,
    #[field(10u32, "json_name", string, optional)]
    pub json_name: Option<String>,
    #[field(8u32, "options", nested, optional)]
    pub options: Option<Box<FieldOptions>>,
    #[field(17u32, "proto3_optional", bool, optional)]
    pub proto3_optional: Option<bool>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct OneofDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "options", nested, optional)]
    pub options: Option<Box<OneofOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "value", nested, repeated)]
    pub value: Vec<EnumValueDescriptorProto>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<EnumOptions>>,
    #[field(4u32, "reserved_range", nested, repeated)]
    pub reserved_range: Vec<EnumDescriptorProtoEnumReservedRange>,
    #[field(5u32, "reserved_name", string, repeated)]
    pub reserved_name: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumDescriptorProtoEnumReservedRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumValueDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<EnumValueOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ServiceDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "method", nested, repeated)]
    pub method: Vec<MethodDescriptorProto>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<ServiceOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct MethodDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "input_type", string, optional)]
    pub input_type: Option<String>,
    #[field(3u32, "output_type", string, optional)]
    pub output_type: Option<String>,
    #[field(4u32, "options", nested, optional)]
    pub options: Option<Box<MethodOptions>>,
    #[field(5u32, "client_streaming", bool, optional)]
    pub client_streaming: Option<bool>,
    #[field(6u32, "server_streaming", bool, optional)]
    pub server_streaming: Option<bool>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FileOptions {
    #[field(1u32, "java_package", string, optional)]
    pub java_package: Option<String>,
    #[field(8u32, "java_outer_classname", string, optional)]
    pub java_outer_classname: Option<String>,
    #[field(10u32, "java_multiple_files", bool, optional)]
    pub java_multiple_files: Option<bool>,
    #[field(20u32, "java_generate_equals_and_hash", bool, optional)]
    pub java_generate_equals_and_hash: Option<bool>,
    #[field(27u32, "java_string_check_utf8", bool, optional)]
    pub java_string_check_utf8: Option<bool>,
    #[field(9u32, "optimize_for", protoenum, optional)]
    pub optimize_for: Option<FileOptionsOptimizeMode>,
    #[field(11u32, "go_package", string, optional)]
    pub go_package: Option<String>,
    #[field(16u32, "cc_generic_services", bool, optional)]
    pub cc_generic_services: Option<bool>,
    #[field(17u32, "java_generic_services", bool, optional)]
    pub java_generic_services: Option<bool>,
    #[field(18u32, "py_generic_services", bool, optional)]
    pub py_generic_services: Option<bool>,
    #[field(42u32, "php_generic_services", bool, optional)]
    pub php_generic_services: Option<bool>,
    #[field(23u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(31u32, "cc_enable_arenas", bool, optional)]
    pub cc_enable_arenas: Option<bool>,
    #[field(36u32, "objc_class_prefix", string, optional)]
    pub objc_class_prefix: Option<String>,
    #[field(37u32, "csharp_namespace", string, optional)]
    pub csharp_namespace: Option<String>,
    #[field(39u32, "swift_prefix", string, optional)]
    pub swift_prefix: Option<String>,
    #[field(40u32, "php_class_prefix", string, optional)]
    pub php_class_prefix: Option<String>,
    #[field(41u32, "php_namespace", string, optional)]
    pub php_namespace: Option<String>,
    #[field(44u32, "php_metadata_namespace", string, optional)]
    pub php_metadata_namespace: Option<String>,
    #[field(45u32, "ruby_package", string, optional)]
    pub ruby_package: Option<String>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct MessageOptions {
    #[field(1u32, "message_set_wire_format", bool, optional)]
    pub message_set_wire_format: Option<bool>,
    #[field(2u32, "no_standard_descriptor_accessor", bool, optional)]
    pub no_standard_descriptor_accessor: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(7u32, "map_entry", bool, optional)]
    pub map_entry: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FieldOptions {
    #[field(1u32, "ctype", protoenum, optional)]
    pub ctype: Option<FieldOptionsCType>,
    #[field(2u32, "packed", bool, optional)]
    pub packed: Option<bool>,
    #[field(6u32, "jstype", protoenum, optional)]
    pub jstype: Option<FieldOptionsJSType>,
    #[field(5u32, "lazy", bool, optional)]
    pub lazy: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(10u32, "weak", bool, optional)]
    pub weak: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct OneofOptions {
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumOptions {
    #[field(2u32, "allow_alias", bool, optional)]
    pub allow_alias: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumValueOptions {
    #[field(1u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ServiceOptions {
    #[field(33u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct MethodOptions {
    #[field(33u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(34u32, "idempotency_level", protoenum, optional)]
    pub idempotency_level: Option<MethodOptionsIdempotencyLevel>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct UninterpretedOption {
    #[field(2u32, "name", nested, repeated)]
    pub name: Vec<UninterpretedOptionNamePart>,
    #[field(3u32, "identifier_value", string, optional)]
    pub identifier_value: Option<String>,
    #[field(4u32, "positive_int_value", varint, optional)]
    pub positive_int_value: Option<u64>,
    #[field(5u32, "negative_int_value", varint, optional)]
    pub negative_int_value: Option<i64>,
    #[field(6u32, "double_value", fixed64, optional)]
    pub double_value: Option<f64>,
    #[field(7u32, "string_value", bytes, optional)]
    pub string_value: Option<Vec<u8>>,
    #[field(8u32, "aggregate_value", string, optional)]
    pub aggregate_value: Option<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct UninterpretedOptionNamePart {
    #[field(1u32, "name_part", string, required)]
    pub name_part: String,
    #[field(2u32, "is_extension", bool, required)]
    pub is_extension: bool,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct SourceCodeInfo {
    #[field(1u32, "location", nested, repeated)]
    pub location: Vec<SourceCodeInfoLocation>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct SourceCodeInfoLocation {
    #[field(1u32, "path", varint, repeated)]
    pub path: Vec<i32>,
    #[field(2u32, "span", varint, repeated)]
    pub span: Vec<i32>,
    #[field(3u32, "leading_comments", string, optional)]
    pub leading_comments: Option<String>,
    #[field(4u32, "trailing_comments", string, optional)]
    pub trailing_comments: Option<String>,
    #[field(6u32, "leading_detached_comments", string, repeated)]
    pub leading_detached_comments: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct GeneratedCodeInfo {
    #[field(1u32, "annotation", nested, repeated)]
    pub annotation: Vec<GeneratedCodeInfoAnnotation>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct GeneratedCodeInfoAnnotation {
    #[field(1u32, "path", varint, repeated)]
    pub path: Vec<i32>,
    #[field(2u32, "source_file", string, optional)]
    pub source_file: Option<String>,
    #[field(3u32, "begin", varint, optional)]
    pub begin: Option<i32>,
    #[field(4u32, "end", varint, optional)]
    pub end: Option<i32>,
}
