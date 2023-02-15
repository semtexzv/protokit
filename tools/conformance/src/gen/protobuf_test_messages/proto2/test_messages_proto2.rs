#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&TestAllTypesProto2::default());
    registry.register(&TestAllTypesProto2NestedMessage::default());
    registry.register(&TestAllTypesProto2MessageSetCorrect::default());
    registry.register(&TestAllTypesProto2MessageSetCorrectExtension1::default());
    registry.register(&TestAllTypesProto2MessageSetCorrectExtension2::default());
    registry.register(&ForeignMessageProto2::default());
    registry.register(&UnknownToTestAllTypes::default());
    registry.register(&NullHypothesisProto2::default());
    registry.register(&EnumOnlyProto2::default());
    registry.register(&OneStringProto2::default());
    registry.register(&ProtoWithKeywords::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto2 {
    pub optional_int32: Option<i32>,
    pub optional_int64: Option<i64>,
    pub optional_uint32: Option<u32>,
    pub optional_uint64: Option<u64>,
    pub optional_sint32: Option<i32>,
    pub optional_sint64: Option<i64>,
    pub optional_fixed32: Option<u32>,
    pub optional_fixed64: Option<u64>,
    pub optional_sfixed32: Option<i32>,
    pub optional_sfixed64: Option<i64>,
    pub optional_float: Option<f32>,
    pub optional_double: Option<f64>,
    pub optional_bool: Option<bool>,
    pub optional_string: Option<String>,
    pub optional_bytes: Option<Vec<u8>>,
    pub optional_nested_message: Option<Box<TestAllTypesProto2NestedMessage>>,
    pub optional_foreign_message: Option<Box<ForeignMessageProto2>>,
    pub optional_nested_enum: Option<TestAllTypesProto2NestedEnum>,
    pub optional_foreign_enum: Option<ForeignEnumProto2>,
    pub optional_string_piece: Option<String>,
    pub optional_cord: Option<String>,
    pub recursive_message: Option<Box<TestAllTypesProto2>>,
    pub repeated_int32: Vec<i32>,
    pub repeated_int64: Vec<i64>,
    pub repeated_uint32: Vec<u32>,
    pub repeated_uint64: Vec<u64>,
    pub repeated_sint32: Vec<i32>,
    pub repeated_sint64: Vec<i64>,
    pub repeated_fixed32: Vec<u32>,
    pub repeated_fixed64: Vec<u64>,
    pub repeated_sfixed32: Vec<i32>,
    pub repeated_sfixed64: Vec<i64>,
    pub repeated_float: Vec<f32>,
    pub repeated_double: Vec<f64>,
    pub repeated_bool: Vec<bool>,
    pub repeated_string: Vec<String>,
    pub repeated_bytes: Vec<Vec<u8>>,
    pub repeated_nested_message: Vec<TestAllTypesProto2NestedMessage>,
    pub repeated_foreign_message: Vec<ForeignMessageProto2>,
    pub repeated_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
    pub repeated_foreign_enum: Vec<ForeignEnumProto2>,
    pub repeated_string_piece: Vec<String>,
    pub repeated_cord: Vec<String>,
    pub packed_int32: Vec<i32>,
    pub packed_int64: Vec<i64>,
    pub packed_uint32: Vec<u32>,
    pub packed_uint64: Vec<u64>,
    pub packed_sint32: Vec<i32>,
    pub packed_sint64: Vec<i64>,
    pub packed_fixed32: Vec<u32>,
    pub packed_fixed64: Vec<u64>,
    pub packed_sfixed32: Vec<i32>,
    pub packed_sfixed64: Vec<i64>,
    pub packed_float: Vec<f32>,
    pub packed_double: Vec<f64>,
    pub packed_bool: Vec<bool>,
    pub packed_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
    pub unpacked_int32: Vec<i32>,
    pub unpacked_int64: Vec<i64>,
    pub unpacked_uint32: Vec<u32>,
    pub unpacked_uint64: Vec<u64>,
    pub unpacked_sint32: Vec<i32>,
    pub unpacked_sint64: Vec<i64>,
    pub unpacked_fixed32: Vec<u32>,
    pub unpacked_fixed64: Vec<u64>,
    pub unpacked_sfixed32: Vec<i32>,
    pub unpacked_sfixed64: Vec<i64>,
    pub unpacked_float: Vec<f32>,
    pub unpacked_double: Vec<f64>,
    pub unpacked_bool: Vec<bool>,
    pub unpacked_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
    pub map_int32_int32: ::std::collections::HashMap<i32, i32>,
    pub map_int64_int64: ::std::collections::HashMap<i64, i64>,
    pub map_uint32_uint32: ::std::collections::HashMap<u32, u32>,
    pub map_uint64_uint64: ::std::collections::HashMap<u64, u64>,
    pub map_sint32_sint32: ::std::collections::HashMap<i32, i32>,
    pub map_sint64_sint64: ::std::collections::HashMap<i64, i64>,
    pub map_fixed32_fixed32: ::std::collections::HashMap<u32, u32>,
    pub map_fixed64_fixed64: ::std::collections::HashMap<u64, u64>,
    pub map_sfixed32_sfixed32: ::std::collections::HashMap<i32, i32>,
    pub map_sfixed64_sfixed64: ::std::collections::HashMap<i64, i64>,
    pub map_int32_float: ::std::collections::HashMap<i32, f32>,
    pub map_int32_double: ::std::collections::HashMap<i32, f64>,
    pub map_bool_bool: ::std::collections::HashMap<bool, bool>,
    pub map_string_string: ::std::collections::HashMap<String, String>,
    pub map_string_bytes: ::std::collections::HashMap<String, Vec<u8>>,
    pub map_string_nested_message: ::std::collections::HashMap<
        String,
        TestAllTypesProto2NestedMessage,
    >,
    pub map_string_foreign_message: ::std::collections::HashMap<
        String,
        ForeignMessageProto2,
    >,
    pub map_string_nested_enum: ::std::collections::HashMap<
        String,
        TestAllTypesProto2NestedEnum,
    >,
    pub map_string_foreign_enum: ::std::collections::HashMap<String, ForeignEnumProto2>,
    pub group_int32: Option<i32>,
    pub group_uint32: Option<u32>,
    pub default_int32: Option<i32>,
    pub default_int64: Option<i64>,
    pub default_uint32: Option<u32>,
    pub default_uint64: Option<u64>,
    pub default_sint32: Option<i32>,
    pub default_sint64: Option<i64>,
    pub default_fixed32: Option<u32>,
    pub default_fixed64: Option<u64>,
    pub default_sfixed32: Option<i32>,
    pub default_sfixed64: Option<i64>,
    pub default_float: Option<f32>,
    pub default_double: Option<f64>,
    pub default_bool: Option<bool>,
    pub default_string: Option<String>,
    pub default_bytes: Option<Vec<u8>>,
    pub fieldname1: Option<i32>,
    pub field_name2: Option<i32>,
    pub _field_name3: Option<i32>,
    pub field__name4_: Option<i32>,
    pub field0name5: Option<i32>,
    pub field_0_name6: Option<i32>,
    pub fieldName7: Option<i32>,
    pub FieldName8: Option<i32>,
    pub field_Name9: Option<i32>,
    pub Field_Name10: Option<i32>,
    pub FIELD_NAME11: Option<i32>,
    pub FIELD_name12: Option<i32>,
    pub __field_name13: Option<i32>,
    pub __Field_name14: Option<i32>,
    pub field__name15: Option<i32>,
    pub field__Name16: Option<i32>,
    pub field_name17__: Option<i32>,
    pub Field_name18__: Option<i32>,
    pub extension_int32: Option<i32>,
    pub oneof_field: TestAllTypesProto2OneOfOneofField,
    pub _unknown: (),
}
impl TestAllTypesProto2 {
    #[inline(always)]
    pub fn r#with_optional_int32(mut self, it: i32) -> Self {
        self.r#set_optional_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_int32(&mut self, it: i32) -> &mut Self {
        self.optional_int32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_int64(mut self, it: i64) -> Self {
        self.r#set_optional_int64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_int64(&mut self, it: i64) -> &mut Self {
        self.optional_int64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_uint32(mut self, it: u32) -> Self {
        self.r#set_optional_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_uint32(&mut self, it: u32) -> &mut Self {
        self.optional_uint32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_uint64(mut self, it: u64) -> Self {
        self.r#set_optional_uint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_uint64(&mut self, it: u64) -> &mut Self {
        self.optional_uint64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_sint32(mut self, it: i32) -> Self {
        self.r#set_optional_sint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_sint32(&mut self, it: i32) -> &mut Self {
        self.optional_sint32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_sint64(mut self, it: i64) -> Self {
        self.r#set_optional_sint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_sint64(&mut self, it: i64) -> &mut Self {
        self.optional_sint64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_fixed32(mut self, it: u32) -> Self {
        self.r#set_optional_fixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_fixed32(&mut self, it: u32) -> &mut Self {
        self.optional_fixed32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_fixed64(mut self, it: u64) -> Self {
        self.r#set_optional_fixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_fixed64(&mut self, it: u64) -> &mut Self {
        self.optional_fixed64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_sfixed32(mut self, it: i32) -> Self {
        self.r#set_optional_sfixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_sfixed32(&mut self, it: i32) -> &mut Self {
        self.optional_sfixed32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_sfixed64(mut self, it: i64) -> Self {
        self.r#set_optional_sfixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_sfixed64(&mut self, it: i64) -> &mut Self {
        self.optional_sfixed64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_float(mut self, it: f32) -> Self {
        self.r#set_optional_float(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_float(&mut self, it: f32) -> &mut Self {
        self.optional_float = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_double(mut self, it: f64) -> Self {
        self.r#set_optional_double(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_double(&mut self, it: f64) -> &mut Self {
        self.optional_double = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_bool(mut self, it: bool) -> Self {
        self.r#set_optional_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_bool(&mut self, it: bool) -> &mut Self {
        self.optional_bool = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_string(mut self, it: String) -> Self {
        self.r#set_optional_string(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_string(&mut self, it: String) -> &mut Self {
        self.optional_string = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_bytes(mut self, it: Vec<u8>) -> Self {
        self.r#set_optional_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_bytes(&mut self, it: Vec<u8>) -> &mut Self {
        self.optional_bytes = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_nested_message(
        mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> Self {
        self.r#set_optional_nested_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_nested_message(
        &mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> &mut Self {
        self.optional_nested_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_foreign_message(mut self, it: ForeignMessageProto2) -> Self {
        self.r#set_optional_foreign_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_foreign_message(
        &mut self,
        it: ForeignMessageProto2,
    ) -> &mut Self {
        self.optional_foreign_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_nested_enum(
        mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.r#set_optional_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_nested_enum(
        &mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        self.optional_nested_enum = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_foreign_enum(mut self, it: ForeignEnumProto2) -> Self {
        self.r#set_optional_foreign_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_foreign_enum(&mut self, it: ForeignEnumProto2) -> &mut Self {
        self.optional_foreign_enum = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_string_piece(mut self, it: String) -> Self {
        self.r#set_optional_string_piece(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_string_piece(&mut self, it: String) -> &mut Self {
        self.optional_string_piece = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_cord(mut self, it: String) -> Self {
        self.r#set_optional_cord(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_cord(&mut self, it: String) -> &mut Self {
        self.optional_cord = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_recursive_message(mut self, it: TestAllTypesProto2) -> Self {
        self.r#set_recursive_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_recursive_message(&mut self, it: TestAllTypesProto2) -> &mut Self {
        self.recursive_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_int32(mut self, it: i32) -> Self {
        self.r#add_repeated_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_int32(&mut self, it: i32) -> &mut Self {
        self.repeated_int32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_int64(mut self, it: i64) -> Self {
        self.r#add_repeated_int64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_int64(&mut self, it: i64) -> &mut Self {
        self.repeated_int64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_uint32(mut self, it: u32) -> Self {
        self.r#add_repeated_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_uint32(&mut self, it: u32) -> &mut Self {
        self.repeated_uint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_uint64(mut self, it: u64) -> Self {
        self.r#add_repeated_uint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_uint64(&mut self, it: u64) -> &mut Self {
        self.repeated_uint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_sint32(mut self, it: i32) -> Self {
        self.r#add_repeated_sint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_sint32(&mut self, it: i32) -> &mut Self {
        self.repeated_sint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_sint64(mut self, it: i64) -> Self {
        self.r#add_repeated_sint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_sint64(&mut self, it: i64) -> &mut Self {
        self.repeated_sint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_fixed32(mut self, it: u32) -> Self {
        self.r#add_repeated_fixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_fixed32(&mut self, it: u32) -> &mut Self {
        self.repeated_fixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_fixed64(mut self, it: u64) -> Self {
        self.r#add_repeated_fixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_fixed64(&mut self, it: u64) -> &mut Self {
        self.repeated_fixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_sfixed32(mut self, it: i32) -> Self {
        self.r#add_repeated_sfixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_sfixed32(&mut self, it: i32) -> &mut Self {
        self.repeated_sfixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_sfixed64(mut self, it: i64) -> Self {
        self.r#add_repeated_sfixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_sfixed64(&mut self, it: i64) -> &mut Self {
        self.repeated_sfixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_float(mut self, it: f32) -> Self {
        self.r#add_repeated_float(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_float(&mut self, it: f32) -> &mut Self {
        self.repeated_float.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_double(mut self, it: f64) -> Self {
        self.r#add_repeated_double(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_double(&mut self, it: f64) -> &mut Self {
        self.repeated_double.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_bool(mut self, it: bool) -> Self {
        self.r#add_repeated_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_bool(&mut self, it: bool) -> &mut Self {
        self.repeated_bool.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_string(mut self, it: String) -> Self {
        self.r#add_repeated_string(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_string(&mut self, it: String) -> &mut Self {
        self.repeated_string.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_bytes(mut self, it: Vec<u8>) -> Self {
        self.r#add_repeated_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_bytes(&mut self, it: Vec<u8>) -> &mut Self {
        self.repeated_bytes.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_nested_message(
        mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> Self {
        self.r#add_repeated_nested_message(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_nested_message(
        &mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> &mut Self {
        self.repeated_nested_message.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_foreign_message(mut self, it: ForeignMessageProto2) -> Self {
        self.r#add_repeated_foreign_message(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_foreign_message(
        &mut self,
        it: ForeignMessageProto2,
    ) -> &mut Self {
        self.repeated_foreign_message.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_nested_enum(
        mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.r#add_repeated_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_nested_enum(
        &mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        self.repeated_nested_enum.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_foreign_enum(mut self, it: ForeignEnumProto2) -> Self {
        self.r#add_repeated_foreign_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_foreign_enum(&mut self, it: ForeignEnumProto2) -> &mut Self {
        self.repeated_foreign_enum.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_string_piece(mut self, it: String) -> Self {
        self.r#add_repeated_string_piece(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_string_piece(&mut self, it: String) -> &mut Self {
        self.repeated_string_piece.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_cord(mut self, it: String) -> Self {
        self.r#add_repeated_cord(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_cord(&mut self, it: String) -> &mut Self {
        self.repeated_cord.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_int32(mut self, it: i32) -> Self {
        self.r#add_packed_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_int32(&mut self, it: i32) -> &mut Self {
        self.packed_int32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_int64(mut self, it: i64) -> Self {
        self.r#add_packed_int64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_int64(&mut self, it: i64) -> &mut Self {
        self.packed_int64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_uint32(mut self, it: u32) -> Self {
        self.r#add_packed_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_uint32(&mut self, it: u32) -> &mut Self {
        self.packed_uint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_uint64(mut self, it: u64) -> Self {
        self.r#add_packed_uint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_uint64(&mut self, it: u64) -> &mut Self {
        self.packed_uint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_sint32(mut self, it: i32) -> Self {
        self.r#add_packed_sint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_sint32(&mut self, it: i32) -> &mut Self {
        self.packed_sint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_sint64(mut self, it: i64) -> Self {
        self.r#add_packed_sint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_sint64(&mut self, it: i64) -> &mut Self {
        self.packed_sint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_fixed32(mut self, it: u32) -> Self {
        self.r#add_packed_fixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_fixed32(&mut self, it: u32) -> &mut Self {
        self.packed_fixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_fixed64(mut self, it: u64) -> Self {
        self.r#add_packed_fixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_fixed64(&mut self, it: u64) -> &mut Self {
        self.packed_fixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_sfixed32(mut self, it: i32) -> Self {
        self.r#add_packed_sfixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_sfixed32(&mut self, it: i32) -> &mut Self {
        self.packed_sfixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_sfixed64(mut self, it: i64) -> Self {
        self.r#add_packed_sfixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_sfixed64(&mut self, it: i64) -> &mut Self {
        self.packed_sfixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_float(mut self, it: f32) -> Self {
        self.r#add_packed_float(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_float(&mut self, it: f32) -> &mut Self {
        self.packed_float.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_double(mut self, it: f64) -> Self {
        self.r#add_packed_double(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_double(&mut self, it: f64) -> &mut Self {
        self.packed_double.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_bool(mut self, it: bool) -> Self {
        self.r#add_packed_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_bool(&mut self, it: bool) -> &mut Self {
        self.packed_bool.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_packed_nested_enum(
        mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.r#add_packed_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_nested_enum(
        &mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        self.packed_nested_enum.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_int32(mut self, it: i32) -> Self {
        self.r#add_unpacked_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_int32(&mut self, it: i32) -> &mut Self {
        self.unpacked_int32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_int64(mut self, it: i64) -> Self {
        self.r#add_unpacked_int64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_int64(&mut self, it: i64) -> &mut Self {
        self.unpacked_int64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_uint32(mut self, it: u32) -> Self {
        self.r#add_unpacked_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_uint32(&mut self, it: u32) -> &mut Self {
        self.unpacked_uint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_uint64(mut self, it: u64) -> Self {
        self.r#add_unpacked_uint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_uint64(&mut self, it: u64) -> &mut Self {
        self.unpacked_uint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_sint32(mut self, it: i32) -> Self {
        self.r#add_unpacked_sint32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_sint32(&mut self, it: i32) -> &mut Self {
        self.unpacked_sint32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_sint64(mut self, it: i64) -> Self {
        self.r#add_unpacked_sint64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_sint64(&mut self, it: i64) -> &mut Self {
        self.unpacked_sint64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_fixed32(mut self, it: u32) -> Self {
        self.r#add_unpacked_fixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_fixed32(&mut self, it: u32) -> &mut Self {
        self.unpacked_fixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_fixed64(mut self, it: u64) -> Self {
        self.r#add_unpacked_fixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_fixed64(&mut self, it: u64) -> &mut Self {
        self.unpacked_fixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_sfixed32(mut self, it: i32) -> Self {
        self.r#add_unpacked_sfixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_sfixed32(&mut self, it: i32) -> &mut Self {
        self.unpacked_sfixed32.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_sfixed64(mut self, it: i64) -> Self {
        self.r#add_unpacked_sfixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_sfixed64(&mut self, it: i64) -> &mut Self {
        self.unpacked_sfixed64.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_float(mut self, it: f32) -> Self {
        self.r#add_unpacked_float(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_float(&mut self, it: f32) -> &mut Self {
        self.unpacked_float.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_double(mut self, it: f64) -> Self {
        self.r#add_unpacked_double(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_double(&mut self, it: f64) -> &mut Self {
        self.unpacked_double.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_bool(mut self, it: bool) -> Self {
        self.r#add_unpacked_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_bool(&mut self, it: bool) -> &mut Self {
        self.unpacked_bool.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_unpacked_nested_enum(
        mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.r#add_unpacked_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_nested_enum(
        &mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        self.unpacked_nested_enum.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_map_int32_int32(mut self, k: i32, v: i32) -> Self {
        self.r#add_map_int32_int32(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_int32_int32(&mut self, k: i32, v: i32) -> &mut Self {
        let _ = self.map_int32_int32.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_int64_int64(mut self, k: i64, v: i64) -> Self {
        self.r#add_map_int64_int64(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_int64_int64(&mut self, k: i64, v: i64) -> &mut Self {
        let _ = self.map_int64_int64.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_uint32_uint32(mut self, k: u32, v: u32) -> Self {
        self.r#add_map_uint32_uint32(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_uint32_uint32(&mut self, k: u32, v: u32) -> &mut Self {
        let _ = self.map_uint32_uint32.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_uint64_uint64(mut self, k: u64, v: u64) -> Self {
        self.r#add_map_uint64_uint64(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_uint64_uint64(&mut self, k: u64, v: u64) -> &mut Self {
        let _ = self.map_uint64_uint64.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_sint32_sint32(mut self, k: i32, v: i32) -> Self {
        self.r#add_map_sint32_sint32(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_sint32_sint32(&mut self, k: i32, v: i32) -> &mut Self {
        let _ = self.map_sint32_sint32.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_sint64_sint64(mut self, k: i64, v: i64) -> Self {
        self.r#add_map_sint64_sint64(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_sint64_sint64(&mut self, k: i64, v: i64) -> &mut Self {
        let _ = self.map_sint64_sint64.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_fixed32_fixed32(mut self, k: u32, v: u32) -> Self {
        self.r#add_map_fixed32_fixed32(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_fixed32_fixed32(&mut self, k: u32, v: u32) -> &mut Self {
        let _ = self.map_fixed32_fixed32.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_fixed64_fixed64(mut self, k: u64, v: u64) -> Self {
        self.r#add_map_fixed64_fixed64(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_fixed64_fixed64(&mut self, k: u64, v: u64) -> &mut Self {
        let _ = self.map_fixed64_fixed64.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_sfixed32_sfixed32(mut self, k: i32, v: i32) -> Self {
        self.r#add_map_sfixed32_sfixed32(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_sfixed32_sfixed32(&mut self, k: i32, v: i32) -> &mut Self {
        let _ = self.map_sfixed32_sfixed32.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_sfixed64_sfixed64(mut self, k: i64, v: i64) -> Self {
        self.r#add_map_sfixed64_sfixed64(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_sfixed64_sfixed64(&mut self, k: i64, v: i64) -> &mut Self {
        let _ = self.map_sfixed64_sfixed64.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_int32_float(mut self, k: i32, v: f32) -> Self {
        self.r#add_map_int32_float(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_int32_float(&mut self, k: i32, v: f32) -> &mut Self {
        let _ = self.map_int32_float.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_int32_double(mut self, k: i32, v: f64) -> Self {
        self.r#add_map_int32_double(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_int32_double(&mut self, k: i32, v: f64) -> &mut Self {
        let _ = self.map_int32_double.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_bool_bool(mut self, k: bool, v: bool) -> Self {
        self.r#add_map_bool_bool(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_bool_bool(&mut self, k: bool, v: bool) -> &mut Self {
        let _ = self.map_bool_bool.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_string(mut self, k: String, v: String) -> Self {
        self.r#add_map_string_string(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_string(&mut self, k: String, v: String) -> &mut Self {
        let _ = self.map_string_string.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_bytes(mut self, k: String, v: Vec<u8>) -> Self {
        self.r#add_map_string_bytes(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_bytes(&mut self, k: String, v: Vec<u8>) -> &mut Self {
        let _ = self.map_string_bytes.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_nested_message(
        mut self,
        k: String,
        v: TestAllTypesProto2NestedMessage,
    ) -> Self {
        self.r#add_map_string_nested_message(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_nested_message(
        &mut self,
        k: String,
        v: TestAllTypesProto2NestedMessage,
    ) -> &mut Self {
        let _ = self.map_string_nested_message.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_foreign_message(
        mut self,
        k: String,
        v: ForeignMessageProto2,
    ) -> Self {
        self.r#add_map_string_foreign_message(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_foreign_message(
        &mut self,
        k: String,
        v: ForeignMessageProto2,
    ) -> &mut Self {
        let _ = self.map_string_foreign_message.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_nested_enum(
        mut self,
        k: String,
        v: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.r#add_map_string_nested_enum(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_nested_enum(
        &mut self,
        k: String,
        v: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        let _ = self.map_string_nested_enum.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_foreign_enum(
        mut self,
        k: String,
        v: ForeignEnumProto2,
    ) -> Self {
        self.r#add_map_string_foreign_enum(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_foreign_enum(
        &mut self,
        k: String,
        v: ForeignEnumProto2,
    ) -> &mut Self {
        let _ = self.map_string_foreign_enum.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_group_int32(mut self, it: i32) -> Self {
        self.r#set_group_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_group_int32(&mut self, it: i32) -> &mut Self {
        self.group_int32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_group_uint32(mut self, it: u32) -> Self {
        self.r#set_group_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_group_uint32(&mut self, it: u32) -> &mut Self {
        self.group_uint32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_int32(mut self, it: i32) -> Self {
        self.r#set_default_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_int32(&mut self, it: i32) -> &mut Self {
        self.default_int32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_int64(mut self, it: i64) -> Self {
        self.r#set_default_int64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_int64(&mut self, it: i64) -> &mut Self {
        self.default_int64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_uint32(mut self, it: u32) -> Self {
        self.r#set_default_uint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_uint32(&mut self, it: u32) -> &mut Self {
        self.default_uint32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_uint64(mut self, it: u64) -> Self {
        self.r#set_default_uint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_uint64(&mut self, it: u64) -> &mut Self {
        self.default_uint64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_sint32(mut self, it: i32) -> Self {
        self.r#set_default_sint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_sint32(&mut self, it: i32) -> &mut Self {
        self.default_sint32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_sint64(mut self, it: i64) -> Self {
        self.r#set_default_sint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_sint64(&mut self, it: i64) -> &mut Self {
        self.default_sint64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_fixed32(mut self, it: u32) -> Self {
        self.r#set_default_fixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_fixed32(&mut self, it: u32) -> &mut Self {
        self.default_fixed32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_fixed64(mut self, it: u64) -> Self {
        self.r#set_default_fixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_fixed64(&mut self, it: u64) -> &mut Self {
        self.default_fixed64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_sfixed32(mut self, it: i32) -> Self {
        self.r#set_default_sfixed32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_sfixed32(&mut self, it: i32) -> &mut Self {
        self.default_sfixed32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_sfixed64(mut self, it: i64) -> Self {
        self.r#set_default_sfixed64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_sfixed64(&mut self, it: i64) -> &mut Self {
        self.default_sfixed64 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_float(mut self, it: f32) -> Self {
        self.r#set_default_float(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_float(&mut self, it: f32) -> &mut Self {
        self.default_float = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_double(mut self, it: f64) -> Self {
        self.r#set_default_double(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_double(&mut self, it: f64) -> &mut Self {
        self.default_double = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_bool(mut self, it: bool) -> Self {
        self.r#set_default_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_bool(&mut self, it: bool) -> &mut Self {
        self.default_bool = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_string(mut self, it: String) -> Self {
        self.r#set_default_string(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_string(&mut self, it: String) -> &mut Self {
        self.default_string = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_bytes(mut self, it: Vec<u8>) -> Self {
        self.r#set_default_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_bytes(&mut self, it: Vec<u8>) -> &mut Self {
        self.default_bytes = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_fieldname1(mut self, it: i32) -> Self {
        self.r#set_fieldname1(it);
        self
    }
    #[inline(always)]
    pub fn r#set_fieldname1(&mut self, it: i32) -> &mut Self {
        self.fieldname1 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field_name2(mut self, it: i32) -> Self {
        self.r#set_field_name2(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field_name2(&mut self, it: i32) -> &mut Self {
        self.field_name2 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with__field_name3(mut self, it: i32) -> Self {
        self.r#set__field_name3(it);
        self
    }
    #[inline(always)]
    pub fn r#set__field_name3(&mut self, it: i32) -> &mut Self {
        self._field_name3 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field__name4_(mut self, it: i32) -> Self {
        self.r#set_field__name4_(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field__name4_(&mut self, it: i32) -> &mut Self {
        self.field__name4_ = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field0name5(mut self, it: i32) -> Self {
        self.r#set_field0name5(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field0name5(&mut self, it: i32) -> &mut Self {
        self.field0name5 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field_0_name6(mut self, it: i32) -> Self {
        self.r#set_field_0_name6(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field_0_name6(&mut self, it: i32) -> &mut Self {
        self.field_0_name6 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_fieldName7(mut self, it: i32) -> Self {
        self.r#set_fieldName7(it);
        self
    }
    #[inline(always)]
    pub fn r#set_fieldName7(&mut self, it: i32) -> &mut Self {
        self.fieldName7 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_FieldName8(mut self, it: i32) -> Self {
        self.r#set_FieldName8(it);
        self
    }
    #[inline(always)]
    pub fn r#set_FieldName8(&mut self, it: i32) -> &mut Self {
        self.FieldName8 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field_Name9(mut self, it: i32) -> Self {
        self.r#set_field_Name9(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field_Name9(&mut self, it: i32) -> &mut Self {
        self.field_Name9 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_Field_Name10(mut self, it: i32) -> Self {
        self.r#set_Field_Name10(it);
        self
    }
    #[inline(always)]
    pub fn r#set_Field_Name10(&mut self, it: i32) -> &mut Self {
        self.Field_Name10 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_FIELD_NAME11(mut self, it: i32) -> Self {
        self.r#set_FIELD_NAME11(it);
        self
    }
    #[inline(always)]
    pub fn r#set_FIELD_NAME11(&mut self, it: i32) -> &mut Self {
        self.FIELD_NAME11 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_FIELD_name12(mut self, it: i32) -> Self {
        self.r#set_FIELD_name12(it);
        self
    }
    #[inline(always)]
    pub fn r#set_FIELD_name12(&mut self, it: i32) -> &mut Self {
        self.FIELD_name12 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with___field_name13(mut self, it: i32) -> Self {
        self.r#set___field_name13(it);
        self
    }
    #[inline(always)]
    pub fn r#set___field_name13(&mut self, it: i32) -> &mut Self {
        self.__field_name13 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with___Field_name14(mut self, it: i32) -> Self {
        self.r#set___Field_name14(it);
        self
    }
    #[inline(always)]
    pub fn r#set___Field_name14(&mut self, it: i32) -> &mut Self {
        self.__Field_name14 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field__name15(mut self, it: i32) -> Self {
        self.r#set_field__name15(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field__name15(&mut self, it: i32) -> &mut Self {
        self.field__name15 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field__Name16(mut self, it: i32) -> Self {
        self.r#set_field__Name16(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field__Name16(&mut self, it: i32) -> &mut Self {
        self.field__Name16 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_field_name17__(mut self, it: i32) -> Self {
        self.r#set_field_name17__(it);
        self
    }
    #[inline(always)]
    pub fn r#set_field_name17__(&mut self, it: i32) -> &mut Self {
        self.field_name17__ = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_Field_name18__(mut self, it: i32) -> Self {
        self.r#set_Field_name18__(it);
        self
    }
    #[inline(always)]
    pub fn r#set_Field_name18__(&mut self, it: i32) -> &mut Self {
        self.Field_name18__ = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_extension_int32(mut self, it: i32) -> Self {
        self.r#set_extension_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_extension_int32(&mut self, it: i32) -> &mut Self {
        self.extension_int32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_uint32(mut self, it: u32) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_uint32(&mut self, it: u32) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint32(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_nested_message(
        mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofNestedMessage(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_nested_message(
        &mut self,
        it: TestAllTypesProto2NestedMessage,
    ) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofNestedMessage(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_string(mut self, it: String) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofString(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_string(&mut self, it: String) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofString(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_bytes(mut self, it: Vec<u8>) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_bytes(&mut self, it: Vec<u8>) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBytes(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_bool(mut self, it: bool) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBool(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_bool(&mut self, it: bool) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBool(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_uint64(mut self, it: u64) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_uint64(&mut self, it: u64) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint64(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_float(mut self, it: f32) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofFloat(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_float(&mut self, it: f32) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofFloat(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_double(mut self, it: f64) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofDouble(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_double(&mut self, it: f64) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofDouble(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_enum(
        mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofEnum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_enum(
        &mut self,
        it: TestAllTypesProto2NestedEnum,
    ) -> &mut Self {
        self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofEnum(it);
        self
    }
}
impl textformat::Decodable for TestAllTypesProto2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("optional_int32") => {
                textformat::Field::merge(&mut self.optional_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_int64") => {
                textformat::Field::merge(&mut self.optional_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_uint32") => {
                textformat::Field::merge(&mut self.optional_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_uint64") => {
                textformat::Field::merge(&mut self.optional_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_sint32") => {
                textformat::Field::merge(&mut self.optional_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_sint64") => {
                textformat::Field::merge(&mut self.optional_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_fixed32") => {
                textformat::Field::merge(&mut self.optional_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_fixed64") => {
                textformat::Field::merge(&mut self.optional_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_sfixed32") => {
                textformat::Field::merge(&mut self.optional_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_sfixed64") => {
                textformat::Field::merge(&mut self.optional_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_float") => {
                textformat::Field::merge(&mut self.optional_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_double") => {
                textformat::Field::merge(&mut self.optional_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_bool") => {
                textformat::Field::merge(&mut self.optional_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_string") => {
                textformat::Field::merge(&mut self.optional_string, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_bytes") => {
                textformat::Field::merge(&mut self.optional_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_nested_message") => {
                textformat::Field::merge(&mut self.optional_nested_message, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_foreign_message") => {
                textformat::Field::merge(
                    &mut self.optional_foreign_message,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("optional_nested_enum") => {
                textformat::Field::merge(&mut self.optional_nested_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_foreign_enum") => {
                textformat::Field::merge(&mut self.optional_foreign_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_string_piece") => {
                textformat::Field::merge(&mut self.optional_string_piece, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_cord") => {
                textformat::Field::merge(&mut self.optional_cord, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("recursive_message") => {
                textformat::Field::merge(&mut self.recursive_message, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_int32") => {
                textformat::Field::merge(&mut self.repeated_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_int64") => {
                textformat::Field::merge(&mut self.repeated_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_uint32") => {
                textformat::Field::merge(&mut self.repeated_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_uint64") => {
                textformat::Field::merge(&mut self.repeated_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_sint32") => {
                textformat::Field::merge(&mut self.repeated_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_sint64") => {
                textformat::Field::merge(&mut self.repeated_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_fixed32") => {
                textformat::Field::merge(&mut self.repeated_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_fixed64") => {
                textformat::Field::merge(&mut self.repeated_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_sfixed32") => {
                textformat::Field::merge(&mut self.repeated_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_sfixed64") => {
                textformat::Field::merge(&mut self.repeated_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_float") => {
                textformat::Field::merge(&mut self.repeated_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_double") => {
                textformat::Field::merge(&mut self.repeated_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_bool") => {
                textformat::Field::merge(&mut self.repeated_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_string") => {
                textformat::Field::merge(&mut self.repeated_string, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_bytes") => {
                textformat::Field::merge(&mut self.repeated_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_nested_message") => {
                textformat::Field::merge(&mut self.repeated_nested_message, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_foreign_message") => {
                textformat::Field::merge(
                    &mut self.repeated_foreign_message,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("repeated_nested_enum") => {
                textformat::Field::merge(&mut self.repeated_nested_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_foreign_enum") => {
                textformat::Field::merge(&mut self.repeated_foreign_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_string_piece") => {
                textformat::Field::merge(&mut self.repeated_string_piece, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_cord") => {
                textformat::Field::merge(&mut self.repeated_cord, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_int32") => {
                textformat::Field::merge(&mut self.packed_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_int64") => {
                textformat::Field::merge(&mut self.packed_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_uint32") => {
                textformat::Field::merge(&mut self.packed_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_uint64") => {
                textformat::Field::merge(&mut self.packed_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_sint32") => {
                textformat::Field::merge(&mut self.packed_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_sint64") => {
                textformat::Field::merge(&mut self.packed_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_fixed32") => {
                textformat::Field::merge(&mut self.packed_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_fixed64") => {
                textformat::Field::merge(&mut self.packed_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_sfixed32") => {
                textformat::Field::merge(&mut self.packed_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_sfixed64") => {
                textformat::Field::merge(&mut self.packed_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_float") => {
                textformat::Field::merge(&mut self.packed_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_double") => {
                textformat::Field::merge(&mut self.packed_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_bool") => {
                textformat::Field::merge(&mut self.packed_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed_nested_enum") => {
                textformat::Field::merge(&mut self.packed_nested_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_int32") => {
                textformat::Field::merge(&mut self.unpacked_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_int64") => {
                textformat::Field::merge(&mut self.unpacked_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_uint32") => {
                textformat::Field::merge(&mut self.unpacked_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_uint64") => {
                textformat::Field::merge(&mut self.unpacked_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_sint32") => {
                textformat::Field::merge(&mut self.unpacked_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_sint64") => {
                textformat::Field::merge(&mut self.unpacked_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_fixed32") => {
                textformat::Field::merge(&mut self.unpacked_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_fixed64") => {
                textformat::Field::merge(&mut self.unpacked_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_sfixed32") => {
                textformat::Field::merge(&mut self.unpacked_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_sfixed64") => {
                textformat::Field::merge(&mut self.unpacked_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_float") => {
                textformat::Field::merge(&mut self.unpacked_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_double") => {
                textformat::Field::merge(&mut self.unpacked_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_bool") => {
                textformat::Field::merge(&mut self.unpacked_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unpacked_nested_enum") => {
                textformat::Field::merge(&mut self.unpacked_nested_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_int32_int32") => {
                textformat::Field::merge(&mut self.map_int32_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_int64_int64") => {
                textformat::Field::merge(&mut self.map_int64_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_uint32_uint32") => {
                textformat::Field::merge(&mut self.map_uint32_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_uint64_uint64") => {
                textformat::Field::merge(&mut self.map_uint64_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_sint32_sint32") => {
                textformat::Field::merge(&mut self.map_sint32_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_sint64_sint64") => {
                textformat::Field::merge(&mut self.map_sint64_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_fixed32_fixed32") => {
                textformat::Field::merge(&mut self.map_fixed32_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_fixed64_fixed64") => {
                textformat::Field::merge(&mut self.map_fixed64_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_sfixed32_sfixed32") => {
                textformat::Field::merge(&mut self.map_sfixed32_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_sfixed64_sfixed64") => {
                textformat::Field::merge(&mut self.map_sfixed64_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_int32_float") => {
                textformat::Field::merge(&mut self.map_int32_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_int32_double") => {
                textformat::Field::merge(&mut self.map_int32_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_bool_bool") => {
                textformat::Field::merge(&mut self.map_bool_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_string_string") => {
                textformat::Field::merge(&mut self.map_string_string, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_string_bytes") => {
                textformat::Field::merge(&mut self.map_string_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_string_nested_message") => {
                textformat::Field::merge(
                    &mut self.map_string_nested_message,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("map_string_foreign_message") => {
                textformat::Field::merge(
                    &mut self.map_string_foreign_message,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("map_string_nested_enum") => {
                textformat::Field::merge(&mut self.map_string_nested_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_string_foreign_enum") => {
                textformat::Field::merge(&mut self.map_string_foreign_enum, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("group_int32") => {
                textformat::Field::merge(&mut self.group_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("group_uint32") => {
                textformat::Field::merge(&mut self.group_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_int32") => {
                textformat::Field::merge(&mut self.default_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_int64") => {
                textformat::Field::merge(&mut self.default_int64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_uint32") => {
                textformat::Field::merge(&mut self.default_uint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_uint64") => {
                textformat::Field::merge(&mut self.default_uint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_sint32") => {
                textformat::Field::merge(&mut self.default_sint32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_sint64") => {
                textformat::Field::merge(&mut self.default_sint64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_fixed32") => {
                textformat::Field::merge(&mut self.default_fixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_fixed64") => {
                textformat::Field::merge(&mut self.default_fixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_sfixed32") => {
                textformat::Field::merge(&mut self.default_sfixed32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_sfixed64") => {
                textformat::Field::merge(&mut self.default_sfixed64, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_float") => {
                textformat::Field::merge(&mut self.default_float, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_double") => {
                textformat::Field::merge(&mut self.default_double, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_bool") => {
                textformat::Field::merge(&mut self.default_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_string") => {
                textformat::Field::merge(&mut self.default_string, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_bytes") => {
                textformat::Field::merge(&mut self.default_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("fieldname1") => {
                textformat::Field::merge(&mut self.fieldname1, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field_name2") => {
                textformat::Field::merge(&mut self.field_name2, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("_field_name3") => {
                textformat::Field::merge(&mut self._field_name3, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field__name4_") => {
                textformat::Field::merge(&mut self.field__name4_, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field0name5") => {
                textformat::Field::merge(&mut self.field0name5, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field_0_name6") => {
                textformat::Field::merge(&mut self.field_0_name6, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("fieldName7") => {
                textformat::Field::merge(&mut self.fieldName7, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("FieldName8") => {
                textformat::Field::merge(&mut self.FieldName8, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field_Name9") => {
                textformat::Field::merge(&mut self.field_Name9, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("Field_Name10") => {
                textformat::Field::merge(&mut self.Field_Name10, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("FIELD_NAME11") => {
                textformat::Field::merge(&mut self.FIELD_NAME11, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("FIELD_name12") => {
                textformat::Field::merge(&mut self.FIELD_name12, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("__field_name13") => {
                textformat::Field::merge(&mut self.__field_name13, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("__Field_name14") => {
                textformat::Field::merge(&mut self.__Field_name14, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field__name15") => {
                textformat::Field::merge(&mut self.field__name15, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field__Name16") => {
                textformat::Field::merge(&mut self.field__Name16, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("field_name17__") => {
                textformat::Field::merge(&mut self.field_name17__, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("Field_name18__") => {
                textformat::Field::merge(&mut self.Field_name18__, ctx, value)?;
            }
            textformat::ast::FieldName::Extended(
                "protobuf_test_messages.proto2.extension_int32",
            ) => {
                textformat::Field::merge(&mut self.extension_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oneof_uint32") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint32(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_nested_message") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofNestedMessage(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_string") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofString(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_bytes") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBytes(target);
            }
            textformat::ast::FieldName::Normal("oneof_bool") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBool(target);
            }
            textformat::ast::FieldName::Normal("oneof_uint64") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint64(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_float") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofFloat(target);
            }
            textformat::ast::FieldName::Normal("oneof_double") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofDouble(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_enum") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofEnum(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.optional_int32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_int32: ");
            textformat::Field::format(&self.optional_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_int64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_int64: ");
            textformat::Field::format(&self.optional_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint32 != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_uint32: ");
            textformat::Field::format(&self.optional_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint64 != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_uint64: ");
            textformat::Field::format(&self.optional_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sint32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sint32: ");
            textformat::Field::format(&self.optional_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sint64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sint64: ");
            textformat::Field::format(&self.optional_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_fixed32 != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_fixed32: ");
            textformat::Field::format(&self.optional_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_fixed64 != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_fixed64: ");
            textformat::Field::format(&self.optional_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sfixed32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sfixed32: ");
            textformat::Field::format(&self.optional_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sfixed64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sfixed64: ");
            textformat::Field::format(&self.optional_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_float != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_float: ");
            textformat::Field::format(&self.optional_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_double != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_double: ");
            textformat::Field::format(&self.optional_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bool != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bool: ");
            textformat::Field::format(&self.optional_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_string: ");
            textformat::Field::format(&self.optional_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bytes != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bytes: ");
            textformat::Field::format(&self.optional_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_nested_message
            != <Option<Box<TestAllTypesProto2NestedMessage>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_nested_message ");
            textformat::Field::format(&self.optional_nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_foreign_message
            != <Option<Box<ForeignMessageProto2>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_foreign_message ");
            textformat::Field::format(&self.optional_foreign_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_nested_enum
            != <Option<TestAllTypesProto2NestedEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_nested_enum: ");
            textformat::Field::format(&self.optional_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_foreign_enum
            != <Option<ForeignEnumProto2> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_foreign_enum: ");
            textformat::Field::format(&self.optional_foreign_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string_piece != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_string_piece: ");
            textformat::Field::format(&self.optional_string_piece, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_cord != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_cord: ");
            textformat::Field::format(&self.optional_cord, ctx, pad, out)?;
            out.push('\n');
        }
        if self.recursive_message
            != <Option<Box<TestAllTypesProto2>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("recursive_message ");
            textformat::Field::format(&self.recursive_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_int32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_int32: ");
            textformat::Field::format(&self.repeated_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_int64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_int64: ");
            textformat::Field::format(&self.repeated_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_uint32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_uint32: ");
            textformat::Field::format(&self.repeated_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_uint64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_uint64: ");
            textformat::Field::format(&self.repeated_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_sint32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_sint32: ");
            textformat::Field::format(&self.repeated_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_sint64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_sint64: ");
            textformat::Field::format(&self.repeated_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_fixed32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_fixed32: ");
            textformat::Field::format(&self.repeated_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_fixed64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_fixed64: ");
            textformat::Field::format(&self.repeated_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_sfixed32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_sfixed32: ");
            textformat::Field::format(&self.repeated_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_sfixed64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_sfixed64: ");
            textformat::Field::format(&self.repeated_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_float != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_float: ");
            textformat::Field::format(&self.repeated_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_double != <Vec<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_double: ");
            textformat::Field::format(&self.repeated_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_bool != <Vec<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_bool: ");
            textformat::Field::format(&self.repeated_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_string != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_string: ");
            textformat::Field::format(&self.repeated_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_bytes != <Vec<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_bytes: ");
            textformat::Field::format(&self.repeated_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_nested_message
            != <Vec<TestAllTypesProto2NestedMessage> as Default>::default()
        {
            out.indent(pad);
            out.push_str("repeated_nested_message ");
            textformat::Field::format(&self.repeated_nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_foreign_message
            != <Vec<ForeignMessageProto2> as Default>::default()
        {
            out.indent(pad);
            out.push_str("repeated_foreign_message ");
            textformat::Field::format(&self.repeated_foreign_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_nested_enum
            != <Vec<TestAllTypesProto2NestedEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("repeated_nested_enum: ");
            textformat::Field::format(&self.repeated_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_foreign_enum != <Vec<ForeignEnumProto2> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_foreign_enum: ");
            textformat::Field::format(&self.repeated_foreign_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_string_piece != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_string_piece: ");
            textformat::Field::format(&self.repeated_string_piece, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_cord != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_cord: ");
            textformat::Field::format(&self.repeated_cord, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_int32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_int32: ");
            textformat::Field::format(&self.packed_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_int64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_int64: ");
            textformat::Field::format(&self.packed_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_uint32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_uint32: ");
            textformat::Field::format(&self.packed_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_uint64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_uint64: ");
            textformat::Field::format(&self.packed_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_sint32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_sint32: ");
            textformat::Field::format(&self.packed_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_sint64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_sint64: ");
            textformat::Field::format(&self.packed_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_fixed32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_fixed32: ");
            textformat::Field::format(&self.packed_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_fixed64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_fixed64: ");
            textformat::Field::format(&self.packed_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_sfixed32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_sfixed32: ");
            textformat::Field::format(&self.packed_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_sfixed64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_sfixed64: ");
            textformat::Field::format(&self.packed_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_float != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_float: ");
            textformat::Field::format(&self.packed_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_double != <Vec<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_double: ");
            textformat::Field::format(&self.packed_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_bool != <Vec<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("packed_bool: ");
            textformat::Field::format(&self.packed_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed_nested_enum
            != <Vec<TestAllTypesProto2NestedEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("packed_nested_enum: ");
            textformat::Field::format(&self.packed_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_int32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_int32: ");
            textformat::Field::format(&self.unpacked_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_int64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_int64: ");
            textformat::Field::format(&self.unpacked_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_uint32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_uint32: ");
            textformat::Field::format(&self.unpacked_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_uint64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_uint64: ");
            textformat::Field::format(&self.unpacked_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_sint32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_sint32: ");
            textformat::Field::format(&self.unpacked_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_sint64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_sint64: ");
            textformat::Field::format(&self.unpacked_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_fixed32 != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_fixed32: ");
            textformat::Field::format(&self.unpacked_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_fixed64 != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_fixed64: ");
            textformat::Field::format(&self.unpacked_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_sfixed32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_sfixed32: ");
            textformat::Field::format(&self.unpacked_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_sfixed64 != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_sfixed64: ");
            textformat::Field::format(&self.unpacked_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_float != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_float: ");
            textformat::Field::format(&self.unpacked_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_double != <Vec<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_double: ");
            textformat::Field::format(&self.unpacked_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_bool != <Vec<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("unpacked_bool: ");
            textformat::Field::format(&self.unpacked_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unpacked_nested_enum
            != <Vec<TestAllTypesProto2NestedEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("unpacked_nested_enum: ");
            textformat::Field::format(&self.unpacked_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_int32_int32
            != <::std::collections::HashMap<i32, i32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_int32_int32 ");
            textformat::Field::format(&self.map_int32_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_int64_int64
            != <::std::collections::HashMap<i64, i64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_int64_int64 ");
            textformat::Field::format(&self.map_int64_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_uint32_uint32
            != <::std::collections::HashMap<u32, u32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_uint32_uint32 ");
            textformat::Field::format(&self.map_uint32_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_uint64_uint64
            != <::std::collections::HashMap<u64, u64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_uint64_uint64 ");
            textformat::Field::format(&self.map_uint64_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_sint32_sint32
            != <::std::collections::HashMap<i32, i32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_sint32_sint32 ");
            textformat::Field::format(&self.map_sint32_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_sint64_sint64
            != <::std::collections::HashMap<i64, i64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_sint64_sint64 ");
            textformat::Field::format(&self.map_sint64_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_fixed32_fixed32
            != <::std::collections::HashMap<u32, u32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_fixed32_fixed32 ");
            textformat::Field::format(&self.map_fixed32_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_fixed64_fixed64
            != <::std::collections::HashMap<u64, u64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_fixed64_fixed64 ");
            textformat::Field::format(&self.map_fixed64_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_sfixed32_sfixed32
            != <::std::collections::HashMap<i32, i32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_sfixed32_sfixed32 ");
            textformat::Field::format(&self.map_sfixed32_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_sfixed64_sfixed64
            != <::std::collections::HashMap<i64, i64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_sfixed64_sfixed64 ");
            textformat::Field::format(&self.map_sfixed64_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_int32_float
            != <::std::collections::HashMap<i32, f32> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_int32_float ");
            textformat::Field::format(&self.map_int32_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_int32_double
            != <::std::collections::HashMap<i32, f64> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_int32_double ");
            textformat::Field::format(&self.map_int32_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_bool_bool
            != <::std::collections::HashMap<bool, bool> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_bool_bool ");
            textformat::Field::format(&self.map_bool_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_string
            != <::std::collections::HashMap<String, String> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_string ");
            textformat::Field::format(&self.map_string_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_bytes
            != <::std::collections::HashMap<String, Vec<u8>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_bytes ");
            textformat::Field::format(&self.map_string_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_nested_message
            != <::std::collections::HashMap<
                String,
                TestAllTypesProto2NestedMessage,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_nested_message ");
            textformat::Field::format(&self.map_string_nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_foreign_message
            != <::std::collections::HashMap<
                String,
                ForeignMessageProto2,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_foreign_message ");
            textformat::Field::format(&self.map_string_foreign_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_nested_enum
            != <::std::collections::HashMap<
                String,
                TestAllTypesProto2NestedEnum,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_nested_enum ");
            textformat::Field::format(&self.map_string_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_foreign_enum
            != <::std::collections::HashMap<
                String,
                ForeignEnumProto2,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_foreign_enum ");
            textformat::Field::format(&self.map_string_foreign_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.group_int32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("group_int32: ");
            textformat::Field::format(&self.group_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.group_uint32 != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("group_uint32: ");
            textformat::Field::format(&self.group_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_int32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_int32: ");
            textformat::Field::format(&self.default_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_int64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_int64: ");
            textformat::Field::format(&self.default_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_uint32 != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_uint32: ");
            textformat::Field::format(&self.default_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_uint64 != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_uint64: ");
            textformat::Field::format(&self.default_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_sint32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_sint32: ");
            textformat::Field::format(&self.default_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_sint64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_sint64: ");
            textformat::Field::format(&self.default_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_fixed32 != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_fixed32: ");
            textformat::Field::format(&self.default_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_fixed64 != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_fixed64: ");
            textformat::Field::format(&self.default_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_sfixed32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_sfixed32: ");
            textformat::Field::format(&self.default_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_sfixed64 != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_sfixed64: ");
            textformat::Field::format(&self.default_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_float != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("default_float: ");
            textformat::Field::format(&self.default_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_double != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("default_double: ");
            textformat::Field::format(&self.default_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_bool != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("default_bool: ");
            textformat::Field::format(&self.default_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_string != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("default_string: ");
            textformat::Field::format(&self.default_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_bytes != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("default_bytes: ");
            textformat::Field::format(&self.default_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.fieldname1 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("fieldname1: ");
            textformat::Field::format(&self.fieldname1, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_name2 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field_name2: ");
            textformat::Field::format(&self.field_name2, ctx, pad, out)?;
            out.push('\n');
        }
        if self._field_name3 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("_field_name3: ");
            textformat::Field::format(&self._field_name3, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__name4_ != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field__name4_: ");
            textformat::Field::format(&self.field__name4_, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field0name5 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field0name5: ");
            textformat::Field::format(&self.field0name5, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_0_name6 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field_0_name6: ");
            textformat::Field::format(&self.field_0_name6, ctx, pad, out)?;
            out.push('\n');
        }
        if self.fieldName7 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("fieldName7: ");
            textformat::Field::format(&self.fieldName7, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FieldName8 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("FieldName8: ");
            textformat::Field::format(&self.FieldName8, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_Name9 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field_Name9: ");
            textformat::Field::format(&self.field_Name9, ctx, pad, out)?;
            out.push('\n');
        }
        if self.Field_Name10 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("Field_Name10: ");
            textformat::Field::format(&self.Field_Name10, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FIELD_NAME11 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("FIELD_NAME11: ");
            textformat::Field::format(&self.FIELD_NAME11, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FIELD_name12 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("FIELD_name12: ");
            textformat::Field::format(&self.FIELD_name12, ctx, pad, out)?;
            out.push('\n');
        }
        if self.__field_name13 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("__field_name13: ");
            textformat::Field::format(&self.__field_name13, ctx, pad, out)?;
            out.push('\n');
        }
        if self.__Field_name14 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("__Field_name14: ");
            textformat::Field::format(&self.__Field_name14, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__name15 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field__name15: ");
            textformat::Field::format(&self.field__name15, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__Name16 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field__Name16: ");
            textformat::Field::format(&self.field__Name16, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_name17__ != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("field_name17__: ");
            textformat::Field::format(&self.field_name17__, ctx, pad, out)?;
            out.push('\n');
        }
        if self.Field_name18__ != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("Field_name18__: ");
            textformat::Field::format(&self.Field_name18__, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extension_int32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("[protobuf_test_messages.proto2.extension_int32]: ");
            textformat::Field::format(&self.extension_int32, ctx, pad, out)?;
            out.push('\n');
        }
        match &self.oneof_field {
            TestAllTypesProto2OneOfOneofField::OneofUint32(value) => {
                out.indent(pad);
                out.push_str("oneof_uint32: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofNestedMessage(value) => {
                out.indent(pad);
                out.push_str("oneof_nested_message ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofString(value) => {
                out.indent(pad);
                out.push_str("oneof_string: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofBytes(value) => {
                out.indent(pad);
                out.push_str("oneof_bytes: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofBool(value) => {
                out.indent(pad);
                out.push_str("oneof_bool: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofUint64(value) => {
                out.indent(pad);
                out.push_str("oneof_uint64: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofFloat(value) => {
                out.indent(pad);
                out.push_str("oneof_float: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofDouble(value) => {
                out.indent(pad);
                out.push_str("oneof_double: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::OneofEnum(value) => {
                out.indent(pad);
                out.push_str("oneof_enum: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto2OneOfOneofField::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int32, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int32, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int64, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int64, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_uint32, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_uint32, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_uint64, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_uint64, buf)?;
            }
            40u32 => {
                buf = Format::<SInt>::decode(&mut self.optional_sint32, buf)?;
            }
            42u32 => {
                buf = Format::<SInt>::decode(&mut self.optional_sint32, buf)?;
            }
            48u32 => {
                buf = Format::<SInt>::decode(&mut self.optional_sint64, buf)?;
            }
            50u32 => {
                buf = Format::<SInt>::decode(&mut self.optional_sint64, buf)?;
            }
            61u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_fixed32, buf)?;
            }
            58u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_fixed32, buf)?;
            }
            65u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_fixed64, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_fixed64, buf)?;
            }
            77u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_sfixed32, buf)?;
            }
            74u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_sfixed32, buf)?;
            }
            81u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_sfixed64, buf)?;
            }
            82u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_sfixed64, buf)?;
            }
            93u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_float, buf)?;
            }
            90u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_float, buf)?;
            }
            97u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_double, buf)?;
            }
            98u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_double, buf)?;
            }
            104u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_bool, buf)?;
            }
            106u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_bool, buf)?;
            }
            114u32 => {
                buf = Format::<Bytes>::decode(&mut self.optional_string, buf)?;
            }
            122u32 => {
                buf = Format::<Bytes>::decode(&mut self.optional_bytes, buf)?;
            }
            146u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_nested_message, buf)?;
            }
            154u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_foreign_message, buf)?;
            }
            168u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_nested_enum, buf)?;
            }
            170u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_nested_enum, buf)?;
            }
            176u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_foreign_enum, buf)?;
            }
            178u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_foreign_enum, buf)?;
            }
            194u32 => {
                buf = Format::<Bytes>::decode(&mut self.optional_string_piece, buf)?;
            }
            202u32 => {
                buf = Format::<Bytes>::decode(&mut self.optional_cord, buf)?;
            }
            218u32 => {
                buf = Format::<Nest>::decode(&mut self.recursive_message, buf)?;
            }
            248u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.repeated_int32, buf)?;
            }
            250u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.repeated_int32, buf)?;
            }
            256u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.repeated_int64, buf)?;
            }
            258u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.repeated_int64, buf)?;
            }
            264u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.repeated_uint32, buf)?;
            }
            266u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.repeated_uint32, buf)?;
            }
            272u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.repeated_uint64, buf)?;
            }
            274u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.repeated_uint64, buf)?;
            }
            280u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.repeated_sint32, buf)?;
            }
            282u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.repeated_sint32, buf)?;
            }
            288u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.repeated_sint64, buf)?;
            }
            290u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.repeated_sint64, buf)?;
            }
            301u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_fixed32, buf)?;
            }
            298u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_fixed32, buf)?;
            }
            305u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_fixed64, buf)?;
            }
            306u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_fixed64, buf)?;
            }
            317u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_sfixed32, buf)?;
            }
            314u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_sfixed32, buf)?;
            }
            321u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_sfixed64, buf)?;
            }
            322u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_sfixed64, buf)?;
            }
            333u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_float, buf)?;
            }
            330u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_float, buf)?;
            }
            337u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_double, buf)?;
            }
            338u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_double, buf)?;
            }
            344u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.repeated_bool, buf)?;
            }
            346u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.repeated_bool, buf)?;
            }
            354u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.repeated_string, buf)?;
            }
            362u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.repeated_bytes, buf)?;
            }
            386u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_nested_message, buf)?;
            }
            394u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_foreign_message, buf)?;
            }
            408u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.repeated_nested_enum, buf)?;
            }
            410u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.repeated_nested_enum, buf)?;
            }
            416u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.repeated_foreign_enum, buf)?;
            }
            418u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.repeated_foreign_enum, buf)?;
            }
            434u32 => {
                buf = Format::<
                    Repeat::<Bytes>,
                >::decode(&mut self.repeated_string_piece, buf)?;
            }
            442u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.repeated_cord, buf)?;
            }
            600u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.packed_int32, buf)?;
            }
            602u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.packed_int32, buf)?;
            }
            608u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.packed_int64, buf)?;
            }
            610u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.packed_int64, buf)?;
            }
            616u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.packed_uint32, buf)?;
            }
            618u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.packed_uint32, buf)?;
            }
            624u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.packed_uint64, buf)?;
            }
            626u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.packed_uint64, buf)?;
            }
            632u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.packed_sint32, buf)?;
            }
            634u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.packed_sint32, buf)?;
            }
            640u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.packed_sint64, buf)?;
            }
            642u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.packed_sint64, buf)?;
            }
            653u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_fixed32, buf)?;
            }
            650u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_fixed32, buf)?;
            }
            657u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_fixed64, buf)?;
            }
            658u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_fixed64, buf)?;
            }
            669u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_sfixed32, buf)?;
            }
            666u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_sfixed32, buf)?;
            }
            673u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_sfixed64, buf)?;
            }
            674u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_sfixed64, buf)?;
            }
            685u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_float, buf)?;
            }
            682u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_float, buf)?;
            }
            689u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_double, buf)?;
            }
            690u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_double, buf)?;
            }
            696u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.packed_bool, buf)?;
            }
            698u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.packed_bool, buf)?;
            }
            704u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.packed_nested_enum, buf)?;
            }
            706u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.packed_nested_enum, buf)?;
            }
            712u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.unpacked_int32, buf)?;
            }
            714u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.unpacked_int32, buf)?;
            }
            720u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.unpacked_int64, buf)?;
            }
            722u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.unpacked_int64, buf)?;
            }
            728u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.unpacked_uint32, buf)?;
            }
            730u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.unpacked_uint32, buf)?;
            }
            736u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.unpacked_uint64, buf)?;
            }
            738u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.unpacked_uint64, buf)?;
            }
            744u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.unpacked_sint32, buf)?;
            }
            746u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.unpacked_sint32, buf)?;
            }
            752u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.unpacked_sint64, buf)?;
            }
            754u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.unpacked_sint64, buf)?;
            }
            765u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_fixed32, buf)?;
            }
            762u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_fixed32, buf)?;
            }
            769u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_fixed64, buf)?;
            }
            770u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_fixed64, buf)?;
            }
            781u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_sfixed32, buf)?;
            }
            778u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_sfixed32, buf)?;
            }
            785u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_sfixed64, buf)?;
            }
            786u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_sfixed64, buf)?;
            }
            797u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_float, buf)?;
            }
            794u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_float, buf)?;
            }
            801u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_double, buf)?;
            }
            802u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_double, buf)?;
            }
            808u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.unpacked_bool, buf)?;
            }
            810u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.unpacked_bool, buf)?;
            }
            816u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.unpacked_nested_enum, buf)?;
            }
            818u32 => {
                buf = Format::<
                    Repeat::<Enum>,
                >::decode(&mut self.unpacked_nested_enum, buf)?;
            }
            450u32 => {
                buf = Format::<
                    Map::<VInt, VInt>,
                >::decode(&mut self.map_int32_int32, buf)?;
            }
            458u32 => {
                buf = Format::<
                    Map::<VInt, VInt>,
                >::decode(&mut self.map_int64_int64, buf)?;
            }
            466u32 => {
                buf = Format::<
                    Map::<VInt, VInt>,
                >::decode(&mut self.map_uint32_uint32, buf)?;
            }
            474u32 => {
                buf = Format::<
                    Map::<VInt, VInt>,
                >::decode(&mut self.map_uint64_uint64, buf)?;
            }
            482u32 => {
                buf = Format::<
                    Map::<SInt, SInt>,
                >::decode(&mut self.map_sint32_sint32, buf)?;
            }
            490u32 => {
                buf = Format::<
                    Map::<SInt, SInt>,
                >::decode(&mut self.map_sint64_sint64, buf)?;
            }
            498u32 => {
                buf = Format::<
                    Map::<Fix, Fix>,
                >::decode(&mut self.map_fixed32_fixed32, buf)?;
            }
            506u32 => {
                buf = Format::<
                    Map::<Fix, Fix>,
                >::decode(&mut self.map_fixed64_fixed64, buf)?;
            }
            514u32 => {
                buf = Format::<
                    Map::<Fix, Fix>,
                >::decode(&mut self.map_sfixed32_sfixed32, buf)?;
            }
            522u32 => {
                buf = Format::<
                    Map::<Fix, Fix>,
                >::decode(&mut self.map_sfixed64_sfixed64, buf)?;
            }
            530u32 => {
                buf = Format::<
                    Map::<VInt, Fix>,
                >::decode(&mut self.map_int32_float, buf)?;
            }
            538u32 => {
                buf = Format::<
                    Map::<VInt, Fix>,
                >::decode(&mut self.map_int32_double, buf)?;
            }
            546u32 => {
                buf = Format::<Map::<Fix, Fix>>::decode(&mut self.map_bool_bool, buf)?;
            }
            554u32 => {
                buf = Format::<
                    Map::<Bytes, Bytes>,
                >::decode(&mut self.map_string_string, buf)?;
            }
            562u32 => {
                buf = Format::<
                    Map::<Bytes, Bytes>,
                >::decode(&mut self.map_string_bytes, buf)?;
            }
            570u32 => {
                buf = Format::<
                    Map::<Bytes, Nest>,
                >::decode(&mut self.map_string_nested_message, buf)?;
            }
            578u32 => {
                buf = Format::<
                    Map::<Bytes, Nest>,
                >::decode(&mut self.map_string_foreign_message, buf)?;
            }
            586u32 => {
                buf = Format::<
                    Map::<Bytes, Enum>,
                >::decode(&mut self.map_string_nested_enum, buf)?;
            }
            594u32 => {
                buf = Format::<
                    Map::<Bytes, Enum>,
                >::decode(&mut self.map_string_foreign_enum, buf)?;
            }
            1616u32 => {
                buf = Format::<VInt>::decode(&mut self.group_int32, buf)?;
            }
            1618u32 => {
                buf = Format::<VInt>::decode(&mut self.group_int32, buf)?;
            }
            1624u32 => {
                buf = Format::<VInt>::decode(&mut self.group_uint32, buf)?;
            }
            1626u32 => {
                buf = Format::<VInt>::decode(&mut self.group_uint32, buf)?;
            }
            1928u32 => {
                buf = Format::<VInt>::decode(&mut self.default_int32, buf)?;
            }
            1930u32 => {
                buf = Format::<VInt>::decode(&mut self.default_int32, buf)?;
            }
            1936u32 => {
                buf = Format::<VInt>::decode(&mut self.default_int64, buf)?;
            }
            1938u32 => {
                buf = Format::<VInt>::decode(&mut self.default_int64, buf)?;
            }
            1944u32 => {
                buf = Format::<VInt>::decode(&mut self.default_uint32, buf)?;
            }
            1946u32 => {
                buf = Format::<VInt>::decode(&mut self.default_uint32, buf)?;
            }
            1952u32 => {
                buf = Format::<VInt>::decode(&mut self.default_uint64, buf)?;
            }
            1954u32 => {
                buf = Format::<VInt>::decode(&mut self.default_uint64, buf)?;
            }
            1960u32 => {
                buf = Format::<SInt>::decode(&mut self.default_sint32, buf)?;
            }
            1962u32 => {
                buf = Format::<SInt>::decode(&mut self.default_sint32, buf)?;
            }
            1968u32 => {
                buf = Format::<SInt>::decode(&mut self.default_sint64, buf)?;
            }
            1970u32 => {
                buf = Format::<SInt>::decode(&mut self.default_sint64, buf)?;
            }
            1981u32 => {
                buf = Format::<Fix>::decode(&mut self.default_fixed32, buf)?;
            }
            1978u32 => {
                buf = Format::<Fix>::decode(&mut self.default_fixed32, buf)?;
            }
            1985u32 => {
                buf = Format::<Fix>::decode(&mut self.default_fixed64, buf)?;
            }
            1986u32 => {
                buf = Format::<Fix>::decode(&mut self.default_fixed64, buf)?;
            }
            1997u32 => {
                buf = Format::<Fix>::decode(&mut self.default_sfixed32, buf)?;
            }
            1994u32 => {
                buf = Format::<Fix>::decode(&mut self.default_sfixed32, buf)?;
            }
            2001u32 => {
                buf = Format::<Fix>::decode(&mut self.default_sfixed64, buf)?;
            }
            2002u32 => {
                buf = Format::<Fix>::decode(&mut self.default_sfixed64, buf)?;
            }
            2013u32 => {
                buf = Format::<Fix>::decode(&mut self.default_float, buf)?;
            }
            2010u32 => {
                buf = Format::<Fix>::decode(&mut self.default_float, buf)?;
            }
            2017u32 => {
                buf = Format::<Fix>::decode(&mut self.default_double, buf)?;
            }
            2018u32 => {
                buf = Format::<Fix>::decode(&mut self.default_double, buf)?;
            }
            2024u32 => {
                buf = Format::<Fix>::decode(&mut self.default_bool, buf)?;
            }
            2026u32 => {
                buf = Format::<Fix>::decode(&mut self.default_bool, buf)?;
            }
            2034u32 => {
                buf = Format::<Bytes>::decode(&mut self.default_string, buf)?;
            }
            2042u32 => {
                buf = Format::<Bytes>::decode(&mut self.default_bytes, buf)?;
            }
            3208u32 => {
                buf = Format::<VInt>::decode(&mut self.fieldname1, buf)?;
            }
            3210u32 => {
                buf = Format::<VInt>::decode(&mut self.fieldname1, buf)?;
            }
            3216u32 => {
                buf = Format::<VInt>::decode(&mut self.field_name2, buf)?;
            }
            3218u32 => {
                buf = Format::<VInt>::decode(&mut self.field_name2, buf)?;
            }
            3224u32 => {
                buf = Format::<VInt>::decode(&mut self._field_name3, buf)?;
            }
            3226u32 => {
                buf = Format::<VInt>::decode(&mut self._field_name3, buf)?;
            }
            3232u32 => {
                buf = Format::<VInt>::decode(&mut self.field__name4_, buf)?;
            }
            3234u32 => {
                buf = Format::<VInt>::decode(&mut self.field__name4_, buf)?;
            }
            3240u32 => {
                buf = Format::<VInt>::decode(&mut self.field0name5, buf)?;
            }
            3242u32 => {
                buf = Format::<VInt>::decode(&mut self.field0name5, buf)?;
            }
            3248u32 => {
                buf = Format::<VInt>::decode(&mut self.field_0_name6, buf)?;
            }
            3250u32 => {
                buf = Format::<VInt>::decode(&mut self.field_0_name6, buf)?;
            }
            3256u32 => {
                buf = Format::<VInt>::decode(&mut self.fieldName7, buf)?;
            }
            3258u32 => {
                buf = Format::<VInt>::decode(&mut self.fieldName7, buf)?;
            }
            3264u32 => {
                buf = Format::<VInt>::decode(&mut self.FieldName8, buf)?;
            }
            3266u32 => {
                buf = Format::<VInt>::decode(&mut self.FieldName8, buf)?;
            }
            3272u32 => {
                buf = Format::<VInt>::decode(&mut self.field_Name9, buf)?;
            }
            3274u32 => {
                buf = Format::<VInt>::decode(&mut self.field_Name9, buf)?;
            }
            3280u32 => {
                buf = Format::<VInt>::decode(&mut self.Field_Name10, buf)?;
            }
            3282u32 => {
                buf = Format::<VInt>::decode(&mut self.Field_Name10, buf)?;
            }
            3288u32 => {
                buf = Format::<VInt>::decode(&mut self.FIELD_NAME11, buf)?;
            }
            3290u32 => {
                buf = Format::<VInt>::decode(&mut self.FIELD_NAME11, buf)?;
            }
            3296u32 => {
                buf = Format::<VInt>::decode(&mut self.FIELD_name12, buf)?;
            }
            3298u32 => {
                buf = Format::<VInt>::decode(&mut self.FIELD_name12, buf)?;
            }
            3304u32 => {
                buf = Format::<VInt>::decode(&mut self.__field_name13, buf)?;
            }
            3306u32 => {
                buf = Format::<VInt>::decode(&mut self.__field_name13, buf)?;
            }
            3312u32 => {
                buf = Format::<VInt>::decode(&mut self.__Field_name14, buf)?;
            }
            3314u32 => {
                buf = Format::<VInt>::decode(&mut self.__Field_name14, buf)?;
            }
            3320u32 => {
                buf = Format::<VInt>::decode(&mut self.field__name15, buf)?;
            }
            3322u32 => {
                buf = Format::<VInt>::decode(&mut self.field__name15, buf)?;
            }
            3328u32 => {
                buf = Format::<VInt>::decode(&mut self.field__Name16, buf)?;
            }
            3330u32 => {
                buf = Format::<VInt>::decode(&mut self.field__Name16, buf)?;
            }
            3336u32 => {
                buf = Format::<VInt>::decode(&mut self.field_name17__, buf)?;
            }
            3338u32 => {
                buf = Format::<VInt>::decode(&mut self.field_name17__, buf)?;
            }
            3344u32 => {
                buf = Format::<VInt>::decode(&mut self.Field_name18__, buf)?;
            }
            3346u32 => {
                buf = Format::<VInt>::decode(&mut self.Field_name18__, buf)?;
            }
            960u32 => {
                buf = Format::<VInt>::decode(&mut self.extension_int32, buf)?;
            }
            962u32 => {
                buf = Format::<VInt>::decode(&mut self.extension_int32, buf)?;
            }
            888u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint32(tmp);
            }
            890u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint32(tmp);
            }
            898u32 => {
                let mut tmp = Default::default();
                buf = Format::<Nest>::decode(&mut tmp, buf)?;
                self
                    .oneof_field = TestAllTypesProto2OneOfOneofField::OneofNestedMessage(
                    tmp,
                );
            }
            906u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofString(tmp);
            }
            914u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBytes(tmp);
            }
            920u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBool(tmp);
            }
            922u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofBool(tmp);
            }
            928u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint64(tmp);
            }
            930u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofUint64(tmp);
            }
            941u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofFloat(tmp);
            }
            938u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofFloat(tmp);
            }
            945u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofDouble(tmp);
            }
            946u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofDouble(tmp);
            }
            952u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofEnum(tmp);
            }
            954u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto2OneOfOneofField::OneofEnum(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.TestAllTypesProto2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.optional_int32.should_encode(false) {
            Format::<VInt>::encode(&self.optional_int32, 1u32, buf)?;
        }
        if self.optional_int64.should_encode(false) {
            Format::<VInt>::encode(&self.optional_int64, 2u32, buf)?;
        }
        if self.optional_uint32.should_encode(false) {
            Format::<VInt>::encode(&self.optional_uint32, 3u32, buf)?;
        }
        if self.optional_uint64.should_encode(false) {
            Format::<VInt>::encode(&self.optional_uint64, 4u32, buf)?;
        }
        if self.optional_sint32.should_encode(false) {
            Format::<SInt>::encode(&self.optional_sint32, 5u32, buf)?;
        }
        if self.optional_sint64.should_encode(false) {
            Format::<SInt>::encode(&self.optional_sint64, 6u32, buf)?;
        }
        if self.optional_fixed32.should_encode(false) {
            Format::<Fix>::encode(&self.optional_fixed32, 7u32, buf)?;
        }
        if self.optional_fixed64.should_encode(false) {
            Format::<Fix>::encode(&self.optional_fixed64, 8u32, buf)?;
        }
        if self.optional_sfixed32.should_encode(false) {
            Format::<Fix>::encode(&self.optional_sfixed32, 9u32, buf)?;
        }
        if self.optional_sfixed64.should_encode(false) {
            Format::<Fix>::encode(&self.optional_sfixed64, 10u32, buf)?;
        }
        if self.optional_float.should_encode(false) {
            Format::<Fix>::encode(&self.optional_float, 11u32, buf)?;
        }
        if self.optional_double.should_encode(false) {
            Format::<Fix>::encode(&self.optional_double, 12u32, buf)?;
        }
        if self.optional_bool.should_encode(false) {
            Format::<Fix>::encode(&self.optional_bool, 13u32, buf)?;
        }
        if self.optional_string.should_encode(false) {
            Format::<Bytes>::encode(&self.optional_string, 14u32, buf)?;
        }
        if self.optional_bytes.should_encode(false) {
            Format::<Bytes>::encode(&self.optional_bytes, 15u32, buf)?;
        }
        if self.optional_nested_message.should_encode(false) {
            Format::<Nest>::encode(&self.optional_nested_message, 18u32, buf)?;
        }
        if self.optional_foreign_message.should_encode(false) {
            Format::<Nest>::encode(&self.optional_foreign_message, 19u32, buf)?;
        }
        if self.optional_nested_enum.should_encode(false) {
            Format::<Enum>::encode(&self.optional_nested_enum, 21u32, buf)?;
        }
        if self.optional_foreign_enum.should_encode(false) {
            Format::<Enum>::encode(&self.optional_foreign_enum, 22u32, buf)?;
        }
        if self.optional_string_piece.should_encode(false) {
            Format::<Bytes>::encode(&self.optional_string_piece, 24u32, buf)?;
        }
        if self.optional_cord.should_encode(false) {
            Format::<Bytes>::encode(&self.optional_cord, 25u32, buf)?;
        }
        if self.recursive_message.should_encode(false) {
            Format::<Nest>::encode(&self.recursive_message, 27u32, buf)?;
        }
        if self.repeated_int32.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.repeated_int32, 31u32, buf)?;
        }
        if self.repeated_int64.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.repeated_int64, 32u32, buf)?;
        }
        if self.repeated_uint32.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.repeated_uint32, 33u32, buf)?;
        }
        if self.repeated_uint64.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.repeated_uint64, 34u32, buf)?;
        }
        if self.repeated_sint32.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.repeated_sint32, 35u32, buf)?;
        }
        if self.repeated_sint64.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.repeated_sint64, 36u32, buf)?;
        }
        if self.repeated_fixed32.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_fixed32, 37u32, buf)?;
        }
        if self.repeated_fixed64.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_fixed64, 38u32, buf)?;
        }
        if self.repeated_sfixed32.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_sfixed32, 39u32, buf)?;
        }
        if self.repeated_sfixed64.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_sfixed64, 40u32, buf)?;
        }
        if self.repeated_float.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_float, 41u32, buf)?;
        }
        if self.repeated_double.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_double, 42u32, buf)?;
        }
        if self.repeated_bool.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.repeated_bool, 43u32, buf)?;
        }
        if self.repeated_string.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_string, 44u32, buf)?;
        }
        if self.repeated_bytes.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_bytes, 45u32, buf)?;
        }
        if self.repeated_nested_message.should_encode(false) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_nested_message, 48u32, buf)?;
        }
        if self.repeated_foreign_message.should_encode(false) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_foreign_message, 49u32, buf)?;
        }
        if self.repeated_nested_enum.should_encode(false) {
            Format::<Repeat::<Enum>>::encode(&self.repeated_nested_enum, 51u32, buf)?;
        }
        if self.repeated_foreign_enum.should_encode(false) {
            Format::<Repeat::<Enum>>::encode(&self.repeated_foreign_enum, 52u32, buf)?;
        }
        if self.repeated_string_piece.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_string_piece, 54u32, buf)?;
        }
        if self.repeated_cord.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_cord, 55u32, buf)?;
        }
        if self.packed_int32.should_encode(false) {
            Format::<Pack::<VInt>>::encode(&self.packed_int32, 75u32, buf)?;
        }
        if self.packed_int64.should_encode(false) {
            Format::<Pack::<VInt>>::encode(&self.packed_int64, 76u32, buf)?;
        }
        if self.packed_uint32.should_encode(false) {
            Format::<Pack::<VInt>>::encode(&self.packed_uint32, 77u32, buf)?;
        }
        if self.packed_uint64.should_encode(false) {
            Format::<Pack::<VInt>>::encode(&self.packed_uint64, 78u32, buf)?;
        }
        if self.packed_sint32.should_encode(false) {
            Format::<Pack::<SInt>>::encode(&self.packed_sint32, 79u32, buf)?;
        }
        if self.packed_sint64.should_encode(false) {
            Format::<Pack::<SInt>>::encode(&self.packed_sint64, 80u32, buf)?;
        }
        if self.packed_fixed32.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_fixed32, 81u32, buf)?;
        }
        if self.packed_fixed64.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_fixed64, 82u32, buf)?;
        }
        if self.packed_sfixed32.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_sfixed32, 83u32, buf)?;
        }
        if self.packed_sfixed64.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_sfixed64, 84u32, buf)?;
        }
        if self.packed_float.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_float, 85u32, buf)?;
        }
        if self.packed_double.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_double, 86u32, buf)?;
        }
        if self.packed_bool.should_encode(false) {
            Format::<Pack::<Fix>>::encode(&self.packed_bool, 87u32, buf)?;
        }
        if self.packed_nested_enum.should_encode(false) {
            Format::<Repeat::<Enum>>::encode(&self.packed_nested_enum, 88u32, buf)?;
        }
        if self.unpacked_int32.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.unpacked_int32, 89u32, buf)?;
        }
        if self.unpacked_int64.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.unpacked_int64, 90u32, buf)?;
        }
        if self.unpacked_uint32.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.unpacked_uint32, 91u32, buf)?;
        }
        if self.unpacked_uint64.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.unpacked_uint64, 92u32, buf)?;
        }
        if self.unpacked_sint32.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.unpacked_sint32, 93u32, buf)?;
        }
        if self.unpacked_sint64.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.unpacked_sint64, 94u32, buf)?;
        }
        if self.unpacked_fixed32.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_fixed32, 95u32, buf)?;
        }
        if self.unpacked_fixed64.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_fixed64, 96u32, buf)?;
        }
        if self.unpacked_sfixed32.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_sfixed32, 97u32, buf)?;
        }
        if self.unpacked_sfixed64.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_sfixed64, 98u32, buf)?;
        }
        if self.unpacked_float.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_float, 99u32, buf)?;
        }
        if self.unpacked_double.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_double, 100u32, buf)?;
        }
        if self.unpacked_bool.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.unpacked_bool, 101u32, buf)?;
        }
        if self.unpacked_nested_enum.should_encode(false) {
            Format::<Repeat::<Enum>>::encode(&self.unpacked_nested_enum, 102u32, buf)?;
        }
        if self.map_int32_int32.should_encode(false) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_int32_int32, 56u32, buf)?;
        }
        if self.map_int64_int64.should_encode(false) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_int64_int64, 57u32, buf)?;
        }
        if self.map_uint32_uint32.should_encode(false) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_uint32_uint32, 58u32, buf)?;
        }
        if self.map_uint64_uint64.should_encode(false) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_uint64_uint64, 59u32, buf)?;
        }
        if self.map_sint32_sint32.should_encode(false) {
            Format::<Map::<SInt, SInt>>::encode(&self.map_sint32_sint32, 60u32, buf)?;
        }
        if self.map_sint64_sint64.should_encode(false) {
            Format::<Map::<SInt, SInt>>::encode(&self.map_sint64_sint64, 61u32, buf)?;
        }
        if self.map_fixed32_fixed32.should_encode(false) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_fixed32_fixed32, 62u32, buf)?;
        }
        if self.map_fixed64_fixed64.should_encode(false) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_fixed64_fixed64, 63u32, buf)?;
        }
        if self.map_sfixed32_sfixed32.should_encode(false) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_sfixed32_sfixed32, 64u32, buf)?;
        }
        if self.map_sfixed64_sfixed64.should_encode(false) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_sfixed64_sfixed64, 65u32, buf)?;
        }
        if self.map_int32_float.should_encode(false) {
            Format::<Map::<VInt, Fix>>::encode(&self.map_int32_float, 66u32, buf)?;
        }
        if self.map_int32_double.should_encode(false) {
            Format::<Map::<VInt, Fix>>::encode(&self.map_int32_double, 67u32, buf)?;
        }
        if self.map_bool_bool.should_encode(false) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_bool_bool, 68u32, buf)?;
        }
        if self.map_string_string.should_encode(false) {
            Format::<Map::<Bytes, Bytes>>::encode(&self.map_string_string, 69u32, buf)?;
        }
        if self.map_string_bytes.should_encode(false) {
            Format::<Map::<Bytes, Bytes>>::encode(&self.map_string_bytes, 70u32, buf)?;
        }
        if self.map_string_nested_message.should_encode(false) {
            Format::<
                Map::<Bytes, Nest>,
            >::encode(&self.map_string_nested_message, 71u32, buf)?;
        }
        if self.map_string_foreign_message.should_encode(false) {
            Format::<
                Map::<Bytes, Nest>,
            >::encode(&self.map_string_foreign_message, 72u32, buf)?;
        }
        if self.map_string_nested_enum.should_encode(false) {
            Format::<
                Map::<Bytes, Enum>,
            >::encode(&self.map_string_nested_enum, 73u32, buf)?;
        }
        if self.map_string_foreign_enum.should_encode(false) {
            Format::<
                Map::<Bytes, Enum>,
            >::encode(&self.map_string_foreign_enum, 74u32, buf)?;
        }
        if self.group_int32.should_encode(false) {
            Format::<VInt>::encode(&self.group_int32, 202u32, buf)?;
        }
        if self.group_uint32.should_encode(false) {
            Format::<VInt>::encode(&self.group_uint32, 203u32, buf)?;
        }
        if self.default_int32.should_encode(false) {
            Format::<VInt>::encode(&self.default_int32, 241u32, buf)?;
        }
        if self.default_int64.should_encode(false) {
            Format::<VInt>::encode(&self.default_int64, 242u32, buf)?;
        }
        if self.default_uint32.should_encode(false) {
            Format::<VInt>::encode(&self.default_uint32, 243u32, buf)?;
        }
        if self.default_uint64.should_encode(false) {
            Format::<VInt>::encode(&self.default_uint64, 244u32, buf)?;
        }
        if self.default_sint32.should_encode(false) {
            Format::<SInt>::encode(&self.default_sint32, 245u32, buf)?;
        }
        if self.default_sint64.should_encode(false) {
            Format::<SInt>::encode(&self.default_sint64, 246u32, buf)?;
        }
        if self.default_fixed32.should_encode(false) {
            Format::<Fix>::encode(&self.default_fixed32, 247u32, buf)?;
        }
        if self.default_fixed64.should_encode(false) {
            Format::<Fix>::encode(&self.default_fixed64, 248u32, buf)?;
        }
        if self.default_sfixed32.should_encode(false) {
            Format::<Fix>::encode(&self.default_sfixed32, 249u32, buf)?;
        }
        if self.default_sfixed64.should_encode(false) {
            Format::<Fix>::encode(&self.default_sfixed64, 250u32, buf)?;
        }
        if self.default_float.should_encode(false) {
            Format::<Fix>::encode(&self.default_float, 251u32, buf)?;
        }
        if self.default_double.should_encode(false) {
            Format::<Fix>::encode(&self.default_double, 252u32, buf)?;
        }
        if self.default_bool.should_encode(false) {
            Format::<Fix>::encode(&self.default_bool, 253u32, buf)?;
        }
        if self.default_string.should_encode(false) {
            Format::<Bytes>::encode(&self.default_string, 254u32, buf)?;
        }
        if self.default_bytes.should_encode(false) {
            Format::<Bytes>::encode(&self.default_bytes, 255u32, buf)?;
        }
        if self.fieldname1.should_encode(false) {
            Format::<VInt>::encode(&self.fieldname1, 401u32, buf)?;
        }
        if self.field_name2.should_encode(false) {
            Format::<VInt>::encode(&self.field_name2, 402u32, buf)?;
        }
        if self._field_name3.should_encode(false) {
            Format::<VInt>::encode(&self._field_name3, 403u32, buf)?;
        }
        if self.field__name4_.should_encode(false) {
            Format::<VInt>::encode(&self.field__name4_, 404u32, buf)?;
        }
        if self.field0name5.should_encode(false) {
            Format::<VInt>::encode(&self.field0name5, 405u32, buf)?;
        }
        if self.field_0_name6.should_encode(false) {
            Format::<VInt>::encode(&self.field_0_name6, 406u32, buf)?;
        }
        if self.fieldName7.should_encode(false) {
            Format::<VInt>::encode(&self.fieldName7, 407u32, buf)?;
        }
        if self.FieldName8.should_encode(false) {
            Format::<VInt>::encode(&self.FieldName8, 408u32, buf)?;
        }
        if self.field_Name9.should_encode(false) {
            Format::<VInt>::encode(&self.field_Name9, 409u32, buf)?;
        }
        if self.Field_Name10.should_encode(false) {
            Format::<VInt>::encode(&self.Field_Name10, 410u32, buf)?;
        }
        if self.FIELD_NAME11.should_encode(false) {
            Format::<VInt>::encode(&self.FIELD_NAME11, 411u32, buf)?;
        }
        if self.FIELD_name12.should_encode(false) {
            Format::<VInt>::encode(&self.FIELD_name12, 412u32, buf)?;
        }
        if self.__field_name13.should_encode(false) {
            Format::<VInt>::encode(&self.__field_name13, 413u32, buf)?;
        }
        if self.__Field_name14.should_encode(false) {
            Format::<VInt>::encode(&self.__Field_name14, 414u32, buf)?;
        }
        if self.field__name15.should_encode(false) {
            Format::<VInt>::encode(&self.field__name15, 415u32, buf)?;
        }
        if self.field__Name16.should_encode(false) {
            Format::<VInt>::encode(&self.field__Name16, 416u32, buf)?;
        }
        if self.field_name17__.should_encode(false) {
            Format::<VInt>::encode(&self.field_name17__, 417u32, buf)?;
        }
        if self.Field_name18__.should_encode(false) {
            Format::<VInt>::encode(&self.Field_name18__, 418u32, buf)?;
        }
        if self.extension_int32.should_encode(false) {
            Format::<VInt>::encode(&self.extension_int32, 120u32, buf)?;
        }
        match &self.oneof_field {
            TestAllTypesProto2OneOfOneofField::OneofUint32(value) => {
                Format::<VInt>::encode(value, 111u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofNestedMessage(value) => {
                Format::<Nest>::encode(value, 112u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofString(value) => {
                Format::<Bytes>::encode(value, 113u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofBytes(value) => {
                Format::<Bytes>::encode(value, 114u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofBool(value) => {
                Format::<Fix>::encode(value, 115u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofUint64(value) => {
                Format::<VInt>::encode(value, 116u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofFloat(value) => {
                Format::<Fix>::encode(value, 117u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofDouble(value) => {
                Format::<Fix>::encode(value, 118u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::OneofEnum(value) => {
                Format::<Enum>::encode(value, 119u32, buf)?;
            }
            TestAllTypesProto2OneOfOneofField::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum TestAllTypesProto2OneOfOneofField {
    OneofUint32(u32),
    OneofNestedMessage(TestAllTypesProto2NestedMessage),
    OneofString(String),
    OneofBytes(Vec<u8>),
    OneofBool(bool),
    OneofUint64(u64),
    OneofFloat(f32),
    OneofDouble(f64),
    OneofEnum(TestAllTypesProto2NestedEnum),
    Unknown(::core::marker::PhantomData<()>),
}
impl binformat::ShouldEncode for TestAllTypesProto2OneOfOneofField {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl Default for TestAllTypesProto2OneOfOneofField {
    fn default() -> Self {
        TestAllTypesProto2OneOfOneofField::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto2NestedMessage {
    pub a: Option<i32>,
    pub corecursive: Option<Box<TestAllTypesProto2>>,
    pub _unknown: (),
}
impl TestAllTypesProto2NestedMessage {
    #[inline(always)]
    pub fn r#with_a(mut self, it: i32) -> Self {
        self.r#set_a(it);
        self
    }
    #[inline(always)]
    pub fn r#set_a(&mut self, it: i32) -> &mut Self {
        self.a = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_corecursive(mut self, it: TestAllTypesProto2) -> Self {
        self.r#set_corecursive(it);
        self
    }
    #[inline(always)]
    pub fn r#set_corecursive(&mut self, it: TestAllTypesProto2) -> &mut Self {
        self.corecursive = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for TestAllTypesProto2NestedMessage {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("a") => {
                textformat::Field::merge(&mut self.a, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("corecursive") => {
                textformat::Field::merge(&mut self.corecursive, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto2NestedMessage {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.a != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("a: ");
            textformat::Field::format(&self.a, ctx, pad, out)?;
            out.push('\n');
        }
        if self.corecursive != <Option<Box<TestAllTypesProto2>> as Default>::default() {
            out.indent(pad);
            out.push_str("corecursive ");
            textformat::Field::format(&self.corecursive, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto2NestedMessage {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.a, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.a, buf)?;
            }
            18u32 => {
                buf = Format::<Nest>::decode(&mut self.corecursive, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto2NestedMessage {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.TestAllTypesProto2.NestedMessage"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.a.should_encode(false) {
            Format::<VInt>::encode(&self.a, 1u32, buf)?;
        }
        if self.corecursive.should_encode(false) {
            Format::<Nest>::encode(&self.corecursive, 2u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto2MessageSetCorrect {
    pub message_set_extension: Option<
        Box<TestAllTypesProto2MessageSetCorrectExtension2>,
    >,
    pub _unknown: (),
}
impl TestAllTypesProto2MessageSetCorrect {
    #[inline(always)]
    pub fn r#with_message_set_extension(
        mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension2,
    ) -> Self {
        self.r#set_message_set_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message_set_extension(
        &mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension2,
    ) -> &mut Self {
        self.message_set_extension = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for TestAllTypesProto2MessageSetCorrect {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Extended(
                "protobuf_test_messages.proto2.message_set_extension",
            ) => {
                textformat::Field::merge(&mut self.message_set_extension, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto2MessageSetCorrect {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.message_set_extension
            != <Option<
                Box<TestAllTypesProto2MessageSetCorrectExtension2>,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("[protobuf_test_messages.proto2.message_set_extension] ");
            textformat::Field::format(&self.message_set_extension, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto2MessageSetCorrect {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            33082498u32 => {
                buf = Format::<Nest>::decode(&mut self.message_set_extension, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto2MessageSetCorrect {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrect"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.message_set_extension.should_encode(false) {
            Format::<Nest>::encode(&self.message_set_extension, 4135312u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto2MessageSetCorrectExtension1 {
    pub message_set_extension: Option<
        Box<TestAllTypesProto2MessageSetCorrectExtension1>,
    >,
    pub str: Option<String>,
    pub _unknown: (),
}
impl TestAllTypesProto2MessageSetCorrectExtension1 {
    #[inline(always)]
    pub fn r#with_message_set_extension(
        mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension1,
    ) -> Self {
        self.r#set_message_set_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message_set_extension(
        &mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension1,
    ) -> &mut Self {
        self.message_set_extension = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_str(mut self, it: String) -> Self {
        self.r#set_str(it);
        self
    }
    #[inline(always)]
    pub fn r#set_str(&mut self, it: String) -> &mut Self {
        self.str = it.into();
        self
    }
}
impl textformat::Decodable for TestAllTypesProto2MessageSetCorrectExtension1 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("message_set_extension") => {
                textformat::Field::merge(&mut self.message_set_extension, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("str") => {
                textformat::Field::merge(&mut self.str, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto2MessageSetCorrectExtension1 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.message_set_extension
            != <Option<
                Box<TestAllTypesProto2MessageSetCorrectExtension1>,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("message_set_extension ");
            textformat::Field::format(&self.message_set_extension, ctx, pad, out)?;
            out.push('\n');
        }
        if self.str != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("str: ");
            textformat::Field::format(&self.str, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto2MessageSetCorrectExtension1 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            12382154u32 => {
                buf = Format::<Nest>::decode(&mut self.message_set_extension, buf)?;
            }
            202u32 => {
                buf = Format::<Bytes>::decode(&mut self.str, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto2MessageSetCorrectExtension1 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrectExtension1"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.message_set_extension.should_encode(false) {
            Format::<Nest>::encode(&self.message_set_extension, 1547769u32, buf)?;
        }
        if self.str.should_encode(false) {
            Format::<Bytes>::encode(&self.str, 25u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto2MessageSetCorrectExtension2 {
    pub message_set_extension: Option<
        Box<TestAllTypesProto2MessageSetCorrectExtension2>,
    >,
    pub i: Option<i32>,
    pub _unknown: (),
}
impl TestAllTypesProto2MessageSetCorrectExtension2 {
    #[inline(always)]
    pub fn r#with_message_set_extension(
        mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension2,
    ) -> Self {
        self.r#set_message_set_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message_set_extension(
        &mut self,
        it: TestAllTypesProto2MessageSetCorrectExtension2,
    ) -> &mut Self {
        self.message_set_extension = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_i(mut self, it: i32) -> Self {
        self.r#set_i(it);
        self
    }
    #[inline(always)]
    pub fn r#set_i(&mut self, it: i32) -> &mut Self {
        self.i = it.into();
        self
    }
}
impl textformat::Decodable for TestAllTypesProto2MessageSetCorrectExtension2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("message_set_extension") => {
                textformat::Field::merge(&mut self.message_set_extension, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("i") => {
                textformat::Field::merge(&mut self.i, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto2MessageSetCorrectExtension2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.message_set_extension
            != <Option<
                Box<TestAllTypesProto2MessageSetCorrectExtension2>,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("message_set_extension ");
            textformat::Field::format(&self.message_set_extension, ctx, pad, out)?;
            out.push('\n');
        }
        if self.i != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("i: ");
            textformat::Field::format(&self.i, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto2MessageSetCorrectExtension2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            33082498u32 => {
                buf = Format::<Nest>::decode(&mut self.message_set_extension, buf)?;
            }
            72u32 => {
                buf = Format::<VInt>::decode(&mut self.i, buf)?;
            }
            74u32 => {
                buf = Format::<VInt>::decode(&mut self.i, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto2MessageSetCorrectExtension2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.TestAllTypesProto2.MessageSetCorrectExtension2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.message_set_extension.should_encode(false) {
            Format::<Nest>::encode(&self.message_set_extension, 4135312u32, buf)?;
        }
        if self.i.should_encode(false) {
            Format::<VInt>::encode(&self.i, 9u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ForeignMessageProto2 {
    pub c: Option<i32>,
    pub _unknown: (),
}
impl ForeignMessageProto2 {
    #[inline(always)]
    pub fn r#with_c(mut self, it: i32) -> Self {
        self.r#set_c(it);
        self
    }
    #[inline(always)]
    pub fn r#set_c(&mut self, it: i32) -> &mut Self {
        self.c = it.into();
        self
    }
}
impl textformat::Decodable for ForeignMessageProto2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("c") => {
                textformat::Field::merge(&mut self.c, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ForeignMessageProto2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.c != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("c: ");
            textformat::Field::format(&self.c, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ForeignMessageProto2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.c, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.c, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ForeignMessageProto2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.ForeignMessageProto2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.c.should_encode(false) {
            Format::<VInt>::encode(&self.c, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnknownToTestAllTypes {
    pub optional_int32: Option<i32>,
    pub optional_string: Option<String>,
    pub nested_message: Option<Box<ForeignMessageProto2>>,
    pub a: Option<i32>,
    pub optional_bool: Option<bool>,
    pub repeated_int32: Vec<i32>,
    pub _unknown: (),
}
impl UnknownToTestAllTypes {
    #[inline(always)]
    pub fn r#with_optional_int32(mut self, it: i32) -> Self {
        self.r#set_optional_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_int32(&mut self, it: i32) -> &mut Self {
        self.optional_int32 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_string(mut self, it: String) -> Self {
        self.r#set_optional_string(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_string(&mut self, it: String) -> &mut Self {
        self.optional_string = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_nested_message(mut self, it: ForeignMessageProto2) -> Self {
        self.r#set_nested_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_nested_message(&mut self, it: ForeignMessageProto2) -> &mut Self {
        self.nested_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_a(mut self, it: i32) -> Self {
        self.r#set_a(it);
        self
    }
    #[inline(always)]
    pub fn r#set_a(&mut self, it: i32) -> &mut Self {
        self.a = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_bool(mut self, it: bool) -> Self {
        self.r#set_optional_bool(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_bool(&mut self, it: bool) -> &mut Self {
        self.optional_bool = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_int32(mut self, it: i32) -> Self {
        self.r#add_repeated_int32(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_int32(&mut self, it: i32) -> &mut Self {
        self.repeated_int32.push(it);
        self
    }
}
impl textformat::Decodable for UnknownToTestAllTypes {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("optional_int32") => {
                textformat::Field::merge(&mut self.optional_int32, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_string") => {
                textformat::Field::merge(&mut self.optional_string, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("nested_message") => {
                textformat::Field::merge(&mut self.nested_message, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("a") => {
                textformat::Field::merge(&mut self.a, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_bool") => {
                textformat::Field::merge(&mut self.optional_bool, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_int32") => {
                textformat::Field::merge(&mut self.repeated_int32, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UnknownToTestAllTypes {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.optional_int32 != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_int32: ");
            textformat::Field::format(&self.optional_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_string: ");
            textformat::Field::format(&self.optional_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.nested_message
            != <Option<Box<ForeignMessageProto2>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("nested_message ");
            textformat::Field::format(&self.nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.a != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("a: ");
            textformat::Field::format(&self.a, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bool != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bool: ");
            textformat::Field::format(&self.optional_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_int32 != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_int32: ");
            textformat::Field::format(&self.repeated_int32, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UnknownToTestAllTypes {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8008u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int32, buf)?;
            }
            8010u32 => {
                buf = Format::<VInt>::decode(&mut self.optional_int32, buf)?;
            }
            8018u32 => {
                buf = Format::<Bytes>::decode(&mut self.optional_string, buf)?;
            }
            8026u32 => {
                buf = Format::<Nest>::decode(&mut self.nested_message, buf)?;
            }
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.a, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.a, buf)?;
            }
            8048u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_bool, buf)?;
            }
            8050u32 => {
                buf = Format::<Fix>::decode(&mut self.optional_bool, buf)?;
            }
            8088u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.repeated_int32, buf)?;
            }
            8090u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.repeated_int32, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UnknownToTestAllTypes {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.UnknownToTestAllTypes"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.optional_int32.should_encode(false) {
            Format::<VInt>::encode(&self.optional_int32, 1001u32, buf)?;
        }
        if self.optional_string.should_encode(false) {
            Format::<Bytes>::encode(&self.optional_string, 1002u32, buf)?;
        }
        if self.nested_message.should_encode(false) {
            Format::<Nest>::encode(&self.nested_message, 1003u32, buf)?;
        }
        if self.a.should_encode(false) {
            Format::<VInt>::encode(&self.a, 1u32, buf)?;
        }
        if self.optional_bool.should_encode(false) {
            Format::<Fix>::encode(&self.optional_bool, 1006u32, buf)?;
        }
        if self.repeated_int32.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.repeated_int32, 1011u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NullHypothesisProto2 {
    pub _unknown: (),
}
impl NullHypothesisProto2 {}
impl textformat::Decodable for NullHypothesisProto2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for NullHypothesisProto2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        Ok(())
    }
}
impl binformat::Decodable for NullHypothesisProto2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for NullHypothesisProto2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.NullHypothesisProto2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumOnlyProto2 {
    pub _unknown: (),
}
impl EnumOnlyProto2 {}
impl textformat::Decodable for EnumOnlyProto2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumOnlyProto2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        Ok(())
    }
}
impl binformat::Decodable for EnumOnlyProto2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumOnlyProto2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.EnumOnlyProto2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OneStringProto2 {
    pub data: Option<String>,
    pub _unknown: (),
}
impl OneStringProto2 {
    #[inline(always)]
    pub fn r#with_data(mut self, it: String) -> Self {
        self.r#set_data(it);
        self
    }
    #[inline(always)]
    pub fn r#set_data(&mut self, it: String) -> &mut Self {
        self.data = it.into();
        self
    }
}
impl textformat::Decodable for OneStringProto2 {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("data") => {
                textformat::Field::merge(&mut self.data, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for OneStringProto2 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.data != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("data: ");
            textformat::Field::format(&self.data, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for OneStringProto2 {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.data, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for OneStringProto2 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.OneStringProto2"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.data.should_encode(false) {
            Format::<Bytes>::encode(&self.data, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProtoWithKeywords {
    pub inline: Option<i32>,
    pub concept: Option<String>,
    pub requires: Vec<String>,
    pub _unknown: (),
}
impl ProtoWithKeywords {
    #[inline(always)]
    pub fn r#with_inline(mut self, it: i32) -> Self {
        self.r#set_inline(it);
        self
    }
    #[inline(always)]
    pub fn r#set_inline(&mut self, it: i32) -> &mut Self {
        self.inline = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_concept(mut self, it: String) -> Self {
        self.r#set_concept(it);
        self
    }
    #[inline(always)]
    pub fn r#set_concept(&mut self, it: String) -> &mut Self {
        self.concept = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_requires(mut self, it: String) -> Self {
        self.r#add_requires(it);
        self
    }
    #[inline(always)]
    pub fn r#add_requires(&mut self, it: String) -> &mut Self {
        self.requires.push(it);
        self
    }
}
impl textformat::Decodable for ProtoWithKeywords {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("inline") => {
                textformat::Field::merge(&mut self.inline, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("concept") => {
                textformat::Field::merge(&mut self.concept, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("requires") => {
                textformat::Field::merge(&mut self.requires, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ProtoWithKeywords {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.inline != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("inline: ");
            textformat::Field::format(&self.inline, ctx, pad, out)?;
            out.push('\n');
        }
        if self.concept != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("concept: ");
            textformat::Field::format(&self.concept, ctx, pad, out)?;
            out.push('\n');
        }
        if self.requires != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("requires: ");
            textformat::Field::format(&self.requires, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ProtoWithKeywords {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.inline, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.inline, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.concept, buf)?;
            }
            26u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.requires, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ProtoWithKeywords {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto2.ProtoWithKeywords"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.inline.should_encode(false) {
            Format::<VInt>::encode(&self.inline, 1u32, buf)?;
        }
        if self.concept.should_encode(false) {
            Format::<Bytes>::encode(&self.concept, 2u32, buf)?;
        }
        if self.requires.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.requires, 3u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TestAllTypesProto2NestedEnum {
    FOO,
    BAR,
    BAZ,
    NEG,
    Unknown(u32),
}
impl Default for TestAllTypesProto2NestedEnum {
    fn default() -> TestAllTypesProto2NestedEnum {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for TestAllTypesProto2NestedEnum {}
impl binformat::ShouldEncode for TestAllTypesProto2NestedEnum {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for TestAllTypesProto2NestedEnum {
    fn from(v: u32) -> TestAllTypesProto2NestedEnum {
        match v {
            0u32 => TestAllTypesProto2NestedEnum::FOO,
            1u32 => TestAllTypesProto2NestedEnum::BAR,
            2u32 => TestAllTypesProto2NestedEnum::BAZ,
            4294967295u32 => TestAllTypesProto2NestedEnum::NEG,
            other => TestAllTypesProto2NestedEnum::Unknown(other),
        }
    }
}
impl From<TestAllTypesProto2NestedEnum> for u32 {
    fn from(v: TestAllTypesProto2NestedEnum) -> u32 {
        match v {
            TestAllTypesProto2NestedEnum::FOO => 0u32,
            TestAllTypesProto2NestedEnum::BAR => 1u32,
            TestAllTypesProto2NestedEnum::BAZ => 2u32,
            TestAllTypesProto2NestedEnum::NEG => 4294967295u32,
            TestAllTypesProto2NestedEnum::Unknown(other) => other,
        }
    }
}
impl textformat::Field for TestAllTypesProto2NestedEnum {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            TestAllTypesProto2NestedEnum::FOO => "FOO",
            TestAllTypesProto2NestedEnum::BAR => "BAR",
            TestAllTypesProto2NestedEnum::BAZ => "BAZ",
            TestAllTypesProto2NestedEnum::NEG => "NEG",
            TestAllTypesProto2NestedEnum::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("FOO") => {
                *self = TestAllTypesProto2NestedEnum::FOO;
            }
            textformat::ast::Literal::Identifier("BAR") => {
                *self = TestAllTypesProto2NestedEnum::BAR;
            }
            textformat::ast::Literal::Identifier("BAZ") => {
                *self = TestAllTypesProto2NestedEnum::BAZ;
            }
            textformat::ast::Literal::Identifier("NEG") => {
                *self = TestAllTypesProto2NestedEnum::NEG;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignEnumProto2 {
    FOREIGN_FOO,
    FOREIGN_BAR,
    FOREIGN_BAZ,
    Unknown(u32),
}
impl Default for ForeignEnumProto2 {
    fn default() -> ForeignEnumProto2 {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ForeignEnumProto2 {}
impl binformat::ShouldEncode for ForeignEnumProto2 {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for ForeignEnumProto2 {
    fn from(v: u32) -> ForeignEnumProto2 {
        match v {
            0u32 => ForeignEnumProto2::FOREIGN_FOO,
            1u32 => ForeignEnumProto2::FOREIGN_BAR,
            2u32 => ForeignEnumProto2::FOREIGN_BAZ,
            other => ForeignEnumProto2::Unknown(other),
        }
    }
}
impl From<ForeignEnumProto2> for u32 {
    fn from(v: ForeignEnumProto2) -> u32 {
        match v {
            ForeignEnumProto2::FOREIGN_FOO => 0u32,
            ForeignEnumProto2::FOREIGN_BAR => 1u32,
            ForeignEnumProto2::FOREIGN_BAZ => 2u32,
            ForeignEnumProto2::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ForeignEnumProto2 {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ForeignEnumProto2::FOREIGN_FOO => "FOREIGN_FOO",
            ForeignEnumProto2::FOREIGN_BAR => "FOREIGN_BAR",
            ForeignEnumProto2::FOREIGN_BAZ => "FOREIGN_BAZ",
            ForeignEnumProto2::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("FOREIGN_FOO") => {
                *self = ForeignEnumProto2::FOREIGN_FOO;
            }
            textformat::ast::Literal::Identifier("FOREIGN_BAR") => {
                *self = ForeignEnumProto2::FOREIGN_BAR;
            }
            textformat::ast::Literal::Identifier("FOREIGN_BAZ") => {
                *self = ForeignEnumProto2::FOREIGN_BAZ;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum EnumOnlyProto2Bool {
    kFalse,
    kTrue,
    Unknown(u32),
}
impl Default for EnumOnlyProto2Bool {
    fn default() -> EnumOnlyProto2Bool {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for EnumOnlyProto2Bool {}
impl binformat::ShouldEncode for EnumOnlyProto2Bool {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for EnumOnlyProto2Bool {
    fn from(v: u32) -> EnumOnlyProto2Bool {
        match v {
            0u32 => EnumOnlyProto2Bool::kFalse,
            1u32 => EnumOnlyProto2Bool::kTrue,
            other => EnumOnlyProto2Bool::Unknown(other),
        }
    }
}
impl From<EnumOnlyProto2Bool> for u32 {
    fn from(v: EnumOnlyProto2Bool) -> u32 {
        match v {
            EnumOnlyProto2Bool::kFalse => 0u32,
            EnumOnlyProto2Bool::kTrue => 1u32,
            EnumOnlyProto2Bool::Unknown(other) => other,
        }
    }
}
impl textformat::Field for EnumOnlyProto2Bool {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            EnumOnlyProto2Bool::kFalse => "kFalse",
            EnumOnlyProto2Bool::kTrue => "kTrue",
            EnumOnlyProto2Bool::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("kFalse") => {
                *self = EnumOnlyProto2Bool::kFalse;
            }
            textformat::ast::Literal::Identifier("kTrue") => {
                *self = EnumOnlyProto2Bool::kTrue;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
