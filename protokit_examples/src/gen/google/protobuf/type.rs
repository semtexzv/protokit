#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::any::*;
use super::source_context::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field__Kind(pub u32);
#[protoenum]
impl Field__Kind {
    #[var(0u32, "TYPE_UNKNOWN")]
    pub const TYPE_UNKNOWN: Field__Kind = Field__Kind(0u32);
    #[var(1u32, "TYPE_DOUBLE")]
    pub const TYPE_DOUBLE: Field__Kind = Field__Kind(1u32);
    #[var(2u32, "TYPE_FLOAT")]
    pub const TYPE_FLOAT: Field__Kind = Field__Kind(2u32);
    #[var(3u32, "TYPE_INT64")]
    pub const TYPE_INT64: Field__Kind = Field__Kind(3u32);
    #[var(4u32, "TYPE_UINT64")]
    pub const TYPE_UINT64: Field__Kind = Field__Kind(4u32);
    #[var(5u32, "TYPE_INT32")]
    pub const TYPE_INT32: Field__Kind = Field__Kind(5u32);
    #[var(6u32, "TYPE_FIXED64")]
    pub const TYPE_FIXED64: Field__Kind = Field__Kind(6u32);
    #[var(7u32, "TYPE_FIXED32")]
    pub const TYPE_FIXED32: Field__Kind = Field__Kind(7u32);
    #[var(8u32, "TYPE_BOOL")]
    pub const TYPE_BOOL: Field__Kind = Field__Kind(8u32);
    #[var(9u32, "TYPE_STRING")]
    pub const TYPE_STRING: Field__Kind = Field__Kind(9u32);
    #[var(10u32, "TYPE_GROUP")]
    pub const TYPE_GROUP: Field__Kind = Field__Kind(10u32);
    #[var(11u32, "TYPE_MESSAGE")]
    pub const TYPE_MESSAGE: Field__Kind = Field__Kind(11u32);
    #[var(12u32, "TYPE_BYTES")]
    pub const TYPE_BYTES: Field__Kind = Field__Kind(12u32);
    #[var(13u32, "TYPE_UINT32")]
    pub const TYPE_UINT32: Field__Kind = Field__Kind(13u32);
    #[var(14u32, "TYPE_ENUM")]
    pub const TYPE_ENUM: Field__Kind = Field__Kind(14u32);
    #[var(15u32, "TYPE_SFIXED32")]
    pub const TYPE_SFIXED32: Field__Kind = Field__Kind(15u32);
    #[var(16u32, "TYPE_SFIXED64")]
    pub const TYPE_SFIXED64: Field__Kind = Field__Kind(16u32);
    #[var(17u32, "TYPE_SINT32")]
    pub const TYPE_SINT32: Field__Kind = Field__Kind(17u32);
    #[var(18u32, "TYPE_SINT64")]
    pub const TYPE_SINT64: Field__Kind = Field__Kind(18u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field__Cardinality(pub u32);
#[protoenum]
impl Field__Cardinality {
    #[var(0u32, "CARDINALITY_UNKNOWN")]
    pub const CARDINALITY_UNKNOWN: Field__Cardinality = Field__Cardinality(0u32);
    #[var(1u32, "CARDINALITY_OPTIONAL")]
    pub const CARDINALITY_OPTIONAL: Field__Cardinality = Field__Cardinality(1u32);
    #[var(2u32, "CARDINALITY_REQUIRED")]
    pub const CARDINALITY_REQUIRED: Field__Cardinality = Field__Cardinality(2u32);
    #[var(3u32, "CARDINALITY_REPEATED")]
    pub const CARDINALITY_REPEATED: Field__Cardinality = Field__Cardinality(3u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Syntax(pub u32);
#[protoenum]
impl Syntax {
    #[var(0u32, "SYNTAX_PROTO2")]
    pub const SYNTAX_PROTO2: Syntax = Syntax(0u32);
    #[var(1u32, "SYNTAX_PROTO3")]
    pub const SYNTAX_PROTO3: Syntax = Syntax(1u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Type {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "fields", nested, repeated)]
    pub fields: Vec<Field>,
    #[field(3u32, "oneofs", string, repeated)]
    pub oneofs: Vec<String>,
    #[field(4u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[field(5u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(6u32, "syntax", protoenum, singular)]
    pub syntax: Syntax,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Field {
    #[field(1u32, "kind", protoenum, singular)]
    pub kind: Field__Kind,
    #[field(2u32, "cardinality", protoenum, singular)]
    pub cardinality: Field__Cardinality,
    #[field(3u32, "number", varint, singular)]
    pub number: i32,
    #[field(4u32, "name", string, singular)]
    pub name: String,
    #[field(6u32, "type_url", string, singular)]
    pub type_url: String,
    #[field(7u32, "oneof_index", varint, singular)]
    pub oneof_index: i32,
    #[field(8u32, "packed", bool, singular)]
    pub packed: bool,
    #[field(9u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[field(10u32, "json_name", string, singular)]
    pub json_name: String,
    #[field(11u32, "default_value", string, singular)]
    pub default_value: String,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Enum {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "enumvalue", nested, repeated)]
    pub enumvalue: Vec<EnumValue>,
    #[field(3u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[field(4u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(5u32, "syntax", protoenum, singular)]
    pub syntax: Syntax,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumValue {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "number", varint, singular)]
    pub number: i32,
    #[field(3u32, "options", nested, repeated)]
    pub options: Vec<ProtoOption>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ProtoOption {
    #[field(1u32, "name", string, singular)]
    pub name: String,
    #[field(2u32, "value", nested, optional)]
    pub value: Option<Box<Any>>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
