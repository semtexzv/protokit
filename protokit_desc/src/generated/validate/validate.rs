use crate::*;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {}
use super::super::google::protobuf::descriptor::*;
use super::super::google::protobuf::duration::*;
use super::super::google::protobuf::timestamp::*;
#[derive(Default, Debug, Clone, Copy)]
pub struct KnownRegex(pub u32);
#[protoenum]
impl KnownRegex {
    #[var(0u32, "UNKNOWN")]
    pub const UNKNOWN: KnownRegex = KnownRegex(0u32);
    #[var(1u32, "HTTP_HEADER_NAME")]
    pub const HTTP_HEADER_NAME: KnownRegex = KnownRegex(1u32);
    #[var(2u32, "HTTP_HEADER_VALUE")]
    pub const HTTP_HEADER_VALUE: KnownRegex = KnownRegex(2u32);
}
#[derive(Debug, Clone, Proto)]
pub enum FieldRulesOneOfType {
    #[field(1u32, "float", nested, singular)]
    Float(FloatRules),
    #[field(2u32, "double", nested, singular)]
    Double(DoubleRules),
    #[field(3u32, "int32", nested, singular)]
    Int32(Int32Rules),
    #[field(4u32, "int64", nested, singular)]
    Int64(Int64Rules),
    #[field(5u32, "uint32", nested, singular)]
    Uint32(UInt32Rules),
    #[field(6u32, "uint64", nested, singular)]
    Uint64(UInt64Rules),
    #[field(7u32, "sint32", nested, singular)]
    Sint32(SInt32Rules),
    #[field(8u32, "sint64", nested, singular)]
    Sint64(SInt64Rules),
    #[field(9u32, "fixed32", nested, singular)]
    Fixed32(Fixed32Rules),
    #[field(10u32, "fixed64", nested, singular)]
    Fixed64(Fixed64Rules),
    #[field(11u32, "sfixed32", nested, singular)]
    Sfixed32(SFixed32Rules),
    #[field(12u32, "sfixed64", nested, singular)]
    Sfixed64(SFixed64Rules),
    #[field(13u32, "bool", nested, singular)]
    Bool(BoolRules),
    #[field(14u32, "string", nested, singular)]
    String(StringRules),
    #[field(15u32, "bytes", nested, singular)]
    Bytes(BytesRules),
    #[field(16u32, "enum", nested, singular)]
    Enum(EnumRules),
    #[field(18u32, "repeated", nested, singular)]
    Repeated(RepeatedRules),
    #[field(19u32, "map", nested, singular)]
    Map(MapRules),
    #[field(20u32, "any", nested, singular)]
    Any(AnyRules),
    #[field(21u32, "duration", nested, singular)]
    Duration(DurationRules),
    #[field(22u32, "timestamp", nested, singular)]
    Timestamp(TimestampRules),
}
impl Default for FieldRulesOneOfType {
    fn default() -> Self {
        Self::Float(Default::default())
    }
}
#[derive(Debug, Default, Clone, Proto)]
pub struct FieldRules {
    #[field(17u32, "message", nested, optional)]
    pub message: Option<Box<MessageRules>>,
    #[oneof(
        [1u32,
        2u32,
        3u32,
        4u32,
        5u32,
        6u32,
        7u32,
        8u32,
        9u32,
        10u32,
        11u32,
        12u32,
        13u32,
        14u32,
        15u32,
        16u32,
        18u32,
        19u32,
        20u32,
        21u32,
        22u32,
        ],
        ["float",
        "double",
        "int32",
        "int64",
        "uint32",
        "uint64",
        "sint32",
        "sint64",
        "fixed32",
        "fixed64",
        "sfixed32",
        "sfixed64",
        "bool",
        "string",
        "bytes",
        "enum",
        "repeated",
        "map",
        "any",
        "duration",
        "timestamp",
        ]
    )]
    pub r#type: Option<FieldRulesOneOfType>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct FloatRules {
    #[field(1u32, "const", fixed32, optional)]
    pub r#const: Option<f32>,
    #[field(2u32, "lt", fixed32, optional)]
    pub lt: Option<f32>,
    #[field(3u32, "lte", fixed32, optional)]
    pub lte: Option<f32>,
    #[field(4u32, "gt", fixed32, optional)]
    pub gt: Option<f32>,
    #[field(5u32, "gte", fixed32, optional)]
    pub gte: Option<f32>,
    #[field(6u32, "in", fixed32, repeated)]
    pub r#in: Vec<f32>,
    #[field(7u32, "not_in", fixed32, repeated)]
    pub not_in: Vec<f32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct DoubleRules {
    #[field(1u32, "const", fixed64, optional)]
    pub r#const: Option<f64>,
    #[field(2u32, "lt", fixed64, optional)]
    pub lt: Option<f64>,
    #[field(3u32, "lte", fixed64, optional)]
    pub lte: Option<f64>,
    #[field(4u32, "gt", fixed64, optional)]
    pub gt: Option<f64>,
    #[field(5u32, "gte", fixed64, optional)]
    pub gte: Option<f64>,
    #[field(6u32, "in", fixed64, repeated)]
    pub r#in: Vec<f64>,
    #[field(7u32, "not_in", fixed64, repeated)]
    pub not_in: Vec<f64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct Int32Rules {
    #[field(1u32, "const", varint, optional)]
    pub r#const: Option<i32>,
    #[field(2u32, "lt", varint, optional)]
    pub lt: Option<i32>,
    #[field(3u32, "lte", varint, optional)]
    pub lte: Option<i32>,
    #[field(4u32, "gt", varint, optional)]
    pub gt: Option<i32>,
    #[field(5u32, "gte", varint, optional)]
    pub gte: Option<i32>,
    #[field(6u32, "in", varint, repeated)]
    pub r#in: Vec<i32>,
    #[field(7u32, "not_in", varint, repeated)]
    pub not_in: Vec<i32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct Int64Rules {
    #[field(1u32, "const", varint, optional)]
    pub r#const: Option<i64>,
    #[field(2u32, "lt", varint, optional)]
    pub lt: Option<i64>,
    #[field(3u32, "lte", varint, optional)]
    pub lte: Option<i64>,
    #[field(4u32, "gt", varint, optional)]
    pub gt: Option<i64>,
    #[field(5u32, "gte", varint, optional)]
    pub gte: Option<i64>,
    #[field(6u32, "in", varint, repeated)]
    pub r#in: Vec<i64>,
    #[field(7u32, "not_in", varint, repeated)]
    pub not_in: Vec<i64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct UInt32Rules {
    #[field(1u32, "const", varint, optional)]
    pub r#const: Option<u32>,
    #[field(2u32, "lt", varint, optional)]
    pub lt: Option<u32>,
    #[field(3u32, "lte", varint, optional)]
    pub lte: Option<u32>,
    #[field(4u32, "gt", varint, optional)]
    pub gt: Option<u32>,
    #[field(5u32, "gte", varint, optional)]
    pub gte: Option<u32>,
    #[field(6u32, "in", varint, repeated)]
    pub r#in: Vec<u32>,
    #[field(7u32, "not_in", varint, repeated)]
    pub not_in: Vec<u32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct UInt64Rules {
    #[field(1u32, "const", varint, optional)]
    pub r#const: Option<u64>,
    #[field(2u32, "lt", varint, optional)]
    pub lt: Option<u64>,
    #[field(3u32, "lte", varint, optional)]
    pub lte: Option<u64>,
    #[field(4u32, "gt", varint, optional)]
    pub gt: Option<u64>,
    #[field(5u32, "gte", varint, optional)]
    pub gte: Option<u64>,
    #[field(6u32, "in", varint, repeated)]
    pub r#in: Vec<u64>,
    #[field(7u32, "not_in", varint, repeated)]
    pub not_in: Vec<u64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct SInt32Rules {
    #[field(1u32, "const", sigint, optional)]
    pub r#const: Option<i32>,
    #[field(2u32, "lt", sigint, optional)]
    pub lt: Option<i32>,
    #[field(3u32, "lte", sigint, optional)]
    pub lte: Option<i32>,
    #[field(4u32, "gt", sigint, optional)]
    pub gt: Option<i32>,
    #[field(5u32, "gte", sigint, optional)]
    pub gte: Option<i32>,
    #[field(6u32, "in", sigint, repeated)]
    pub r#in: Vec<i32>,
    #[field(7u32, "not_in", sigint, repeated)]
    pub not_in: Vec<i32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct SInt64Rules {
    #[field(1u32, "const", sigint, optional)]
    pub r#const: Option<i64>,
    #[field(2u32, "lt", sigint, optional)]
    pub lt: Option<i64>,
    #[field(3u32, "lte", sigint, optional)]
    pub lte: Option<i64>,
    #[field(4u32, "gt", sigint, optional)]
    pub gt: Option<i64>,
    #[field(5u32, "gte", sigint, optional)]
    pub gte: Option<i64>,
    #[field(6u32, "in", sigint, repeated)]
    pub r#in: Vec<i64>,
    #[field(7u32, "not_in", sigint, repeated)]
    pub not_in: Vec<i64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct Fixed32Rules {
    #[field(1u32, "const", fixed32, optional)]
    pub r#const: Option<u32>,
    #[field(2u32, "lt", fixed32, optional)]
    pub lt: Option<u32>,
    #[field(3u32, "lte", fixed32, optional)]
    pub lte: Option<u32>,
    #[field(4u32, "gt", fixed32, optional)]
    pub gt: Option<u32>,
    #[field(5u32, "gte", fixed32, optional)]
    pub gte: Option<u32>,
    #[field(6u32, "in", fixed32, repeated)]
    pub r#in: Vec<u32>,
    #[field(7u32, "not_in", fixed32, repeated)]
    pub not_in: Vec<u32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct Fixed64Rules {
    #[field(1u32, "const", fixed64, optional)]
    pub r#const: Option<u64>,
    #[field(2u32, "lt", fixed64, optional)]
    pub lt: Option<u64>,
    #[field(3u32, "lte", fixed64, optional)]
    pub lte: Option<u64>,
    #[field(4u32, "gt", fixed64, optional)]
    pub gt: Option<u64>,
    #[field(5u32, "gte", fixed64, optional)]
    pub gte: Option<u64>,
    #[field(6u32, "in", fixed64, repeated)]
    pub r#in: Vec<u64>,
    #[field(7u32, "not_in", fixed64, repeated)]
    pub not_in: Vec<u64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct SFixed32Rules {
    #[field(1u32, "const", fixed32, optional)]
    pub r#const: Option<i32>,
    #[field(2u32, "lt", fixed32, optional)]
    pub lt: Option<i32>,
    #[field(3u32, "lte", fixed32, optional)]
    pub lte: Option<i32>,
    #[field(4u32, "gt", fixed32, optional)]
    pub gt: Option<i32>,
    #[field(5u32, "gte", fixed32, optional)]
    pub gte: Option<i32>,
    #[field(6u32, "in", fixed32, repeated)]
    pub r#in: Vec<i32>,
    #[field(7u32, "not_in", fixed32, repeated)]
    pub not_in: Vec<i32>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct SFixed64Rules {
    #[field(1u32, "const", fixed64, optional)]
    pub r#const: Option<i64>,
    #[field(2u32, "lt", fixed64, optional)]
    pub lt: Option<i64>,
    #[field(3u32, "lte", fixed64, optional)]
    pub lte: Option<i64>,
    #[field(4u32, "gt", fixed64, optional)]
    pub gt: Option<i64>,
    #[field(5u32, "gte", fixed64, optional)]
    pub gte: Option<i64>,
    #[field(6u32, "in", fixed64, repeated)]
    pub r#in: Vec<i64>,
    #[field(7u32, "not_in", fixed64, repeated)]
    pub not_in: Vec<i64>,
    #[field(8u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct BoolRules {
    #[field(1u32, "const", bool, optional)]
    pub r#const: Option<bool>,
}
#[derive(Debug, Clone, Proto)]
pub enum StringRulesOneOfWellKnown {
    #[field(12u32, "email", bool, singular)]
    Email(bool),
    #[field(13u32, "hostname", bool, singular)]
    Hostname(bool),
    #[field(14u32, "ip", bool, singular)]
    Ip(bool),
    #[field(15u32, "ipv4", bool, singular)]
    Ipv4(bool),
    #[field(16u32, "ipv6", bool, singular)]
    Ipv6(bool),
    #[field(17u32, "uri", bool, singular)]
    Uri(bool),
    #[field(18u32, "uri_ref", bool, singular)]
    UriRef(bool),
    #[field(21u32, "address", bool, singular)]
    Address(bool),
    #[field(22u32, "uuid", bool, singular)]
    Uuid(bool),
    #[field(24u32, "well_known_regex", protoenum, singular)]
    WellKnownRegex(KnownRegex),
}
impl Default for StringRulesOneOfWellKnown {
    fn default() -> Self {
        Self::Email(Default::default())
    }
}
#[derive(Debug, Default, Clone, Proto)]
pub struct StringRules {
    #[field(1u32, "const", string, optional)]
    pub r#const: Option<String>,
    #[field(19u32, "len", varint, optional)]
    pub len: Option<u64>,
    #[field(2u32, "min_len", varint, optional)]
    pub min_len: Option<u64>,
    #[field(3u32, "max_len", varint, optional)]
    pub max_len: Option<u64>,
    #[field(20u32, "len_bytes", varint, optional)]
    pub len_bytes: Option<u64>,
    #[field(4u32, "min_bytes", varint, optional)]
    pub min_bytes: Option<u64>,
    #[field(5u32, "max_bytes", varint, optional)]
    pub max_bytes: Option<u64>,
    #[field(6u32, "pattern", string, optional)]
    pub pattern: Option<String>,
    #[field(7u32, "prefix", string, optional)]
    pub prefix: Option<String>,
    #[field(8u32, "suffix", string, optional)]
    pub suffix: Option<String>,
    #[field(9u32, "contains", string, optional)]
    pub contains: Option<String>,
    #[field(23u32, "not_contains", string, optional)]
    pub not_contains: Option<String>,
    #[field(10u32, "in", string, repeated)]
    pub r#in: Vec<String>,
    #[field(11u32, "not_in", string, repeated)]
    pub not_in: Vec<String>,
    #[field(25u32, "strict", bool, optional)]
    pub strict: Option<bool>,
    #[field(26u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
    #[oneof(
        [12u32,
        13u32,
        14u32,
        15u32,
        16u32,
        17u32,
        18u32,
        21u32,
        22u32,
        24u32,
        ],
        ["email",
        "hostname",
        "ip",
        "ipv4",
        "ipv6",
        "uri",
        "uri_ref",
        "address",
        "uuid",
        "well_known_regex",
        ]
    )]
    pub well_known: Option<StringRulesOneOfWellKnown>,
}
#[derive(Debug, Clone, Proto)]
pub enum BytesRulesOneOfWellKnown {
    #[field(10u32, "ip", bool, singular)]
    Ip(bool),
    #[field(11u32, "ipv4", bool, singular)]
    Ipv4(bool),
    #[field(12u32, "ipv6", bool, singular)]
    Ipv6(bool),
}
impl Default for BytesRulesOneOfWellKnown {
    fn default() -> Self {
        Self::Ip(Default::default())
    }
}
#[derive(Debug, Default, Clone, Proto)]
pub struct BytesRules {
    #[field(1u32, "const", bytes, optional)]
    pub r#const: Option<Vec<u8>>,
    #[field(13u32, "len", varint, optional)]
    pub len: Option<u64>,
    #[field(2u32, "min_len", varint, optional)]
    pub min_len: Option<u64>,
    #[field(3u32, "max_len", varint, optional)]
    pub max_len: Option<u64>,
    #[field(4u32, "pattern", string, optional)]
    pub pattern: Option<String>,
    #[field(5u32, "prefix", bytes, optional)]
    pub prefix: Option<Vec<u8>>,
    #[field(6u32, "suffix", bytes, optional)]
    pub suffix: Option<Vec<u8>>,
    #[field(7u32, "contains", bytes, optional)]
    pub contains: Option<Vec<u8>>,
    #[field(8u32, "in", bytes, repeated)]
    pub r#in: Vec<Vec<u8>>,
    #[field(9u32, "not_in", bytes, repeated)]
    pub not_in: Vec<Vec<u8>>,
    #[field(14u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
    #[oneof([10u32, 11u32, 12u32], ["ip", "ipv4", "ipv6"])]
    pub well_known: Option<BytesRulesOneOfWellKnown>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct EnumRules {
    #[field(1u32, "const", varint, optional)]
    pub r#const: Option<i32>,
    #[field(2u32, "defined_only", bool, optional)]
    pub defined_only: Option<bool>,
    #[field(3u32, "in", varint, repeated)]
    pub r#in: Vec<i32>,
    #[field(4u32, "not_in", varint, repeated)]
    pub not_in: Vec<i32>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct MessageRules {
    #[field(1u32, "skip", bool, optional)]
    pub skip: Option<bool>,
    #[field(2u32, "required", bool, optional)]
    pub required: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct RepeatedRules {
    #[field(1u32, "min_items", varint, optional)]
    pub min_items: Option<u64>,
    #[field(2u32, "max_items", varint, optional)]
    pub max_items: Option<u64>,
    #[field(3u32, "unique", bool, optional)]
    pub unique: Option<bool>,
    #[field(4u32, "items", nested, optional)]
    pub items: Option<Box<FieldRules>>,
    #[field(5u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct MapRules {
    #[field(1u32, "min_pairs", varint, optional)]
    pub min_pairs: Option<u64>,
    #[field(2u32, "max_pairs", varint, optional)]
    pub max_pairs: Option<u64>,
    #[field(3u32, "no_sparse", bool, optional)]
    pub no_sparse: Option<bool>,
    #[field(4u32, "keys", nested, optional)]
    pub keys: Option<Box<FieldRules>>,
    #[field(5u32, "values", nested, optional)]
    pub values: Option<Box<FieldRules>>,
    #[field(6u32, "ignore_empty", bool, optional)]
    pub ignore_empty: Option<bool>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct AnyRules {
    #[field(1u32, "required", bool, optional)]
    pub required: Option<bool>,
    #[field(2u32, "in", string, repeated)]
    pub r#in: Vec<String>,
    #[field(3u32, "not_in", string, repeated)]
    pub not_in: Vec<String>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct DurationRules {
    #[field(1u32, "required", bool, optional)]
    pub required: Option<bool>,
    #[field(2u32, "const", nested, optional)]
    pub r#const: Option<Box<Duration>>,
    #[field(3u32, "lt", nested, optional)]
    pub lt: Option<Box<Duration>>,
    #[field(4u32, "lte", nested, optional)]
    pub lte: Option<Box<Duration>>,
    #[field(5u32, "gt", nested, optional)]
    pub gt: Option<Box<Duration>>,
    #[field(6u32, "gte", nested, optional)]
    pub gte: Option<Box<Duration>>,
    #[field(7u32, "in", nested, repeated)]
    pub r#in: Vec<Duration>,
    #[field(8u32, "not_in", nested, repeated)]
    pub not_in: Vec<Duration>,
}
#[derive(Debug, Default, Clone, Proto)]
pub struct TimestampRules {
    #[field(1u32, "required", bool, optional)]
    pub required: Option<bool>,
    #[field(2u32, "const", nested, optional)]
    pub r#const: Option<Box<Timestamp>>,
    #[field(3u32, "lt", nested, optional)]
    pub lt: Option<Box<Timestamp>>,
    #[field(4u32, "lte", nested, optional)]
    pub lte: Option<Box<Timestamp>>,
    #[field(5u32, "gt", nested, optional)]
    pub gt: Option<Box<Timestamp>>,
    #[field(6u32, "gte", nested, optional)]
    pub gte: Option<Box<Timestamp>>,
    #[field(7u32, "lt_now", bool, optional)]
    pub lt_now: Option<bool>,
    #[field(8u32, "gt_now", bool, optional)]
    pub gt_now: Option<bool>,
    #[field(9u32, "within", nested, optional)]
    pub within: Option<Box<Duration>>,
}
