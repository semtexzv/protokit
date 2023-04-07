#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::any::*;
use super::source_context::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Syntax(pub u32);
#[protoenum]
impl Syntax {
    #[var(0u32, "SYNTAX_PROTO2")]
    pub const SYNTAX_PROTO2: Syntax = Syntax(0u32);
    #[var(1u32, "SYNTAX_PROTO3")]
    pub const SYNTAX_PROTO3: Syntax = Syntax(1u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kind(pub u32);
#[protoenum]
impl Kind {
    #[var(0u32, "TYPE_UNKNOWN")]
    pub const TYPE_UNKNOWN: Kind = Kind(0u32);
    #[var(1u32, "TYPE_DOUBLE")]
    pub const TYPE_DOUBLE: Kind = Kind(1u32);
    #[var(2u32, "TYPE_FLOAT")]
    pub const TYPE_FLOAT: Kind = Kind(2u32);
    #[var(3u32, "TYPE_INT64")]
    pub const TYPE_INT64: Kind = Kind(3u32);
    #[var(4u32, "TYPE_UINT64")]
    pub const TYPE_UINT64: Kind = Kind(4u32);
    #[var(5u32, "TYPE_INT32")]
    pub const TYPE_INT32: Kind = Kind(5u32);
    #[var(6u32, "TYPE_FIXED64")]
    pub const TYPE_FIXED64: Kind = Kind(6u32);
    #[var(7u32, "TYPE_FIXED32")]
    pub const TYPE_FIXED32: Kind = Kind(7u32);
    #[var(8u32, "TYPE_BOOL")]
    pub const TYPE_BOOL: Kind = Kind(8u32);
    #[var(9u32, "TYPE_STRING")]
    pub const TYPE_STRING: Kind = Kind(9u32);
    #[var(10u32, "TYPE_GROUP")]
    pub const TYPE_GROUP: Kind = Kind(10u32);
    #[var(11u32, "TYPE_MESSAGE")]
    pub const TYPE_MESSAGE: Kind = Kind(11u32);
    #[var(12u32, "TYPE_BYTES")]
    pub const TYPE_BYTES: Kind = Kind(12u32);
    #[var(13u32, "TYPE_UINT32")]
    pub const TYPE_UINT32: Kind = Kind(13u32);
    #[var(14u32, "TYPE_ENUM")]
    pub const TYPE_ENUM: Kind = Kind(14u32);
    #[var(15u32, "TYPE_SFIXED32")]
    pub const TYPE_SFIXED32: Kind = Kind(15u32);
    #[var(16u32, "TYPE_SFIXED64")]
    pub const TYPE_SFIXED64: Kind = Kind(16u32);
    #[var(17u32, "TYPE_SINT32")]
    pub const TYPE_SINT32: Kind = Kind(17u32);
    #[var(18u32, "TYPE_SINT64")]
    pub const TYPE_SINT64: Kind = Kind(18u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cardinality(pub u32);
#[protoenum]
impl Cardinality {
    #[var(0u32, "CARDINALITY_UNKNOWN")]
    pub const CARDINALITY_UNKNOWN: Cardinality = Cardinality(0u32);
    #[var(1u32, "CARDINALITY_OPTIONAL")]
    pub const CARDINALITY_OPTIONAL: Cardinality = Cardinality(1u32);
    #[var(2u32, "CARDINALITY_REQUIRED")]
    pub const CARDINALITY_REQUIRED: Cardinality = Cardinality(2u32);
    #[var(3u32, "CARDINALITY_REPEATED")]
    pub const CARDINALITY_REPEATED: Cardinality = Cardinality(3u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Type {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "fields", nested, packed)]
    pub fields: Vec<Field>,
    #[field(3u32, "oneofs", string, packed)]
    pub oneofs: Vec<String>,
    #[field(4u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[field(5u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(6u32, "syntax", protoenum, optional)]
    pub syntax: Option<Syntax>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Field {
    #[field(1u32, "kind", protoenum, optional)]
    pub kind: Option<Kind>,
    #[field(2u32, "cardinality", protoenum, optional)]
    pub cardinality: Option<Cardinality>,
    #[field(3u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(4u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(6u32, "type_url", string, optional)]
    pub type_url: Option<String>,
    #[field(7u32, "oneof_index", varint, optional)]
    pub oneof_index: Option<i32>,
    #[field(8u32, "packed", bool, optional)]
    pub packed: Option<bool>,
    #[field(9u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[field(10u32, "json_name", string, optional)]
    pub json_name: Option<String>,
    #[field(11u32, "default_value", string, optional)]
    pub default_value: Option<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Enum {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "enumvalue", nested, packed)]
    pub enumvalue: Vec<EnumValue>,
    #[field(3u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[field(4u32, "source_context", nested, optional)]
    pub source_context: Option<Box<SourceContext>>,
    #[field(5u32, "syntax", protoenum, optional)]
    pub syntax: Option<Syntax>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumValue {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "number", varint, optional)]
    pub number: Option<i32>,
    #[field(3u32, "options", nested, packed)]
    pub options: Vec<ProtoOption>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ProtoOption {
    #[field(1u32, "name", string, optional)]
    pub name: Option<String>,
    #[field(2u32, "value", nested, optional)]
    pub value: Option<Box<Any>>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
