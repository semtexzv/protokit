#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::super::super::google::protobuf::any::*;
use super::super::super::google::protobuf::duration::*;
use super::super::super::google::protobuf::field_mask::*;
use super::super::super::google::protobuf::r#struct::*;
use super::super::super::google::protobuf::timestamp::*;
use super::super::super::google::protobuf::wrappers::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestAllTypesProto3NestedEnum(pub u32);
#[protoenum]
impl TestAllTypesProto3NestedEnum {
    #[var(0u32, "FOO")]
    pub const FOO: TestAllTypesProto3NestedEnum = TestAllTypesProto3NestedEnum(0u32);
    #[var(1u32, "BAR")]
    pub const BAR: TestAllTypesProto3NestedEnum = TestAllTypesProto3NestedEnum(1u32);
    #[var(2u32, "BAZ")]
    pub const BAZ: TestAllTypesProto3NestedEnum = TestAllTypesProto3NestedEnum(2u32);
    #[var(4294967295u32, "NEG")]
    pub const NEG: TestAllTypesProto3NestedEnum = TestAllTypesProto3NestedEnum(
        4294967295u32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestAllTypesProto3AliasedEnum(pub u32);
#[protoenum]
impl TestAllTypesProto3AliasedEnum {
    #[var(0u32, "ALIAS_FOO")]
    pub const ALIAS_FOO: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(
        0u32,
    );
    #[var(1u32, "ALIAS_BAR")]
    pub const ALIAS_BAR: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(
        1u32,
    );
    #[var(2u32, "ALIAS_BAZ")]
    pub const ALIAS_BAZ: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(
        2u32,
    );
    #[var(2u32, "MOO")]
    pub const MOO: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(2u32);
    #[var(2u32, "moo")]
    pub const moo: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(2u32);
    #[var(2u32, "bAz")]
    pub const bAz: TestAllTypesProto3AliasedEnum = TestAllTypesProto3AliasedEnum(2u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForeignEnum(pub u32);
#[protoenum]
impl ForeignEnum {
    #[var(0u32, "FOREIGN_FOO")]
    pub const FOREIGN_FOO: ForeignEnum = ForeignEnum(0u32);
    #[var(1u32, "FOREIGN_BAR")]
    pub const FOREIGN_BAR: ForeignEnum = ForeignEnum(1u32);
    #[var(2u32, "FOREIGN_BAZ")]
    pub const FOREIGN_BAZ: ForeignEnum = ForeignEnum(2u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumOnlyProto3Bool(pub u32);
#[protoenum]
impl EnumOnlyProto3Bool {
    #[var(0u32, "kFalse")]
    pub const kFalse: EnumOnlyProto3Bool = EnumOnlyProto3Bool(0u32);
    #[var(1u32, "kTrue")]
    pub const kTrue: EnumOnlyProto3Bool = EnumOnlyProto3Bool(1u32);
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum TestAllTypesProto3OneOfOneofField {
    #[field(111u32, "oneof_uint32", varint, raw)]
    OneofUint32(u32),
    #[field(112u32, "oneof_nested_message", nested, raw)]
    OneofNestedMessage(TestAllTypesProto3NestedMessage),
    #[field(113u32, "oneof_string", string, raw)]
    OneofString(String),
    #[field(114u32, "oneof_bytes", bytes, raw)]
    OneofBytes(Vec<u8>),
    #[field(115u32, "oneof_bool", bool, raw)]
    OneofBool(bool),
    #[field(116u32, "oneof_uint64", varint, raw)]
    OneofUint64(u64),
    #[field(117u32, "oneof_float", fixed32, raw)]
    OneofFloat(f32),
    #[field(118u32, "oneof_double", fixed64, raw)]
    OneofDouble(f64),
    #[field(119u32, "oneof_enum", protoenum, raw)]
    OneofEnum(TestAllTypesProto3NestedEnum),
    #[field(120u32, "oneof_null_value", protoenum, raw)]
    OneofNullValue(NullValue),
}
impl Default for TestAllTypesProto3OneOfOneofField {
    fn default() -> Self {
        Self::OneofUint32(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto3 {
    #[field(1u32, "optional_int32", varint, singular)]
    pub optional_int32: i32,
    #[field(2u32, "optional_int64", varint, singular)]
    pub optional_int64: i64,
    #[field(3u32, "optional_uint32", varint, singular)]
    pub optional_uint32: u32,
    #[field(4u32, "optional_uint64", varint, singular)]
    pub optional_uint64: u64,
    #[field(5u32, "optional_sint32", sigint, singular)]
    pub optional_sint32: i32,
    #[field(6u32, "optional_sint64", sigint, singular)]
    pub optional_sint64: i64,
    #[field(7u32, "optional_fixed32", fixed32, singular)]
    pub optional_fixed32: u32,
    #[field(8u32, "optional_fixed64", fixed64, singular)]
    pub optional_fixed64: u64,
    #[field(9u32, "optional_sfixed32", fixed32, singular)]
    pub optional_sfixed32: i32,
    #[field(10u32, "optional_sfixed64", fixed64, singular)]
    pub optional_sfixed64: i64,
    #[field(11u32, "optional_float", fixed32, singular)]
    pub optional_float: f32,
    #[field(12u32, "optional_double", fixed64, singular)]
    pub optional_double: f64,
    #[field(13u32, "optional_bool", bool, singular)]
    pub optional_bool: bool,
    #[field(14u32, "optional_string", string, singular)]
    pub optional_string: String,
    #[field(15u32, "optional_bytes", bytes, singular)]
    pub optional_bytes: Vec<u8>,
    #[field(18u32, "optional_nested_message", nested, optional)]
    pub optional_nested_message: Option<Box<TestAllTypesProto3NestedMessage>>,
    #[field(19u32, "optional_foreign_message", nested, optional)]
    pub optional_foreign_message: Option<Box<ForeignMessage>>,
    #[field(21u32, "optional_nested_enum", protoenum, singular)]
    pub optional_nested_enum: TestAllTypesProto3NestedEnum,
    #[field(22u32, "optional_foreign_enum", protoenum, singular)]
    pub optional_foreign_enum: ForeignEnum,
    #[field(23u32, "optional_aliased_enum", protoenum, singular)]
    pub optional_aliased_enum: TestAllTypesProto3AliasedEnum,
    #[field(24u32, "optional_string_piece", string, singular)]
    pub optional_string_piece: String,
    #[field(25u32, "optional_cord", string, singular)]
    pub optional_cord: String,
    #[field(27u32, "recursive_message", nested, optional)]
    pub recursive_message: Option<Box<TestAllTypesProto3>>,
    #[field(31u32, "repeated_int32", varint, packed)]
    pub repeated_int32: Vec<i32>,
    #[field(32u32, "repeated_int64", varint, packed)]
    pub repeated_int64: Vec<i64>,
    #[field(33u32, "repeated_uint32", varint, packed)]
    pub repeated_uint32: Vec<u32>,
    #[field(34u32, "repeated_uint64", varint, packed)]
    pub repeated_uint64: Vec<u64>,
    #[field(35u32, "repeated_sint32", sigint, packed)]
    pub repeated_sint32: Vec<i32>,
    #[field(36u32, "repeated_sint64", sigint, packed)]
    pub repeated_sint64: Vec<i64>,
    #[field(37u32, "repeated_fixed32", fixed32, packed)]
    pub repeated_fixed32: Vec<u32>,
    #[field(38u32, "repeated_fixed64", fixed64, packed)]
    pub repeated_fixed64: Vec<u64>,
    #[field(39u32, "repeated_sfixed32", fixed32, packed)]
    pub repeated_sfixed32: Vec<i32>,
    #[field(40u32, "repeated_sfixed64", fixed64, packed)]
    pub repeated_sfixed64: Vec<i64>,
    #[field(41u32, "repeated_float", fixed32, packed)]
    pub repeated_float: Vec<f32>,
    #[field(42u32, "repeated_double", fixed64, packed)]
    pub repeated_double: Vec<f64>,
    #[field(43u32, "repeated_bool", bool, packed)]
    pub repeated_bool: Vec<bool>,
    #[field(44u32, "repeated_string", string, repeated)]
    pub repeated_string: Vec<String>,
    #[field(45u32, "repeated_bytes", bytes, repeated)]
    pub repeated_bytes: Vec<Vec<u8>>,
    #[field(48u32, "repeated_nested_message", nested, repeated)]
    pub repeated_nested_message: Vec<TestAllTypesProto3NestedMessage>,
    #[field(49u32, "repeated_foreign_message", nested, repeated)]
    pub repeated_foreign_message: Vec<ForeignMessage>,
    #[field(51u32, "repeated_nested_enum", protoenum, packed)]
    pub repeated_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
    #[field(52u32, "repeated_foreign_enum", protoenum, packed)]
    pub repeated_foreign_enum: Vec<ForeignEnum>,
    #[field(54u32, "repeated_string_piece", string, repeated)]
    pub repeated_string_piece: Vec<String>,
    #[field(55u32, "repeated_cord", string, repeated)]
    pub repeated_cord: Vec<String>,
    #[field(75u32, "packed_int32", varint, packed)]
    pub packed_int32: Vec<i32>,
    #[field(76u32, "packed_int64", varint, packed)]
    pub packed_int64: Vec<i64>,
    #[field(77u32, "packed_uint32", varint, packed)]
    pub packed_uint32: Vec<u32>,
    #[field(78u32, "packed_uint64", varint, packed)]
    pub packed_uint64: Vec<u64>,
    #[field(79u32, "packed_sint32", sigint, packed)]
    pub packed_sint32: Vec<i32>,
    #[field(80u32, "packed_sint64", sigint, packed)]
    pub packed_sint64: Vec<i64>,
    #[field(81u32, "packed_fixed32", fixed32, packed)]
    pub packed_fixed32: Vec<u32>,
    #[field(82u32, "packed_fixed64", fixed64, packed)]
    pub packed_fixed64: Vec<u64>,
    #[field(83u32, "packed_sfixed32", fixed32, packed)]
    pub packed_sfixed32: Vec<i32>,
    #[field(84u32, "packed_sfixed64", fixed64, packed)]
    pub packed_sfixed64: Vec<i64>,
    #[field(85u32, "packed_float", fixed32, packed)]
    pub packed_float: Vec<f32>,
    #[field(86u32, "packed_double", fixed64, packed)]
    pub packed_double: Vec<f64>,
    #[field(87u32, "packed_bool", bool, packed)]
    pub packed_bool: Vec<bool>,
    #[field(88u32, "packed_nested_enum", protoenum, packed)]
    pub packed_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
    #[field(89u32, "unpacked_int32", varint, packed)]
    pub unpacked_int32: Vec<i32>,
    #[field(90u32, "unpacked_int64", varint, packed)]
    pub unpacked_int64: Vec<i64>,
    #[field(91u32, "unpacked_uint32", varint, packed)]
    pub unpacked_uint32: Vec<u32>,
    #[field(92u32, "unpacked_uint64", varint, packed)]
    pub unpacked_uint64: Vec<u64>,
    #[field(93u32, "unpacked_sint32", sigint, packed)]
    pub unpacked_sint32: Vec<i32>,
    #[field(94u32, "unpacked_sint64", sigint, packed)]
    pub unpacked_sint64: Vec<i64>,
    #[field(95u32, "unpacked_fixed32", fixed32, packed)]
    pub unpacked_fixed32: Vec<u32>,
    #[field(96u32, "unpacked_fixed64", fixed64, packed)]
    pub unpacked_fixed64: Vec<u64>,
    #[field(97u32, "unpacked_sfixed32", fixed32, packed)]
    pub unpacked_sfixed32: Vec<i32>,
    #[field(98u32, "unpacked_sfixed64", fixed64, packed)]
    pub unpacked_sfixed64: Vec<i64>,
    #[field(99u32, "unpacked_float", fixed32, packed)]
    pub unpacked_float: Vec<f32>,
    #[field(100u32, "unpacked_double", fixed64, packed)]
    pub unpacked_double: Vec<f64>,
    #[field(101u32, "unpacked_bool", bool, packed)]
    pub unpacked_bool: Vec<bool>,
    #[field(102u32, "unpacked_nested_enum", protoenum, packed)]
    pub unpacked_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
    #[field(56u32, "map_int32_int32", map(varint, varint), singular)]
    pub map_int32_int32: ::std::collections::BTreeMap<i32, i32>,
    #[field(57u32, "map_int64_int64", map(varint, varint), singular)]
    pub map_int64_int64: ::std::collections::BTreeMap<i64, i64>,
    #[field(58u32, "map_uint32_uint32", map(varint, varint), singular)]
    pub map_uint32_uint32: ::std::collections::BTreeMap<u32, u32>,
    #[field(59u32, "map_uint64_uint64", map(varint, varint), singular)]
    pub map_uint64_uint64: ::std::collections::BTreeMap<u64, u64>,
    #[field(60u32, "map_sint32_sint32", map(sigint, sigint), singular)]
    pub map_sint32_sint32: ::std::collections::BTreeMap<i32, i32>,
    #[field(61u32, "map_sint64_sint64", map(sigint, sigint), singular)]
    pub map_sint64_sint64: ::std::collections::BTreeMap<i64, i64>,
    #[field(62u32, "map_fixed32_fixed32", map(fixed32, fixed32), singular)]
    pub map_fixed32_fixed32: ::std::collections::BTreeMap<u32, u32>,
    #[field(63u32, "map_fixed64_fixed64", map(fixed64, fixed64), singular)]
    pub map_fixed64_fixed64: ::std::collections::BTreeMap<u64, u64>,
    #[field(64u32, "map_sfixed32_sfixed32", map(fixed32, fixed32), singular)]
    pub map_sfixed32_sfixed32: ::std::collections::BTreeMap<i32, i32>,
    #[field(65u32, "map_sfixed64_sfixed64", map(fixed64, fixed64), singular)]
    pub map_sfixed64_sfixed64: ::std::collections::BTreeMap<i64, i64>,
    #[field(66u32, "map_int32_float", map(varint, fixed32), singular)]
    pub map_int32_float: ::std::collections::BTreeMap<i32, f32>,
    #[field(67u32, "map_int32_double", map(varint, fixed64), singular)]
    pub map_int32_double: ::std::collections::BTreeMap<i32, f64>,
    #[field(68u32, "map_bool_bool", map(bool, bool), singular)]
    pub map_bool_bool: ::std::collections::BTreeMap<bool, bool>,
    #[field(69u32, "map_string_string", map(string, string), singular)]
    pub map_string_string: ::std::collections::BTreeMap<String, String>,
    #[field(70u32, "map_string_bytes", map(string, bytes), singular)]
    pub map_string_bytes: ::std::collections::BTreeMap<String, Vec<u8>>,
    #[field(71u32, "map_string_nested_message", map(string, nested), singular)]
    pub map_string_nested_message: ::std::collections::BTreeMap<
        String,
        TestAllTypesProto3NestedMessage,
    >,
    #[field(72u32, "map_string_foreign_message", map(string, nested), singular)]
    pub map_string_foreign_message: ::std::collections::BTreeMap<String, ForeignMessage>,
    #[field(73u32, "map_string_nested_enum", map(string, protoenum), singular)]
    pub map_string_nested_enum: ::std::collections::BTreeMap<
        String,
        TestAllTypesProto3NestedEnum,
    >,
    #[field(74u32, "map_string_foreign_enum", map(string, protoenum), singular)]
    pub map_string_foreign_enum: ::std::collections::BTreeMap<String, ForeignEnum>,
    #[field(201u32, "optional_bool_wrapper", nested, optional)]
    pub optional_bool_wrapper: Option<Box<BoolValue>>,
    #[field(202u32, "optional_int32_wrapper", nested, optional)]
    pub optional_int32_wrapper: Option<Box<Int32Value>>,
    #[field(203u32, "optional_int64_wrapper", nested, optional)]
    pub optional_int64_wrapper: Option<Box<Int64Value>>,
    #[field(204u32, "optional_uint32_wrapper", nested, optional)]
    pub optional_uint32_wrapper: Option<Box<UInt32Value>>,
    #[field(205u32, "optional_uint64_wrapper", nested, optional)]
    pub optional_uint64_wrapper: Option<Box<UInt64Value>>,
    #[field(206u32, "optional_float_wrapper", nested, optional)]
    pub optional_float_wrapper: Option<Box<FloatValue>>,
    #[field(207u32, "optional_double_wrapper", nested, optional)]
    pub optional_double_wrapper: Option<Box<DoubleValue>>,
    #[field(208u32, "optional_string_wrapper", nested, optional)]
    pub optional_string_wrapper: Option<Box<StringValue>>,
    #[field(209u32, "optional_bytes_wrapper", nested, optional)]
    pub optional_bytes_wrapper: Option<Box<BytesValue>>,
    #[field(211u32, "repeated_bool_wrapper", nested, repeated)]
    pub repeated_bool_wrapper: Vec<BoolValue>,
    #[field(212u32, "repeated_int32_wrapper", nested, repeated)]
    pub repeated_int32_wrapper: Vec<Int32Value>,
    #[field(213u32, "repeated_int64_wrapper", nested, repeated)]
    pub repeated_int64_wrapper: Vec<Int64Value>,
    #[field(214u32, "repeated_uint32_wrapper", nested, repeated)]
    pub repeated_uint32_wrapper: Vec<UInt32Value>,
    #[field(215u32, "repeated_uint64_wrapper", nested, repeated)]
    pub repeated_uint64_wrapper: Vec<UInt64Value>,
    #[field(216u32, "repeated_float_wrapper", nested, repeated)]
    pub repeated_float_wrapper: Vec<FloatValue>,
    #[field(217u32, "repeated_double_wrapper", nested, repeated)]
    pub repeated_double_wrapper: Vec<DoubleValue>,
    #[field(218u32, "repeated_string_wrapper", nested, repeated)]
    pub repeated_string_wrapper: Vec<StringValue>,
    #[field(219u32, "repeated_bytes_wrapper", nested, repeated)]
    pub repeated_bytes_wrapper: Vec<BytesValue>,
    #[field(301u32, "optional_duration", nested, optional)]
    pub optional_duration: Option<Box<Duration>>,
    #[field(302u32, "optional_timestamp", nested, optional)]
    pub optional_timestamp: Option<Box<Timestamp>>,
    #[field(303u32, "optional_field_mask", nested, optional)]
    pub optional_field_mask: Option<Box<FieldMask>>,
    #[field(304u32, "optional_struct", nested, optional)]
    pub optional_struct: Option<Box<Struct>>,
    #[field(305u32, "optional_any", nested, optional)]
    pub optional_any: Option<Box<Any>>,
    #[field(306u32, "optional_value", nested, optional)]
    pub optional_value: Option<Box<Value>>,
    #[field(307u32, "optional_null_value", protoenum, singular)]
    pub optional_null_value: NullValue,
    #[field(311u32, "repeated_duration", nested, repeated)]
    pub repeated_duration: Vec<Duration>,
    #[field(312u32, "repeated_timestamp", nested, repeated)]
    pub repeated_timestamp: Vec<Timestamp>,
    #[field(313u32, "repeated_fieldmask", nested, repeated)]
    pub repeated_fieldmask: Vec<FieldMask>,
    #[field(324u32, "repeated_struct", nested, repeated)]
    pub repeated_struct: Vec<Struct>,
    #[field(315u32, "repeated_any", nested, repeated)]
    pub repeated_any: Vec<Any>,
    #[field(316u32, "repeated_value", nested, repeated)]
    pub repeated_value: Vec<Value>,
    #[field(317u32, "repeated_list_value", nested, repeated)]
    pub repeated_list_value: Vec<ListValue>,
    #[field(401u32, "fieldname1", varint, singular)]
    pub fieldname1: i32,
    #[field(402u32, "field_name2", varint, singular)]
    pub field_name2: i32,
    #[field(403u32, "_field_name3", varint, singular)]
    pub _field_name3: i32,
    #[field(404u32, "field__name4_", varint, singular)]
    pub field__name4_: i32,
    #[field(405u32, "field0name5", varint, singular)]
    pub field0name5: i32,
    #[field(406u32, "field_0_name6", varint, singular)]
    pub field_0_name6: i32,
    #[field(407u32, "fieldName7", varint, singular)]
    pub fieldName7: i32,
    #[field(408u32, "FieldName8", varint, singular)]
    pub FieldName8: i32,
    #[field(409u32, "field_Name9", varint, singular)]
    pub field_Name9: i32,
    #[field(410u32, "Field_Name10", varint, singular)]
    pub Field_Name10: i32,
    #[field(411u32, "FIELD_NAME11", varint, singular)]
    pub FIELD_NAME11: i32,
    #[field(412u32, "FIELD_name12", varint, singular)]
    pub FIELD_name12: i32,
    #[field(413u32, "__field_name13", varint, singular)]
    pub __field_name13: i32,
    #[field(414u32, "__Field_name14", varint, singular)]
    pub __Field_name14: i32,
    #[field(415u32, "field__name15", varint, singular)]
    pub field__name15: i32,
    #[field(416u32, "field__Name16", varint, singular)]
    pub field__Name16: i32,
    #[field(417u32, "field_name17__", varint, singular)]
    pub field_name17__: i32,
    #[field(418u32, "Field_name18__", varint, singular)]
    pub Field_name18__: i32,
    #[oneof(
        [111u32,
        112u32,
        113u32,
        114u32,
        115u32,
        116u32,
        117u32,
        118u32,
        119u32,
        120u32,
        ],
        ["oneof_uint32",
        "oneof_nested_message",
        "oneof_string",
        "oneof_bytes",
        "oneof_bool",
        "oneof_uint64",
        "oneof_float",
        "oneof_double",
        "oneof_enum",
        "oneof_null_value",
        ]
    )]
    pub oneof_field: Option<TestAllTypesProto3OneOfOneofField>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto3NestedMessage {
    #[field(1u32, "a", varint, singular)]
    pub a: i32,
    #[field(2u32, "corecursive", nested, optional)]
    pub corecursive: Option<Box<TestAllTypesProto3>>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ForeignMessage {
    #[field(1u32, "c", varint, singular)]
    pub c: i32,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct NullHypothesisProto3 {
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumOnlyProto3 {
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
