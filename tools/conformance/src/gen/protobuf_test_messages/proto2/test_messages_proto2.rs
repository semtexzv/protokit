#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestAllTypesProto2NestedEnum(pub u32);
#[protoenum]
impl TestAllTypesProto2NestedEnum {
    #[var(0u32, "FOO")]
    pub const FOO: TestAllTypesProto2NestedEnum = TestAllTypesProto2NestedEnum(0u32);
    #[var(1u32, "BAR")]
    pub const BAR: TestAllTypesProto2NestedEnum = TestAllTypesProto2NestedEnum(1u32);
    #[var(2u32, "BAZ")]
    pub const BAZ: TestAllTypesProto2NestedEnum = TestAllTypesProto2NestedEnum(2u32);
    #[var(4294967295u32, "NEG")]
    pub const NEG: TestAllTypesProto2NestedEnum = TestAllTypesProto2NestedEnum(
        4294967295u32,
    );
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForeignEnumProto2(pub u32);
#[protoenum]
impl ForeignEnumProto2 {
    #[var(0u32, "FOREIGN_FOO")]
    pub const FOREIGN_FOO: ForeignEnumProto2 = ForeignEnumProto2(0u32);
    #[var(1u32, "FOREIGN_BAR")]
    pub const FOREIGN_BAR: ForeignEnumProto2 = ForeignEnumProto2(1u32);
    #[var(2u32, "FOREIGN_BAZ")]
    pub const FOREIGN_BAZ: ForeignEnumProto2 = ForeignEnumProto2(2u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumOnlyProto2Bool(pub u32);
#[protoenum]
impl EnumOnlyProto2Bool {
    #[var(0u32, "kFalse")]
    pub const kFalse: EnumOnlyProto2Bool = EnumOnlyProto2Bool(0u32);
    #[var(1u32, "kTrue")]
    pub const kTrue: EnumOnlyProto2Bool = EnumOnlyProto2Bool(1u32);
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum TestAllTypesProto2OneOfOneofField {
    #[field(111u32, "oneof_uint32", varint, raw)]
    OneofUint32(u32),
    #[field(112u32, "oneof_nested_message", nested, raw)]
    OneofNestedMessage(TestAllTypesProto2NestedMessage),
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
    OneofEnum(TestAllTypesProto2NestedEnum),
}
impl Default for TestAllTypesProto2OneOfOneofField {
    fn default() -> Self {
        Self::OneofUint32(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2 {
    #[field(1u32, "optional_int32", varint, optional)]
    pub optional_int32: Option<i32>,
    #[field(2u32, "optional_int64", varint, optional)]
    pub optional_int64: Option<i64>,
    #[field(3u32, "optional_uint32", varint, optional)]
    pub optional_uint32: Option<u32>,
    #[field(4u32, "optional_uint64", varint, optional)]
    pub optional_uint64: Option<u64>,
    #[field(5u32, "optional_sint32", sigint, optional)]
    pub optional_sint32: Option<i32>,
    #[field(6u32, "optional_sint64", sigint, optional)]
    pub optional_sint64: Option<i64>,
    #[field(7u32, "optional_fixed32", fixed32, optional)]
    pub optional_fixed32: Option<u32>,
    #[field(8u32, "optional_fixed64", fixed64, optional)]
    pub optional_fixed64: Option<u64>,
    #[field(9u32, "optional_sfixed32", fixed32, optional)]
    pub optional_sfixed32: Option<i32>,
    #[field(10u32, "optional_sfixed64", fixed64, optional)]
    pub optional_sfixed64: Option<i64>,
    #[field(11u32, "optional_float", fixed32, optional)]
    pub optional_float: Option<f32>,
    #[field(12u32, "optional_double", fixed64, optional)]
    pub optional_double: Option<f64>,
    #[field(13u32, "optional_bool", bool, optional)]
    pub optional_bool: Option<bool>,
    #[field(14u32, "optional_string", string, optional)]
    pub optional_string: Option<String>,
    #[field(15u32, "optional_bytes", bytes, optional)]
    pub optional_bytes: Option<Vec<u8>>,
    #[field(18u32, "optional_nested_message", nested, optional)]
    pub optional_nested_message: Option<Box<TestAllTypesProto2NestedMessage>>,
    #[field(19u32, "optional_foreign_message", nested, optional)]
    pub optional_foreign_message: Option<Box<ForeignMessageProto2>>,
    #[field(21u32, "optional_nested_enum", protoenum, optional)]
    pub optional_nested_enum: Option<TestAllTypesProto2NestedEnum>,
    #[field(22u32, "optional_foreign_enum", protoenum, optional)]
    pub optional_foreign_enum: Option<ForeignEnumProto2>,
    #[field(24u32, "optional_string_piece", string, optional)]
    pub optional_string_piece: Option<String>,
    #[field(25u32, "optional_cord", string, optional)]
    pub optional_cord: Option<String>,
    #[field(27u32, "recursive_message", nested, optional)]
    pub recursive_message: Option<Box<TestAllTypesProto2>>,
    #[field(31u32, "repeated_int32", varint, repeated)]
    pub repeated_int32: Vec<i32>,
    #[field(32u32, "repeated_int64", varint, repeated)]
    pub repeated_int64: Vec<i64>,
    #[field(33u32, "repeated_uint32", varint, repeated)]
    pub repeated_uint32: Vec<u32>,
    #[field(34u32, "repeated_uint64", varint, repeated)]
    pub repeated_uint64: Vec<u64>,
    #[field(35u32, "repeated_sint32", sigint, repeated)]
    pub repeated_sint32: Vec<i32>,
    #[field(36u32, "repeated_sint64", sigint, repeated)]
    pub repeated_sint64: Vec<i64>,
    #[field(37u32, "repeated_fixed32", fixed32, repeated)]
    pub repeated_fixed32: Vec<u32>,
    #[field(38u32, "repeated_fixed64", fixed64, repeated)]
    pub repeated_fixed64: Vec<u64>,
    #[field(39u32, "repeated_sfixed32", fixed32, repeated)]
    pub repeated_sfixed32: Vec<i32>,
    #[field(40u32, "repeated_sfixed64", fixed64, repeated)]
    pub repeated_sfixed64: Vec<i64>,
    #[field(41u32, "repeated_float", fixed32, repeated)]
    pub repeated_float: Vec<f32>,
    #[field(42u32, "repeated_double", fixed64, repeated)]
    pub repeated_double: Vec<f64>,
    #[field(43u32, "repeated_bool", bool, repeated)]
    pub repeated_bool: Vec<bool>,
    #[field(44u32, "repeated_string", string, repeated)]
    pub repeated_string: Vec<String>,
    #[field(45u32, "repeated_bytes", bytes, repeated)]
    pub repeated_bytes: Vec<Vec<u8>>,
    #[field(48u32, "repeated_nested_message", nested, repeated)]
    pub repeated_nested_message: Vec<TestAllTypesProto2NestedMessage>,
    #[field(49u32, "repeated_foreign_message", nested, repeated)]
    pub repeated_foreign_message: Vec<ForeignMessageProto2>,
    #[field(51u32, "repeated_nested_enum", protoenum, repeated)]
    pub repeated_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
    #[field(52u32, "repeated_foreign_enum", protoenum, repeated)]
    pub repeated_foreign_enum: Vec<ForeignEnumProto2>,
    #[field(54u32, "repeated_string_piece", string, repeated)]
    pub repeated_string_piece: Vec<String>,
    #[field(55u32, "repeated_cord", string, repeated)]
    pub repeated_cord: Vec<String>,
    #[field(75u32, "packed_int32", varint, repeated)]
    pub packed_int32: Vec<i32>,
    #[field(76u32, "packed_int64", varint, repeated)]
    pub packed_int64: Vec<i64>,
    #[field(77u32, "packed_uint32", varint, repeated)]
    pub packed_uint32: Vec<u32>,
    #[field(78u32, "packed_uint64", varint, repeated)]
    pub packed_uint64: Vec<u64>,
    #[field(79u32, "packed_sint32", sigint, repeated)]
    pub packed_sint32: Vec<i32>,
    #[field(80u32, "packed_sint64", sigint, repeated)]
    pub packed_sint64: Vec<i64>,
    #[field(81u32, "packed_fixed32", fixed32, repeated)]
    pub packed_fixed32: Vec<u32>,
    #[field(82u32, "packed_fixed64", fixed64, repeated)]
    pub packed_fixed64: Vec<u64>,
    #[field(83u32, "packed_sfixed32", fixed32, repeated)]
    pub packed_sfixed32: Vec<i32>,
    #[field(84u32, "packed_sfixed64", fixed64, repeated)]
    pub packed_sfixed64: Vec<i64>,
    #[field(85u32, "packed_float", fixed32, repeated)]
    pub packed_float: Vec<f32>,
    #[field(86u32, "packed_double", fixed64, repeated)]
    pub packed_double: Vec<f64>,
    #[field(87u32, "packed_bool", bool, repeated)]
    pub packed_bool: Vec<bool>,
    #[field(88u32, "packed_nested_enum", protoenum, repeated)]
    pub packed_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
    #[field(89u32, "unpacked_int32", varint, repeated)]
    pub unpacked_int32: Vec<i32>,
    #[field(90u32, "unpacked_int64", varint, repeated)]
    pub unpacked_int64: Vec<i64>,
    #[field(91u32, "unpacked_uint32", varint, repeated)]
    pub unpacked_uint32: Vec<u32>,
    #[field(92u32, "unpacked_uint64", varint, repeated)]
    pub unpacked_uint64: Vec<u64>,
    #[field(93u32, "unpacked_sint32", sigint, repeated)]
    pub unpacked_sint32: Vec<i32>,
    #[field(94u32, "unpacked_sint64", sigint, repeated)]
    pub unpacked_sint64: Vec<i64>,
    #[field(95u32, "unpacked_fixed32", fixed32, repeated)]
    pub unpacked_fixed32: Vec<u32>,
    #[field(96u32, "unpacked_fixed64", fixed64, repeated)]
    pub unpacked_fixed64: Vec<u64>,
    #[field(97u32, "unpacked_sfixed32", fixed32, repeated)]
    pub unpacked_sfixed32: Vec<i32>,
    #[field(98u32, "unpacked_sfixed64", fixed64, repeated)]
    pub unpacked_sfixed64: Vec<i64>,
    #[field(99u32, "unpacked_float", fixed32, repeated)]
    pub unpacked_float: Vec<f32>,
    #[field(100u32, "unpacked_double", fixed64, repeated)]
    pub unpacked_double: Vec<f64>,
    #[field(101u32, "unpacked_bool", bool, repeated)]
    pub unpacked_bool: Vec<bool>,
    #[field(102u32, "unpacked_nested_enum", protoenum, repeated)]
    pub unpacked_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
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
        TestAllTypesProto2NestedMessage,
    >,
    #[field(72u32, "map_string_foreign_message", map(string, nested), singular)]
    pub map_string_foreign_message: ::std::collections::BTreeMap<
        String,
        ForeignMessageProto2,
    >,
    #[field(73u32, "map_string_nested_enum", map(string, protoenum), singular)]
    pub map_string_nested_enum: ::std::collections::BTreeMap<
        String,
        TestAllTypesProto2NestedEnum,
    >,
    #[field(74u32, "map_string_foreign_enum", map(string, protoenum), singular)]
    pub map_string_foreign_enum: ::std::collections::BTreeMap<String, ForeignEnumProto2>,
    #[field(201u32, "Data", group, optional)]
    pub Data: Option<Box<TestAllTypesProto2Data>>,
    #[field(241u32, "default_int32", varint, optional)]
    pub default_int32: Option<i32>,
    #[field(242u32, "default_int64", varint, optional)]
    pub default_int64: Option<i64>,
    #[field(243u32, "default_uint32", varint, optional)]
    pub default_uint32: Option<u32>,
    #[field(244u32, "default_uint64", varint, optional)]
    pub default_uint64: Option<u64>,
    #[field(245u32, "default_sint32", sigint, optional)]
    pub default_sint32: Option<i32>,
    #[field(246u32, "default_sint64", sigint, optional)]
    pub default_sint64: Option<i64>,
    #[field(247u32, "default_fixed32", fixed32, optional)]
    pub default_fixed32: Option<u32>,
    #[field(248u32, "default_fixed64", fixed64, optional)]
    pub default_fixed64: Option<u64>,
    #[field(249u32, "default_sfixed32", fixed32, optional)]
    pub default_sfixed32: Option<i32>,
    #[field(250u32, "default_sfixed64", fixed64, optional)]
    pub default_sfixed64: Option<i64>,
    #[field(251u32, "default_float", fixed32, optional)]
    pub default_float: Option<f32>,
    #[field(252u32, "default_double", fixed64, optional)]
    pub default_double: Option<f64>,
    #[field(253u32, "default_bool", bool, optional)]
    pub default_bool: Option<bool>,
    #[field(254u32, "default_string", string, optional)]
    pub default_string: Option<String>,
    #[field(255u32, "default_bytes", bytes, optional)]
    pub default_bytes: Option<Vec<u8>>,
    #[field(401u32, "fieldname1", varint, optional)]
    pub fieldname1: Option<i32>,
    #[field(402u32, "field_name2", varint, optional)]
    pub field_name2: Option<i32>,
    #[field(403u32, "_field_name3", varint, optional)]
    pub _field_name3: Option<i32>,
    #[field(404u32, "field__name4_", varint, optional)]
    pub field__name4_: Option<i32>,
    #[field(405u32, "field0name5", varint, optional)]
    pub field0name5: Option<i32>,
    #[field(406u32, "field_0_name6", varint, optional)]
    pub field_0_name6: Option<i32>,
    #[field(407u32, "fieldName7", varint, optional)]
    pub fieldName7: Option<i32>,
    #[field(408u32, "FieldName8", varint, optional)]
    pub FieldName8: Option<i32>,
    #[field(409u32, "field_Name9", varint, optional)]
    pub field_Name9: Option<i32>,
    #[field(410u32, "Field_Name10", varint, optional)]
    pub Field_Name10: Option<i32>,
    #[field(411u32, "FIELD_NAME11", varint, optional)]
    pub FIELD_NAME11: Option<i32>,
    #[field(412u32, "FIELD_name12", varint, optional)]
    pub FIELD_name12: Option<i32>,
    #[field(413u32, "__field_name13", varint, optional)]
    pub __field_name13: Option<i32>,
    #[field(414u32, "__Field_name14", varint, optional)]
    pub __Field_name14: Option<i32>,
    #[field(415u32, "field__name15", varint, optional)]
    pub field__name15: Option<i32>,
    #[field(416u32, "field__Name16", varint, optional)]
    pub field__Name16: Option<i32>,
    #[field(417u32, "field_name17__", varint, optional)]
    pub field_name17__: Option<i32>,
    #[field(418u32, "Field_name18__", varint, optional)]
    pub Field_name18__: Option<i32>,
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
        ]
    )]
    pub oneof_field: Option<TestAllTypesProto2OneOfOneofField>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2NestedMessage {
    #[field(1u32, "a", varint, optional)]
    pub a: Option<i32>,
    #[field(2u32, "corecursive", nested, optional)]
    pub corecursive: Option<Box<TestAllTypesProto2>>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2Data {
    #[field(202u32, "group_int32", varint, optional)]
    pub group_int32: Option<i32>,
    #[field(203u32, "group_uint32", varint, optional)]
    pub group_uint32: Option<u32>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2MessageSetCorrect {
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2MessageSetCorrectExtension1 {
    #[field(1547769u32, "message_set_extension", nested, optional)]
    pub message_set_extension: Option<
        Box<TestAllTypesProto2MessageSetCorrectExtension1>,
    >,
    #[field(25u32, "str", string, optional)]
    pub str: Option<String>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TestAllTypesProto2MessageSetCorrectExtension2 {
    #[field(4135312u32, "message_set_extension", nested, optional)]
    pub message_set_extension: Option<
        Box<TestAllTypesProto2MessageSetCorrectExtension2>,
    >,
    #[field(9u32, "i", varint, optional)]
    pub i: Option<i32>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ForeignMessageProto2 {
    #[field(1u32, "c", varint, optional)]
    pub c: Option<i32>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct UnknownToTestAllTypes {
    #[field(1001u32, "optional_int32", varint, optional)]
    pub optional_int32: Option<i32>,
    #[field(1002u32, "optional_string", string, optional)]
    pub optional_string: Option<String>,
    #[field(1003u32, "nested_message", nested, optional)]
    pub nested_message: Option<Box<ForeignMessageProto2>>,
    #[field(1004u32, "OptionalGroup", group, optional)]
    pub OptionalGroup: Option<Box<UnknownToTestAllTypesOptionalGroup>>,
    #[field(1006u32, "optional_bool", bool, optional)]
    pub optional_bool: Option<bool>,
    #[field(1011u32, "repeated_int32", varint, repeated)]
    pub repeated_int32: Vec<i32>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct UnknownToTestAllTypesOptionalGroup {
    #[field(1u32, "a", varint, optional)]
    pub a: Option<i32>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct NullHypothesisProto2 {
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct EnumOnlyProto2 {
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct OneStringProto2 {
    #[field(1u32, "data", string, optional)]
    pub data: Option<String>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ProtoWithKeywords {
    #[field(1u32, "inline", varint, optional)]
    pub inline: Option<i32>,
    #[field(2u32, "concept", string, optional)]
    pub concept: Option<String>,
    #[field(3u32, "requires", string, repeated)]
    pub requires: Vec<String>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
