#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use crate as protokit;
use protokit::*;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&FileDescriptorSet::default());
    registry.register(&FileDescriptorProto::default());
    registry.register(&DescriptorProtoExtensionRange::default());
    registry.register(&DescriptorProtoReservedRange::default());
    registry.register(&DescriptorProto::default());
    registry.register(&ExtensionRangeOptionsDeclaration::default());
    registry.register(&ExtensionRangeOptions::default());
    registry.register(&FieldDescriptorProto::default());
    registry.register(&OneofDescriptorProto::default());
    registry.register(&EnumDescriptorProtoEnumReservedRange::default());
    registry.register(&EnumDescriptorProto::default());
    registry.register(&EnumValueDescriptorProto::default());
    registry.register(&ServiceDescriptorProto::default());
    registry.register(&MethodDescriptorProto::default());
    registry.register(&FileOptions::default());
    registry.register(&MessageOptions::default());
    registry.register(&FieldOptionsEditionDefault::default());
    registry.register(&FieldOptionsFeatureSupport::default());
    registry.register(&FieldOptions::default());
    registry.register(&OneofOptions::default());
    registry.register(&EnumOptions::default());
    registry.register(&EnumValueOptions::default());
    registry.register(&ServiceOptions::default());
    registry.register(&MethodOptions::default());
    registry.register(&UninterpretedOptionNamePart::default());
    registry.register(&UninterpretedOption::default());
    registry.register(&FeatureSetVisibilityFeature::default());
    registry.register(&FeatureSet::default());
    registry.register(&FeatureSetDefaultsFeatureSetEditionDefault::default());
    registry.register(&FeatureSetDefaults::default());
    registry.register(&SourceCodeInfoLocation::default());
    registry.register(&SourceCodeInfo::default());
    registry.register(&GeneratedCodeInfoAnnotation::default());
    registry.register(&GeneratedCodeInfo::default());
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edition(pub i32);
#[protoenum]
impl Edition {
    #[var(0i32, "EDITION_UNKNOWN")]
    pub const EDITION_UNKNOWN: Edition = Edition(0i32);
    #[var(900i32, "EDITION_LEGACY")]
    pub const EDITION_LEGACY: Edition = Edition(900i32);
    #[var(998i32, "EDITION_PROTO2")]
    pub const EDITION_PROTO2: Edition = Edition(998i32);
    #[var(999i32, "EDITION_PROTO3")]
    pub const EDITION_PROTO3: Edition = Edition(999i32);
    #[var(1000i32, "EDITION_2023")]
    pub const EDITION_2023: Edition = Edition(1000i32);
    #[var(1001i32, "EDITION_2024")]
    pub const EDITION_2024: Edition = Edition(1001i32);
    #[var(1i32, "EDITION_1_TEST_ONLY")]
    pub const EDITION_1_TEST_ONLY: Edition = Edition(1i32);
    #[var(2i32, "EDITION_2_TEST_ONLY")]
    pub const EDITION_2_TEST_ONLY: Edition = Edition(2i32);
    #[var(99997i32, "EDITION_99997_TEST_ONLY")]
    pub const EDITION_99997_TEST_ONLY: Edition = Edition(99997i32);
    #[var(99998i32, "EDITION_99998_TEST_ONLY")]
    pub const EDITION_99998_TEST_ONLY: Edition = Edition(99998i32);
    #[var(99999i32, "EDITION_99999_TEST_ONLY")]
    pub const EDITION_99999_TEST_ONLY: Edition = Edition(99999i32);
    #[var(2147483647i32, "EDITION_MAX")]
    pub const EDITION_MAX: Edition = Edition(2147483647i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolVisibility(pub i32);
#[protoenum]
impl SymbolVisibility {
    #[var(0i32, "VISIBILITY_UNSET")]
    pub const VISIBILITY_UNSET: SymbolVisibility = SymbolVisibility(0i32);
    #[var(1i32, "VISIBILITY_LOCAL")]
    pub const VISIBILITY_LOCAL: SymbolVisibility = SymbolVisibility(1i32);
    #[var(2i32, "VISIBILITY_EXPORT")]
    pub const VISIBILITY_EXPORT: SymbolVisibility = SymbolVisibility(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionRangeOptionsVerificationState(pub i32);
#[protoenum]
impl ExtensionRangeOptionsVerificationState {
    #[var(0i32, "DECLARATION")]
    pub const DECLARATION: ExtensionRangeOptionsVerificationState = ExtensionRangeOptionsVerificationState(
        0i32,
    );
    #[var(1i32, "UNVERIFIED")]
    pub const UNVERIFIED: ExtensionRangeOptionsVerificationState = ExtensionRangeOptionsVerificationState(
        1i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldDescriptorProtoType(pub i32);
#[protoenum]
impl FieldDescriptorProtoType {
    #[var(1i32, "TYPE_DOUBLE")]
    pub const TYPE_DOUBLE: FieldDescriptorProtoType = FieldDescriptorProtoType(1i32);
    #[var(2i32, "TYPE_FLOAT")]
    pub const TYPE_FLOAT: FieldDescriptorProtoType = FieldDescriptorProtoType(2i32);
    #[var(3i32, "TYPE_INT64")]
    pub const TYPE_INT64: FieldDescriptorProtoType = FieldDescriptorProtoType(3i32);
    #[var(4i32, "TYPE_UINT64")]
    pub const TYPE_UINT64: FieldDescriptorProtoType = FieldDescriptorProtoType(4i32);
    #[var(5i32, "TYPE_INT32")]
    pub const TYPE_INT32: FieldDescriptorProtoType = FieldDescriptorProtoType(5i32);
    #[var(6i32, "TYPE_FIXED64")]
    pub const TYPE_FIXED64: FieldDescriptorProtoType = FieldDescriptorProtoType(6i32);
    #[var(7i32, "TYPE_FIXED32")]
    pub const TYPE_FIXED32: FieldDescriptorProtoType = FieldDescriptorProtoType(7i32);
    #[var(8i32, "TYPE_BOOL")]
    pub const TYPE_BOOL: FieldDescriptorProtoType = FieldDescriptorProtoType(8i32);
    #[var(9i32, "TYPE_STRING")]
    pub const TYPE_STRING: FieldDescriptorProtoType = FieldDescriptorProtoType(9i32);
    #[var(10i32, "TYPE_GROUP")]
    pub const TYPE_GROUP: FieldDescriptorProtoType = FieldDescriptorProtoType(10i32);
    #[var(11i32, "TYPE_MESSAGE")]
    pub const TYPE_MESSAGE: FieldDescriptorProtoType = FieldDescriptorProtoType(11i32);
    #[var(12i32, "TYPE_BYTES")]
    pub const TYPE_BYTES: FieldDescriptorProtoType = FieldDescriptorProtoType(12i32);
    #[var(13i32, "TYPE_UINT32")]
    pub const TYPE_UINT32: FieldDescriptorProtoType = FieldDescriptorProtoType(13i32);
    #[var(14i32, "TYPE_ENUM")]
    pub const TYPE_ENUM: FieldDescriptorProtoType = FieldDescriptorProtoType(14i32);
    #[var(15i32, "TYPE_SFIXED32")]
    pub const TYPE_SFIXED32: FieldDescriptorProtoType = FieldDescriptorProtoType(15i32);
    #[var(16i32, "TYPE_SFIXED64")]
    pub const TYPE_SFIXED64: FieldDescriptorProtoType = FieldDescriptorProtoType(16i32);
    #[var(17i32, "TYPE_SINT32")]
    pub const TYPE_SINT32: FieldDescriptorProtoType = FieldDescriptorProtoType(17i32);
    #[var(18i32, "TYPE_SINT64")]
    pub const TYPE_SINT64: FieldDescriptorProtoType = FieldDescriptorProtoType(18i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldDescriptorProtoLabel(pub i32);
#[protoenum]
impl FieldDescriptorProtoLabel {
    #[var(1i32, "LABEL_OPTIONAL")]
    pub const LABEL_OPTIONAL: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(
        1i32,
    );
    #[var(3i32, "LABEL_REPEATED")]
    pub const LABEL_REPEATED: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(
        3i32,
    );
    #[var(2i32, "LABEL_REQUIRED")]
    pub const LABEL_REQUIRED: FieldDescriptorProtoLabel = FieldDescriptorProtoLabel(
        2i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileOptionsOptimizeMode(pub i32);
#[protoenum]
impl FileOptionsOptimizeMode {
    #[var(1i32, "SPEED")]
    pub const SPEED: FileOptionsOptimizeMode = FileOptionsOptimizeMode(1i32);
    #[var(2i32, "CODE_SIZE")]
    pub const CODE_SIZE: FileOptionsOptimizeMode = FileOptionsOptimizeMode(2i32);
    #[var(3i32, "LITE_RUNTIME")]
    pub const LITE_RUNTIME: FileOptionsOptimizeMode = FileOptionsOptimizeMode(3i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsCType(pub i32);
#[protoenum]
impl FieldOptionsCType {
    #[var(0i32, "STRING")]
    pub const STRING: FieldOptionsCType = FieldOptionsCType(0i32);
    #[var(1i32, "CORD")]
    pub const CORD: FieldOptionsCType = FieldOptionsCType(1i32);
    #[var(2i32, "STRING_PIECE")]
    pub const STRING_PIECE: FieldOptionsCType = FieldOptionsCType(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsJSType(pub i32);
#[protoenum]
impl FieldOptionsJSType {
    #[var(0i32, "JS_NORMAL")]
    pub const JS_NORMAL: FieldOptionsJSType = FieldOptionsJSType(0i32);
    #[var(1i32, "JS_STRING")]
    pub const JS_STRING: FieldOptionsJSType = FieldOptionsJSType(1i32);
    #[var(2i32, "JS_NUMBER")]
    pub const JS_NUMBER: FieldOptionsJSType = FieldOptionsJSType(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsOptionRetention(pub i32);
#[protoenum]
impl FieldOptionsOptionRetention {
    #[var(0i32, "RETENTION_UNKNOWN")]
    pub const RETENTION_UNKNOWN: FieldOptionsOptionRetention = FieldOptionsOptionRetention(
        0i32,
    );
    #[var(1i32, "RETENTION_RUNTIME")]
    pub const RETENTION_RUNTIME: FieldOptionsOptionRetention = FieldOptionsOptionRetention(
        1i32,
    );
    #[var(2i32, "RETENTION_SOURCE")]
    pub const RETENTION_SOURCE: FieldOptionsOptionRetention = FieldOptionsOptionRetention(
        2i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldOptionsOptionTargetType(pub i32);
#[protoenum]
impl FieldOptionsOptionTargetType {
    #[var(0i32, "TARGET_TYPE_UNKNOWN")]
    pub const TARGET_TYPE_UNKNOWN: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        0i32,
    );
    #[var(1i32, "TARGET_TYPE_FILE")]
    pub const TARGET_TYPE_FILE: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        1i32,
    );
    #[var(2i32, "TARGET_TYPE_EXTENSION_RANGE")]
    pub const TARGET_TYPE_EXTENSION_RANGE: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        2i32,
    );
    #[var(3i32, "TARGET_TYPE_MESSAGE")]
    pub const TARGET_TYPE_MESSAGE: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        3i32,
    );
    #[var(4i32, "TARGET_TYPE_FIELD")]
    pub const TARGET_TYPE_FIELD: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        4i32,
    );
    #[var(5i32, "TARGET_TYPE_ONEOF")]
    pub const TARGET_TYPE_ONEOF: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        5i32,
    );
    #[var(6i32, "TARGET_TYPE_ENUM")]
    pub const TARGET_TYPE_ENUM: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        6i32,
    );
    #[var(7i32, "TARGET_TYPE_ENUM_ENTRY")]
    pub const TARGET_TYPE_ENUM_ENTRY: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        7i32,
    );
    #[var(8i32, "TARGET_TYPE_SERVICE")]
    pub const TARGET_TYPE_SERVICE: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        8i32,
    );
    #[var(9i32, "TARGET_TYPE_METHOD")]
    pub const TARGET_TYPE_METHOD: FieldOptionsOptionTargetType = FieldOptionsOptionTargetType(
        9i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodOptionsIdempotencyLevel(pub i32);
#[protoenum]
impl MethodOptionsIdempotencyLevel {
    #[var(0i32, "IDEMPOTENCY_UNKNOWN")]
    pub const IDEMPOTENCY_UNKNOWN: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(
        0i32,
    );
    #[var(1i32, "NO_SIDE_EFFECTS")]
    pub const NO_SIDE_EFFECTS: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(
        1i32,
    );
    #[var(2i32, "IDEMPOTENT")]
    pub const IDEMPOTENT: MethodOptionsIdempotencyLevel = MethodOptionsIdempotencyLevel(
        2i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetVisibilityFeatureDefaultSymbolVisibility(pub i32);
#[protoenum]
impl FeatureSetVisibilityFeatureDefaultSymbolVisibility {
    #[var(0i32, "DEFAULT_SYMBOL_VISIBILITY_UNKNOWN")]
    pub const DEFAULT_SYMBOL_VISIBILITY_UNKNOWN: FeatureSetVisibilityFeatureDefaultSymbolVisibility = FeatureSetVisibilityFeatureDefaultSymbolVisibility(
        0i32,
    );
    #[var(1i32, "EXPORT_ALL")]
    pub const EXPORT_ALL: FeatureSetVisibilityFeatureDefaultSymbolVisibility = FeatureSetVisibilityFeatureDefaultSymbolVisibility(
        1i32,
    );
    #[var(2i32, "EXPORT_TOP_LEVEL")]
    pub const EXPORT_TOP_LEVEL: FeatureSetVisibilityFeatureDefaultSymbolVisibility = FeatureSetVisibilityFeatureDefaultSymbolVisibility(
        2i32,
    );
    #[var(3i32, "LOCAL_ALL")]
    pub const LOCAL_ALL: FeatureSetVisibilityFeatureDefaultSymbolVisibility = FeatureSetVisibilityFeatureDefaultSymbolVisibility(
        3i32,
    );
    #[var(4i32, "STRICT")]
    pub const STRICT: FeatureSetVisibilityFeatureDefaultSymbolVisibility = FeatureSetVisibilityFeatureDefaultSymbolVisibility(
        4i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetFieldPresence(pub i32);
#[protoenum]
impl FeatureSetFieldPresence {
    #[var(0i32, "FIELD_PRESENCE_UNKNOWN")]
    pub const FIELD_PRESENCE_UNKNOWN: FeatureSetFieldPresence = FeatureSetFieldPresence(
        0i32,
    );
    #[var(1i32, "EXPLICIT")]
    pub const EXPLICIT: FeatureSetFieldPresence = FeatureSetFieldPresence(1i32);
    #[var(2i32, "IMPLICIT")]
    pub const IMPLICIT: FeatureSetFieldPresence = FeatureSetFieldPresence(2i32);
    #[var(3i32, "LEGACY_REQUIRED")]
    pub const LEGACY_REQUIRED: FeatureSetFieldPresence = FeatureSetFieldPresence(3i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetEnumType(pub i32);
#[protoenum]
impl FeatureSetEnumType {
    #[var(0i32, "ENUM_TYPE_UNKNOWN")]
    pub const ENUM_TYPE_UNKNOWN: FeatureSetEnumType = FeatureSetEnumType(0i32);
    #[var(1i32, "OPEN")]
    pub const OPEN: FeatureSetEnumType = FeatureSetEnumType(1i32);
    #[var(2i32, "CLOSED")]
    pub const CLOSED: FeatureSetEnumType = FeatureSetEnumType(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetRepeatedFieldEncoding(pub i32);
#[protoenum]
impl FeatureSetRepeatedFieldEncoding {
    #[var(0i32, "REPEATED_FIELD_ENCODING_UNKNOWN")]
    pub const REPEATED_FIELD_ENCODING_UNKNOWN: FeatureSetRepeatedFieldEncoding = FeatureSetRepeatedFieldEncoding(
        0i32,
    );
    #[var(1i32, "PACKED")]
    pub const PACKED: FeatureSetRepeatedFieldEncoding = FeatureSetRepeatedFieldEncoding(
        1i32,
    );
    #[var(2i32, "EXPANDED")]
    pub const EXPANDED: FeatureSetRepeatedFieldEncoding = FeatureSetRepeatedFieldEncoding(
        2i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetUtf8Validation(pub i32);
#[protoenum]
impl FeatureSetUtf8Validation {
    #[var(0i32, "UTF8_VALIDATION_UNKNOWN")]
    pub const UTF8_VALIDATION_UNKNOWN: FeatureSetUtf8Validation = FeatureSetUtf8Validation(
        0i32,
    );
    #[var(2i32, "VERIFY")]
    pub const VERIFY: FeatureSetUtf8Validation = FeatureSetUtf8Validation(2i32);
    #[var(3i32, "NONE")]
    pub const NONE: FeatureSetUtf8Validation = FeatureSetUtf8Validation(3i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetMessageEncoding(pub i32);
#[protoenum]
impl FeatureSetMessageEncoding {
    #[var(0i32, "MESSAGE_ENCODING_UNKNOWN")]
    pub const MESSAGE_ENCODING_UNKNOWN: FeatureSetMessageEncoding = FeatureSetMessageEncoding(
        0i32,
    );
    #[var(1i32, "LENGTH_PREFIXED")]
    pub const LENGTH_PREFIXED: FeatureSetMessageEncoding = FeatureSetMessageEncoding(
        1i32,
    );
    #[var(2i32, "DELIMITED")]
    pub const DELIMITED: FeatureSetMessageEncoding = FeatureSetMessageEncoding(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetJsonFormat(pub i32);
#[protoenum]
impl FeatureSetJsonFormat {
    #[var(0i32, "JSON_FORMAT_UNKNOWN")]
    pub const JSON_FORMAT_UNKNOWN: FeatureSetJsonFormat = FeatureSetJsonFormat(0i32);
    #[var(1i32, "ALLOW")]
    pub const ALLOW: FeatureSetJsonFormat = FeatureSetJsonFormat(1i32);
    #[var(2i32, "LEGACY_BEST_EFFORT")]
    pub const LEGACY_BEST_EFFORT: FeatureSetJsonFormat = FeatureSetJsonFormat(2i32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureSetEnforceNamingStyle(pub i32);
#[protoenum]
impl FeatureSetEnforceNamingStyle {
    #[var(0i32, "ENFORCE_NAMING_STYLE_UNKNOWN")]
    pub const ENFORCE_NAMING_STYLE_UNKNOWN: FeatureSetEnforceNamingStyle = FeatureSetEnforceNamingStyle(
        0i32,
    );
    #[var(1i32, "STYLE2024")]
    pub const STYLE2024: FeatureSetEnforceNamingStyle = FeatureSetEnforceNamingStyle(
        1i32,
    );
    #[var(2i32, "STYLE_LEGACY")]
    pub const STYLE_LEGACY: FeatureSetEnforceNamingStyle = FeatureSetEnforceNamingStyle(
        2i32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneratedCodeInfoAnnotationSemantic(pub i32);
#[protoenum]
impl GeneratedCodeInfoAnnotationSemantic {
    #[var(0i32, "NONE")]
    pub const NONE: GeneratedCodeInfoAnnotationSemantic = GeneratedCodeInfoAnnotationSemantic(
        0i32,
    );
    #[var(1i32, "SET")]
    pub const SET: GeneratedCodeInfoAnnotationSemantic = GeneratedCodeInfoAnnotationSemantic(
        1i32,
    );
    #[var(2i32, "ALIAS")]
    pub const ALIAS: GeneratedCodeInfoAnnotationSemantic = GeneratedCodeInfoAnnotationSemantic(
        2i32,
    );
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FileDescriptorSet", package = "google.protobuf")]
pub struct FileDescriptorSet {
    #[field(1u32, "file", nested, repeated)]
    pub file: Vec<FileDescriptorProto>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FileDescriptorProto", package = "google.protobuf")]
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
    #[field(15u32, "option_dependency", string, repeated)]
    pub option_dependency: Vec<String>,
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
    #[field(14u32, "edition", protoenum, optional)]
    pub edition: Option<Edition>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "DescriptorProto.ExtensionRange", package = "google.protobuf")]
pub struct DescriptorProtoExtensionRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<ExtensionRangeOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "DescriptorProto.ReservedRange", package = "google.protobuf")]
pub struct DescriptorProtoReservedRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "DescriptorProto", package = "google.protobuf")]
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
    #[field(11u32, "visibility", protoenum, optional)]
    pub visibility: Option<SymbolVisibility>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "ExtensionRangeOptions.Declaration", package = "google.protobuf")]
pub struct ExtensionRangeOptionsDeclaration {
    #[field(1u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(2u32, "full_name", string, optional)]
    pub full_name: Option<String>,
    #[field(3u32, "type", string, optional)]
    pub r#type: Option<String>,
    #[field(5u32, "reserved", bool, optional)]
    pub reserved: Option<bool>,
    #[field(6u32, "repeated", bool, optional)]
    pub repeated: Option<bool>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "ExtensionRangeOptions", package = "google.protobuf")]
pub struct ExtensionRangeOptions {
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
    #[field(2u32, "declaration", nested, repeated)]
    pub declaration: Vec<ExtensionRangeOptionsDeclaration>,
    #[field(50u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(3u32, "verification", protoenum, optional)]
    pub verification: Option<ExtensionRangeOptionsVerificationState>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FieldDescriptorProto", package = "google.protobuf")]
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
#[proto(name = "OneofDescriptorProto", package = "google.protobuf")]
pub struct OneofDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "options", nested, optional)]
    pub options: Option<Box<OneofOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "EnumDescriptorProto.EnumReservedRange", package = "google.protobuf")]
pub struct EnumDescriptorProtoEnumReservedRange {
    #[field(1u32, "start", varint, optional)]
    pub start: Option<i32>,
    #[field(2u32, "end", varint, optional)]
    pub end: Option<i32>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "EnumDescriptorProto", package = "google.protobuf")]
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
    #[field(6u32, "visibility", protoenum, optional)]
    pub visibility: Option<SymbolVisibility>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "EnumValueDescriptorProto", package = "google.protobuf")]
pub struct EnumValueDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<EnumValueOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "ServiceDescriptorProto", package = "google.protobuf")]
pub struct ServiceDescriptorProto {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "method", nested, repeated)]
    pub method: Vec<MethodDescriptorProto>,
    #[field(3u32, "options", nested, optional)]
    pub options: Option<Box<ServiceOptions>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "MethodDescriptorProto", package = "google.protobuf")]
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
#[proto(name = "FileOptions", package = "google.protobuf")]
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
    #[field(50u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "MessageOptions", package = "google.protobuf")]
pub struct MessageOptions {
    #[field(1u32, "message_set_wire_format", bool, optional)]
    pub message_set_wire_format: Option<bool>,
    #[field(2u32, "no_standard_descriptor_accessor", bool, optional)]
    pub no_standard_descriptor_accessor: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(7u32, "map_entry", bool, optional)]
    pub map_entry: Option<bool>,
    #[field(11u32, "deprecated_legacy_json_field_conflicts", bool, optional)]
    pub deprecated_legacy_json_field_conflicts: Option<bool>,
    #[field(12u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FieldOptions.EditionDefault", package = "google.protobuf")]
pub struct FieldOptionsEditionDefault {
    #[field(3u32, "edition", protoenum, optional)]
    pub edition: Option<Edition>,
    #[field(2u32, "value", string, optional)]
    pub value: Option<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FieldOptions.FeatureSupport", package = "google.protobuf")]
pub struct FieldOptionsFeatureSupport {
    #[field(1u32, "edition_introduced", protoenum, optional)]
    pub edition_introduced: Option<Edition>,
    #[field(2u32, "edition_deprecated", protoenum, optional)]
    pub edition_deprecated: Option<Edition>,
    #[field(3u32, "deprecation_warning", string, optional)]
    pub deprecation_warning: Option<String>,
    #[field(4u32, "edition_removed", protoenum, optional)]
    pub edition_removed: Option<Edition>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FieldOptions", package = "google.protobuf")]
pub struct FieldOptions {
    #[field(1u32, "ctype", protoenum, optional)]
    pub ctype: Option<FieldOptionsCType>,
    #[field(2u32, "packed", bool, optional)]
    pub packed: Option<bool>,
    #[field(6u32, "jstype", protoenum, optional)]
    pub jstype: Option<FieldOptionsJSType>,
    #[field(5u32, "lazy", bool, optional)]
    pub lazy: Option<bool>,
    #[field(15u32, "unverified_lazy", bool, optional)]
    pub unverified_lazy: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(10u32, "weak", bool, optional)]
    pub weak: Option<bool>,
    #[field(16u32, "debug_redact", bool, optional)]
    pub debug_redact: Option<bool>,
    #[field(17u32, "retention", protoenum, optional)]
    pub retention: Option<FieldOptionsOptionRetention>,
    #[field(19u32, "targets", protoenum, repeated)]
    pub targets: Vec<FieldOptionsOptionTargetType>,
    #[field(20u32, "edition_defaults", nested, repeated)]
    pub edition_defaults: Vec<FieldOptionsEditionDefault>,
    #[field(21u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(22u32, "feature_support", nested, optional)]
    pub feature_support: Option<FieldOptionsFeatureSupport>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "OneofOptions", package = "google.protobuf")]
pub struct OneofOptions {
    #[field(1u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "EnumOptions", package = "google.protobuf")]
pub struct EnumOptions {
    #[field(2u32, "allow_alias", bool, optional)]
    pub allow_alias: Option<bool>,
    #[field(3u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(6u32, "deprecated_legacy_json_field_conflicts", bool, optional)]
    pub deprecated_legacy_json_field_conflicts: Option<bool>,
    #[field(7u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "EnumValueOptions", package = "google.protobuf")]
pub struct EnumValueOptions {
    #[field(1u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(2u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(3u32, "debug_redact", bool, optional)]
    pub debug_redact: Option<bool>,
    #[field(4u32, "feature_support", nested, optional)]
    pub feature_support: Option<FieldOptionsFeatureSupport>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "ServiceOptions", package = "google.protobuf")]
pub struct ServiceOptions {
    #[field(34u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(33u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "MethodOptions", package = "google.protobuf")]
pub struct MethodOptions {
    #[field(33u32, "deprecated", bool, optional)]
    pub deprecated: Option<bool>,
    #[field(34u32, "idempotency_level", protoenum, optional)]
    pub idempotency_level: Option<MethodOptionsIdempotencyLevel>,
    #[field(35u32, "features", nested, optional)]
    pub features: Option<FeatureSet>,
    #[field(999u32, "uninterpreted_option", nested, repeated)]
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "UninterpretedOption.NamePart", package = "google.protobuf")]
pub struct UninterpretedOptionNamePart {
    #[field(1u32, "name_part", string, required)]
    pub name_part: String,
    #[field(2u32, "is_extension", bool, required)]
    pub is_extension: bool,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "UninterpretedOption", package = "google.protobuf")]
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
#[proto(name = "FeatureSet.VisibilityFeature", package = "google.protobuf")]
pub struct FeatureSetVisibilityFeature {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FeatureSet", package = "google.protobuf")]
pub struct FeatureSet {
    #[field(1u32, "field_presence", protoenum, optional)]
    pub field_presence: Option<FeatureSetFieldPresence>,
    #[field(2u32, "enum_type", protoenum, optional)]
    pub enum_type: Option<FeatureSetEnumType>,
    #[field(3u32, "repeated_field_encoding", protoenum, optional)]
    pub repeated_field_encoding: Option<FeatureSetRepeatedFieldEncoding>,
    #[field(4u32, "utf8_validation", protoenum, optional)]
    pub utf8_validation: Option<FeatureSetUtf8Validation>,
    #[field(5u32, "message_encoding", protoenum, optional)]
    pub message_encoding: Option<FeatureSetMessageEncoding>,
    #[field(6u32, "json_format", protoenum, optional)]
    pub json_format: Option<FeatureSetJsonFormat>,
    #[field(7u32, "enforce_naming_style", protoenum, optional)]
    pub enforce_naming_style: Option<FeatureSetEnforceNamingStyle>,
    #[field(8u32, "default_symbol_visibility", protoenum, optional)]
    pub default_symbol_visibility: Option<
        FeatureSetVisibilityFeatureDefaultSymbolVisibility,
    >,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(
    name = "FeatureSetDefaults.FeatureSetEditionDefault",
    package = "google.protobuf"
)]
pub struct FeatureSetDefaultsFeatureSetEditionDefault {
    #[field(3u32, "edition", protoenum, optional)]
    pub edition: Option<Edition>,
    #[field(4u32, "overridable_features", nested, optional)]
    pub overridable_features: Option<FeatureSet>,
    #[field(5u32, "fixed_features", nested, optional)]
    pub fixed_features: Option<FeatureSet>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FeatureSetDefaults", package = "google.protobuf")]
pub struct FeatureSetDefaults {
    #[field(1u32, "defaults", nested, repeated)]
    pub defaults: Vec<FeatureSetDefaultsFeatureSetEditionDefault>,
    #[field(4u32, "minimum_edition", protoenum, optional)]
    pub minimum_edition: Option<Edition>,
    #[field(5u32, "maximum_edition", protoenum, optional)]
    pub maximum_edition: Option<Edition>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "SourceCodeInfo.Location", package = "google.protobuf")]
pub struct SourceCodeInfoLocation {
    #[field(1u32, "path", varint, packed)]
    pub path: Vec<i32>,
    #[field(2u32, "span", varint, packed)]
    pub span: Vec<i32>,
    #[field(3u32, "leading_comments", string, optional)]
    pub leading_comments: Option<String>,
    #[field(4u32, "trailing_comments", string, optional)]
    pub trailing_comments: Option<String>,
    #[field(6u32, "leading_detached_comments", string, repeated)]
    pub leading_detached_comments: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "SourceCodeInfo", package = "google.protobuf")]
pub struct SourceCodeInfo {
    #[field(1u32, "location", nested, repeated)]
    pub location: Vec<SourceCodeInfoLocation>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "GeneratedCodeInfo.Annotation", package = "google.protobuf")]
pub struct GeneratedCodeInfoAnnotation {
    #[field(1u32, "path", varint, packed)]
    pub path: Vec<i32>,
    #[field(2u32, "source_file", string, optional)]
    pub source_file: Option<String>,
    #[field(3u32, "begin", varint, optional)]
    pub begin: Option<i32>,
    #[field(4u32, "end", varint, optional)]
    pub end: Option<i32>,
    #[field(5u32, "semantic", protoenum, optional)]
    pub semantic: Option<GeneratedCodeInfoAnnotationSemantic>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "GeneratedCodeInfo", package = "google.protobuf")]
pub struct GeneratedCodeInfo {
    #[field(1u32, "annotation", nested, repeated)]
    pub annotation: Vec<GeneratedCodeInfoAnnotation>,
}
