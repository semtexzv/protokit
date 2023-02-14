#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::any::*;
use super::super::super::google::protobuf::duration::*;
use root::types::field_mask::*;
use super::super::super::google::protobuf::r#struct::*;
use root::types::timestamp::*;
use super::super::super::google::protobuf::wrappers::*;
use root::types::field_mask::*;
use super::super::super::google::protobuf::duration::*;
use super::super::super::google::protobuf::wrappers::*;
use super::super::super::google::protobuf::r#struct::*;
use root::types::timestamp::*;
use root::types::any::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&TestAllTypesProto3::default());
    registry.register(&TestAllTypesProto3NestedMessage::default());
    registry.register(&ForeignMessage::default());
    registry.register(&NullHypothesisProto3::default());
    registry.register(&EnumOnlyProto3::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto3 {
    pub optional_int32: i32,
    pub optional_int64: i64,
    pub optional_uint32: u32,
    pub optional_uint64: u64,
    pub optional_sint32: i32,
    pub optional_sint64: i64,
    pub optional_fixed32: u32,
    pub optional_fixed64: u64,
    pub optional_sfixed32: i32,
    pub optional_sfixed64: i64,
    pub optional_float: f32,
    pub optional_double: f64,
    pub optional_bool: bool,
    pub optional_string: String,
    pub optional_bytes: Vec<u8>,
    pub optional_nested_message: Option<Box<TestAllTypesProto3NestedMessage>>,
    pub optional_foreign_message: Option<Box<ForeignMessage>>,
    pub optional_nested_enum: TestAllTypesProto3NestedEnum,
    pub optional_foreign_enum: ForeignEnum,
    pub optional_aliased_enum: TestAllTypesProto3AliasedEnum,
    pub optional_string_piece: String,
    pub optional_cord: String,
    pub recursive_message: Option<Box<TestAllTypesProto3>>,
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
    pub repeated_nested_message: Vec<TestAllTypesProto3NestedMessage>,
    pub repeated_foreign_message: Vec<ForeignMessage>,
    pub repeated_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
    pub repeated_foreign_enum: Vec<ForeignEnum>,
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
    pub packed_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
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
    pub unpacked_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
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
        TestAllTypesProto3NestedMessage,
    >,
    pub map_string_foreign_message: ::std::collections::HashMap<String, ForeignMessage>,
    pub map_string_nested_enum: ::std::collections::HashMap<
        String,
        TestAllTypesProto3NestedEnum,
    >,
    pub map_string_foreign_enum: ::std::collections::HashMap<String, ForeignEnum>,
    pub optional_bool_wrapper: Option<Box<BoolValue>>,
    pub optional_int32_wrapper: Option<Box<Int32Value>>,
    pub optional_int64_wrapper: Option<Box<Int64Value>>,
    pub optional_uint32_wrapper: Option<Box<UInt32Value>>,
    pub optional_uint64_wrapper: Option<Box<UInt64Value>>,
    pub optional_float_wrapper: Option<Box<FloatValue>>,
    pub optional_double_wrapper: Option<Box<DoubleValue>>,
    pub optional_string_wrapper: Option<Box<StringValue>>,
    pub optional_bytes_wrapper: Option<Box<BytesValue>>,
    pub repeated_bool_wrapper: Vec<BoolValue>,
    pub repeated_int32_wrapper: Vec<Int32Value>,
    pub repeated_int64_wrapper: Vec<Int64Value>,
    pub repeated_uint32_wrapper: Vec<UInt32Value>,
    pub repeated_uint64_wrapper: Vec<UInt64Value>,
    pub repeated_float_wrapper: Vec<FloatValue>,
    pub repeated_double_wrapper: Vec<DoubleValue>,
    pub repeated_string_wrapper: Vec<StringValue>,
    pub repeated_bytes_wrapper: Vec<BytesValue>,
    pub optional_duration: Option<Box<Duration>>,
    pub optional_timestamp: Option<Box<Timestamp>>,
    pub optional_field_mask: Option<Box<FieldMask>>,
    pub optional_struct: Option<Box<Struct>>,
    pub optional_any: Option<Box<Any>>,
    pub optional_value: Option<Box<Value>>,
    pub optional_null_value: NullValue,
    pub repeated_duration: Vec<Duration>,
    pub repeated_timestamp: Vec<Timestamp>,
    pub repeated_fieldmask: Vec<FieldMask>,
    pub repeated_struct: Vec<Struct>,
    pub repeated_any: Vec<Any>,
    pub repeated_value: Vec<Value>,
    pub repeated_list_value: Vec<ListValue>,
    pub fieldname1: i32,
    pub field_name2: i32,
    pub _field_name3: i32,
    pub field__name4_: i32,
    pub field0name5: i32,
    pub field_0_name6: i32,
    pub fieldName7: i32,
    pub FieldName8: i32,
    pub field_Name9: i32,
    pub Field_Name10: i32,
    pub FIELD_NAME11: i32,
    pub FIELD_name12: i32,
    pub __field_name13: i32,
    pub __Field_name14: i32,
    pub field__name15: i32,
    pub field__Name16: i32,
    pub field_name17__: i32,
    pub Field_name18__: i32,
    pub oneof_field: TestAllTypesProto3OneOfOneofField,
    pub _unknown: (),
}
impl TestAllTypesProto3 {
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
        it: TestAllTypesProto3NestedMessage,
    ) -> Self {
        self.r#set_optional_nested_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_nested_message(
        &mut self,
        it: TestAllTypesProto3NestedMessage,
    ) -> &mut Self {
        self.optional_nested_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_foreign_message(mut self, it: ForeignMessage) -> Self {
        self.r#set_optional_foreign_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_foreign_message(&mut self, it: ForeignMessage) -> &mut Self {
        self.optional_foreign_message = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_nested_enum(
        mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.r#set_optional_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_nested_enum(
        &mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> &mut Self {
        self.optional_nested_enum = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_foreign_enum(mut self, it: ForeignEnum) -> Self {
        self.r#set_optional_foreign_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_foreign_enum(&mut self, it: ForeignEnum) -> &mut Self {
        self.optional_foreign_enum = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_aliased_enum(
        mut self,
        it: TestAllTypesProto3AliasedEnum,
    ) -> Self {
        self.r#set_optional_aliased_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_aliased_enum(
        &mut self,
        it: TestAllTypesProto3AliasedEnum,
    ) -> &mut Self {
        self.optional_aliased_enum = it.into();
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
    pub fn r#with_recursive_message(mut self, it: TestAllTypesProto3) -> Self {
        self.r#set_recursive_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_recursive_message(&mut self, it: TestAllTypesProto3) -> &mut Self {
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
        it: TestAllTypesProto3NestedMessage,
    ) -> Self {
        self.r#add_repeated_nested_message(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_nested_message(
        &mut self,
        it: TestAllTypesProto3NestedMessage,
    ) -> &mut Self {
        self.repeated_nested_message.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_foreign_message(mut self, it: ForeignMessage) -> Self {
        self.r#add_repeated_foreign_message(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_foreign_message(&mut self, it: ForeignMessage) -> &mut Self {
        self.repeated_foreign_message.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_nested_enum(
        mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.r#add_repeated_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_nested_enum(
        &mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> &mut Self {
        self.repeated_nested_enum.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_foreign_enum(mut self, it: ForeignEnum) -> Self {
        self.r#add_repeated_foreign_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_foreign_enum(&mut self, it: ForeignEnum) -> &mut Self {
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
        it: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.r#add_packed_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_packed_nested_enum(
        &mut self,
        it: TestAllTypesProto3NestedEnum,
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
        it: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.r#add_unpacked_nested_enum(it);
        self
    }
    #[inline(always)]
    pub fn r#add_unpacked_nested_enum(
        &mut self,
        it: TestAllTypesProto3NestedEnum,
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
        v: TestAllTypesProto3NestedMessage,
    ) -> Self {
        self.r#add_map_string_nested_message(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_nested_message(
        &mut self,
        k: String,
        v: TestAllTypesProto3NestedMessage,
    ) -> &mut Self {
        let _ = self.map_string_nested_message.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_foreign_message(
        mut self,
        k: String,
        v: ForeignMessage,
    ) -> Self {
        self.r#add_map_string_foreign_message(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_foreign_message(
        &mut self,
        k: String,
        v: ForeignMessage,
    ) -> &mut Self {
        let _ = self.map_string_foreign_message.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_nested_enum(
        mut self,
        k: String,
        v: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.r#add_map_string_nested_enum(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_nested_enum(
        &mut self,
        k: String,
        v: TestAllTypesProto3NestedEnum,
    ) -> &mut Self {
        let _ = self.map_string_nested_enum.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_map_string_foreign_enum(mut self, k: String, v: ForeignEnum) -> Self {
        self.r#add_map_string_foreign_enum(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_map_string_foreign_enum(
        &mut self,
        k: String,
        v: ForeignEnum,
    ) -> &mut Self {
        let _ = self.map_string_foreign_enum.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_optional_bool_wrapper(mut self, it: BoolValue) -> Self {
        self.r#set_optional_bool_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_bool_wrapper(&mut self, it: BoolValue) -> &mut Self {
        self.optional_bool_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_int32_wrapper(mut self, it: Int32Value) -> Self {
        self.r#set_optional_int32_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_int32_wrapper(&mut self, it: Int32Value) -> &mut Self {
        self.optional_int32_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_int64_wrapper(mut self, it: Int64Value) -> Self {
        self.r#set_optional_int64_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_int64_wrapper(&mut self, it: Int64Value) -> &mut Self {
        self.optional_int64_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_uint32_wrapper(mut self, it: UInt32Value) -> Self {
        self.r#set_optional_uint32_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_uint32_wrapper(&mut self, it: UInt32Value) -> &mut Self {
        self.optional_uint32_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_uint64_wrapper(mut self, it: UInt64Value) -> Self {
        self.r#set_optional_uint64_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_uint64_wrapper(&mut self, it: UInt64Value) -> &mut Self {
        self.optional_uint64_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_float_wrapper(mut self, it: FloatValue) -> Self {
        self.r#set_optional_float_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_float_wrapper(&mut self, it: FloatValue) -> &mut Self {
        self.optional_float_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_double_wrapper(mut self, it: DoubleValue) -> Self {
        self.r#set_optional_double_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_double_wrapper(&mut self, it: DoubleValue) -> &mut Self {
        self.optional_double_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_string_wrapper(mut self, it: StringValue) -> Self {
        self.r#set_optional_string_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_string_wrapper(&mut self, it: StringValue) -> &mut Self {
        self.optional_string_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_bytes_wrapper(mut self, it: BytesValue) -> Self {
        self.r#set_optional_bytes_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_bytes_wrapper(&mut self, it: BytesValue) -> &mut Self {
        self.optional_bytes_wrapper = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_bool_wrapper(mut self, it: BoolValue) -> Self {
        self.r#add_repeated_bool_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_bool_wrapper(&mut self, it: BoolValue) -> &mut Self {
        self.repeated_bool_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_int32_wrapper(mut self, it: Int32Value) -> Self {
        self.r#add_repeated_int32_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_int32_wrapper(&mut self, it: Int32Value) -> &mut Self {
        self.repeated_int32_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_int64_wrapper(mut self, it: Int64Value) -> Self {
        self.r#add_repeated_int64_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_int64_wrapper(&mut self, it: Int64Value) -> &mut Self {
        self.repeated_int64_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_uint32_wrapper(mut self, it: UInt32Value) -> Self {
        self.r#add_repeated_uint32_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_uint32_wrapper(&mut self, it: UInt32Value) -> &mut Self {
        self.repeated_uint32_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_uint64_wrapper(mut self, it: UInt64Value) -> Self {
        self.r#add_repeated_uint64_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_uint64_wrapper(&mut self, it: UInt64Value) -> &mut Self {
        self.repeated_uint64_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_float_wrapper(mut self, it: FloatValue) -> Self {
        self.r#add_repeated_float_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_float_wrapper(&mut self, it: FloatValue) -> &mut Self {
        self.repeated_float_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_double_wrapper(mut self, it: DoubleValue) -> Self {
        self.r#add_repeated_double_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_double_wrapper(&mut self, it: DoubleValue) -> &mut Self {
        self.repeated_double_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_string_wrapper(mut self, it: StringValue) -> Self {
        self.r#add_repeated_string_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_string_wrapper(&mut self, it: StringValue) -> &mut Self {
        self.repeated_string_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_bytes_wrapper(mut self, it: BytesValue) -> Self {
        self.r#add_repeated_bytes_wrapper(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_bytes_wrapper(&mut self, it: BytesValue) -> &mut Self {
        self.repeated_bytes_wrapper.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_optional_duration(mut self, it: Duration) -> Self {
        self.r#set_optional_duration(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_duration(&mut self, it: Duration) -> &mut Self {
        self.optional_duration = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_timestamp(mut self, it: Timestamp) -> Self {
        self.r#set_optional_timestamp(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_timestamp(&mut self, it: Timestamp) -> &mut Self {
        self.optional_timestamp = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_field_mask(mut self, it: FieldMask) -> Self {
        self.r#set_optional_field_mask(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_field_mask(&mut self, it: FieldMask) -> &mut Self {
        self.optional_field_mask = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_struct(mut self, it: Struct) -> Self {
        self.r#set_optional_struct(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_struct(&mut self, it: Struct) -> &mut Self {
        self.optional_struct = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_any(mut self, it: Any) -> Self {
        self.r#set_optional_any(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_any(&mut self, it: Any) -> &mut Self {
        self.optional_any = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_value(mut self, it: Value) -> Self {
        self.r#set_optional_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_value(&mut self, it: Value) -> &mut Self {
        self.optional_value = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_optional_null_value(mut self, it: NullValue) -> Self {
        self.r#set_optional_null_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optional_null_value(&mut self, it: NullValue) -> &mut Self {
        self.optional_null_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_duration(mut self, it: Duration) -> Self {
        self.r#add_repeated_duration(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_duration(&mut self, it: Duration) -> &mut Self {
        self.repeated_duration.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_timestamp(mut self, it: Timestamp) -> Self {
        self.r#add_repeated_timestamp(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_timestamp(&mut self, it: Timestamp) -> &mut Self {
        self.repeated_timestamp.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_fieldmask(mut self, it: FieldMask) -> Self {
        self.r#add_repeated_fieldmask(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_fieldmask(&mut self, it: FieldMask) -> &mut Self {
        self.repeated_fieldmask.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_struct(mut self, it: Struct) -> Self {
        self.r#add_repeated_struct(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_struct(&mut self, it: Struct) -> &mut Self {
        self.repeated_struct.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_any(mut self, it: Any) -> Self {
        self.r#add_repeated_any(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_any(&mut self, it: Any) -> &mut Self {
        self.repeated_any.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_value(mut self, it: Value) -> Self {
        self.r#add_repeated_value(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_value(&mut self, it: Value) -> &mut Self {
        self.repeated_value.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_repeated_list_value(mut self, it: ListValue) -> Self {
        self.r#add_repeated_list_value(it);
        self
    }
    #[inline(always)]
    pub fn r#add_repeated_list_value(&mut self, it: ListValue) -> &mut Self {
        self.repeated_list_value.push(it);
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
    pub fn r#with_oneof_field_oneof_uint32(mut self, it: u32) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint32(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_uint32(&mut self, it: u32) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint32(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_nested_message(
        mut self,
        it: TestAllTypesProto3NestedMessage,
    ) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofNestedMessage(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_nested_message(
        &mut self,
        it: TestAllTypesProto3NestedMessage,
    ) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofNestedMessage(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_string(mut self, it: String) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofString(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_string(&mut self, it: String) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofString(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_bytes(mut self, it: Vec<u8>) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_bytes(&mut self, it: Vec<u8>) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBytes(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_bool(mut self, it: bool) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBool(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_bool(&mut self, it: bool) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBool(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_uint64(mut self, it: u64) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint64(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_uint64(&mut self, it: u64) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint64(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_float(mut self, it: f32) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofFloat(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_float(&mut self, it: f32) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofFloat(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_double(mut self, it: f64) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofDouble(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_double(&mut self, it: f64) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofDouble(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_enum(
        mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofEnum(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_enum(
        &mut self,
        it: TestAllTypesProto3NestedEnum,
    ) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofEnum(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_field_oneof_null_value(mut self, it: NullValue) -> Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofNullValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_field_oneof_null_value(&mut self, it: NullValue) -> &mut Self {
        self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofNullValue(it);
        self
    }
}
impl textformat::Decodable for TestAllTypesProto3 {
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
            textformat::ast::FieldName::Normal("optional_aliased_enum") => {
                textformat::Field::merge(&mut self.optional_aliased_enum, ctx, value)?;
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
            textformat::ast::FieldName::Normal("optional_bool_wrapper") => {
                textformat::Field::merge(&mut self.optional_bool_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_int32_wrapper") => {
                textformat::Field::merge(&mut self.optional_int32_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_int64_wrapper") => {
                textformat::Field::merge(&mut self.optional_int64_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_uint32_wrapper") => {
                textformat::Field::merge(&mut self.optional_uint32_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_uint64_wrapper") => {
                textformat::Field::merge(&mut self.optional_uint64_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_float_wrapper") => {
                textformat::Field::merge(&mut self.optional_float_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_double_wrapper") => {
                textformat::Field::merge(&mut self.optional_double_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_string_wrapper") => {
                textformat::Field::merge(&mut self.optional_string_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_bytes_wrapper") => {
                textformat::Field::merge(&mut self.optional_bytes_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_bool_wrapper") => {
                textformat::Field::merge(&mut self.repeated_bool_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_int32_wrapper") => {
                textformat::Field::merge(&mut self.repeated_int32_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_int64_wrapper") => {
                textformat::Field::merge(&mut self.repeated_int64_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_uint32_wrapper") => {
                textformat::Field::merge(&mut self.repeated_uint32_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_uint64_wrapper") => {
                textformat::Field::merge(&mut self.repeated_uint64_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_float_wrapper") => {
                textformat::Field::merge(&mut self.repeated_float_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_double_wrapper") => {
                textformat::Field::merge(&mut self.repeated_double_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_string_wrapper") => {
                textformat::Field::merge(&mut self.repeated_string_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_bytes_wrapper") => {
                textformat::Field::merge(&mut self.repeated_bytes_wrapper, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_duration") => {
                textformat::Field::merge(&mut self.optional_duration, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_timestamp") => {
                textformat::Field::merge(&mut self.optional_timestamp, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_field_mask") => {
                textformat::Field::merge(&mut self.optional_field_mask, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_struct") => {
                textformat::Field::merge(&mut self.optional_struct, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_any") => {
                textformat::Field::merge(&mut self.optional_any, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_value") => {
                textformat::Field::merge(&mut self.optional_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optional_null_value") => {
                textformat::Field::merge(&mut self.optional_null_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_duration") => {
                textformat::Field::merge(&mut self.repeated_duration, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_timestamp") => {
                textformat::Field::merge(&mut self.repeated_timestamp, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_fieldmask") => {
                textformat::Field::merge(&mut self.repeated_fieldmask, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_struct") => {
                textformat::Field::merge(&mut self.repeated_struct, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_any") => {
                textformat::Field::merge(&mut self.repeated_any, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_value") => {
                textformat::Field::merge(&mut self.repeated_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("repeated_list_value") => {
                textformat::Field::merge(&mut self.repeated_list_value, ctx, value)?;
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
            textformat::ast::FieldName::Normal("oneof_uint32") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint32(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_nested_message") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofNestedMessage(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_string") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofString(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_bytes") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBytes(target);
            }
            textformat::ast::FieldName::Normal("oneof_bool") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBool(target);
            }
            textformat::ast::FieldName::Normal("oneof_uint64") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint64(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_float") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofFloat(target);
            }
            textformat::ast::FieldName::Normal("oneof_double") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofDouble(
                    target,
                );
            }
            textformat::ast::FieldName::Normal("oneof_enum") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofEnum(target);
            }
            textformat::ast::FieldName::Normal("oneof_null_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofNullValue(
                    target,
                );
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TestAllTypesProto3 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.optional_int32 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_int32: ");
            textformat::Field::format(&self.optional_int32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_int64 != <i64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_int64: ");
            textformat::Field::format(&self.optional_int64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint32 != <u32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_uint32: ");
            textformat::Field::format(&self.optional_uint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint64 != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_uint64: ");
            textformat::Field::format(&self.optional_uint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sint32 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sint32: ");
            textformat::Field::format(&self.optional_sint32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sint64 != <i64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sint64: ");
            textformat::Field::format(&self.optional_sint64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_fixed32 != <u32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_fixed32: ");
            textformat::Field::format(&self.optional_fixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_fixed64 != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_fixed64: ");
            textformat::Field::format(&self.optional_fixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sfixed32 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sfixed32: ");
            textformat::Field::format(&self.optional_sfixed32, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_sfixed64 != <i64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_sfixed64: ");
            textformat::Field::format(&self.optional_sfixed64, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_float != <f32 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_float: ");
            textformat::Field::format(&self.optional_float, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_double != <f64 as Default>::default() {
            out.indent(pad);
            out.push_str("optional_double: ");
            textformat::Field::format(&self.optional_double, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bool != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bool: ");
            textformat::Field::format(&self.optional_bool, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string != <String as Default>::default() {
            out.indent(pad);
            out.push_str("optional_string: ");
            textformat::Field::format(&self.optional_string, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bytes != <Vec<u8> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bytes: ");
            textformat::Field::format(&self.optional_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_nested_message
            != <Option<Box<TestAllTypesProto3NestedMessage>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_nested_message ");
            textformat::Field::format(&self.optional_nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_foreign_message
            != <Option<Box<ForeignMessage>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_foreign_message ");
            textformat::Field::format(&self.optional_foreign_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_nested_enum
            != <TestAllTypesProto3NestedEnum as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_nested_enum: ");
            textformat::Field::format(&self.optional_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_foreign_enum != <ForeignEnum as Default>::default() {
            out.indent(pad);
            out.push_str("optional_foreign_enum: ");
            textformat::Field::format(&self.optional_foreign_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_aliased_enum
            != <TestAllTypesProto3AliasedEnum as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_aliased_enum: ");
            textformat::Field::format(&self.optional_aliased_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string_piece != <String as Default>::default() {
            out.indent(pad);
            out.push_str("optional_string_piece: ");
            textformat::Field::format(&self.optional_string_piece, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_cord != <String as Default>::default() {
            out.indent(pad);
            out.push_str("optional_cord: ");
            textformat::Field::format(&self.optional_cord, ctx, pad, out)?;
            out.push('\n');
        }
        if self.recursive_message
            != <Option<Box<TestAllTypesProto3>> as Default>::default()
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
            != <Vec<TestAllTypesProto3NestedMessage> as Default>::default()
        {
            out.indent(pad);
            out.push_str("repeated_nested_message ");
            textformat::Field::format(&self.repeated_nested_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_foreign_message != <Vec<ForeignMessage> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_foreign_message ");
            textformat::Field::format(&self.repeated_foreign_message, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_nested_enum
            != <Vec<TestAllTypesProto3NestedEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("repeated_nested_enum: ");
            textformat::Field::format(&self.repeated_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_foreign_enum != <Vec<ForeignEnum> as Default>::default() {
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
            != <Vec<TestAllTypesProto3NestedEnum> as Default>::default()
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
            != <Vec<TestAllTypesProto3NestedEnum> as Default>::default()
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
                TestAllTypesProto3NestedMessage,
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
                ForeignMessage,
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
                TestAllTypesProto3NestedEnum,
            > as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_nested_enum ");
            textformat::Field::format(&self.map_string_nested_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_string_foreign_enum
            != <::std::collections::HashMap<String, ForeignEnum> as Default>::default()
        {
            out.indent(pad);
            out.push_str("map_string_foreign_enum ");
            textformat::Field::format(&self.map_string_foreign_enum, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bool_wrapper != <Option<Box<BoolValue>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_bool_wrapper ");
            textformat::Field::format(&self.optional_bool_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_int32_wrapper != <Option<Box<Int32Value>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_int32_wrapper ");
            textformat::Field::format(&self.optional_int32_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_int64_wrapper != <Option<Box<Int64Value>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_int64_wrapper ");
            textformat::Field::format(&self.optional_int64_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint32_wrapper
            != <Option<Box<UInt32Value>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_uint32_wrapper ");
            textformat::Field::format(&self.optional_uint32_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_uint64_wrapper
            != <Option<Box<UInt64Value>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_uint64_wrapper ");
            textformat::Field::format(&self.optional_uint64_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_float_wrapper != <Option<Box<FloatValue>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_float_wrapper ");
            textformat::Field::format(&self.optional_float_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_double_wrapper
            != <Option<Box<DoubleValue>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_double_wrapper ");
            textformat::Field::format(&self.optional_double_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_string_wrapper
            != <Option<Box<StringValue>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_string_wrapper ");
            textformat::Field::format(&self.optional_string_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_bytes_wrapper != <Option<Box<BytesValue>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("optional_bytes_wrapper ");
            textformat::Field::format(&self.optional_bytes_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_bool_wrapper != <Vec<BoolValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_bool_wrapper ");
            textformat::Field::format(&self.repeated_bool_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_int32_wrapper != <Vec<Int32Value> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_int32_wrapper ");
            textformat::Field::format(&self.repeated_int32_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_int64_wrapper != <Vec<Int64Value> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_int64_wrapper ");
            textformat::Field::format(&self.repeated_int64_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_uint32_wrapper != <Vec<UInt32Value> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_uint32_wrapper ");
            textformat::Field::format(&self.repeated_uint32_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_uint64_wrapper != <Vec<UInt64Value> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_uint64_wrapper ");
            textformat::Field::format(&self.repeated_uint64_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_float_wrapper != <Vec<FloatValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_float_wrapper ");
            textformat::Field::format(&self.repeated_float_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_double_wrapper != <Vec<DoubleValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_double_wrapper ");
            textformat::Field::format(&self.repeated_double_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_string_wrapper != <Vec<StringValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_string_wrapper ");
            textformat::Field::format(&self.repeated_string_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_bytes_wrapper != <Vec<BytesValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_bytes_wrapper ");
            textformat::Field::format(&self.repeated_bytes_wrapper, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_duration != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_duration ");
            textformat::Field::format(&self.optional_duration, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_timestamp != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_timestamp ");
            textformat::Field::format(&self.optional_timestamp, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_field_mask != <Option<Box<FieldMask>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_field_mask ");
            textformat::Field::format(&self.optional_field_mask, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_struct != <Option<Box<Struct>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_struct ");
            textformat::Field::format(&self.optional_struct, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_any != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_any ");
            textformat::Field::format(&self.optional_any, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_value != <Option<Box<Value>> as Default>::default() {
            out.indent(pad);
            out.push_str("optional_value ");
            textformat::Field::format(&self.optional_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optional_null_value != <NullValue as Default>::default() {
            out.indent(pad);
            out.push_str("optional_null_value: ");
            textformat::Field::format(&self.optional_null_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_duration != <Vec<Duration> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_duration ");
            textformat::Field::format(&self.repeated_duration, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_timestamp != <Vec<Timestamp> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_timestamp ");
            textformat::Field::format(&self.repeated_timestamp, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_fieldmask != <Vec<FieldMask> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_fieldmask ");
            textformat::Field::format(&self.repeated_fieldmask, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_struct != <Vec<Struct> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_struct ");
            textformat::Field::format(&self.repeated_struct, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_any != <Vec<Any> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_any ");
            textformat::Field::format(&self.repeated_any, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_value != <Vec<Value> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_value ");
            textformat::Field::format(&self.repeated_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.repeated_list_value != <Vec<ListValue> as Default>::default() {
            out.indent(pad);
            out.push_str("repeated_list_value ");
            textformat::Field::format(&self.repeated_list_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.fieldname1 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("fieldname1: ");
            textformat::Field::format(&self.fieldname1, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_name2 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field_name2: ");
            textformat::Field::format(&self.field_name2, ctx, pad, out)?;
            out.push('\n');
        }
        if self._field_name3 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("_field_name3: ");
            textformat::Field::format(&self._field_name3, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__name4_ != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field__name4_: ");
            textformat::Field::format(&self.field__name4_, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field0name5 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field0name5: ");
            textformat::Field::format(&self.field0name5, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_0_name6 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field_0_name6: ");
            textformat::Field::format(&self.field_0_name6, ctx, pad, out)?;
            out.push('\n');
        }
        if self.fieldName7 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("fieldName7: ");
            textformat::Field::format(&self.fieldName7, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FieldName8 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("FieldName8: ");
            textformat::Field::format(&self.FieldName8, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_Name9 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field_Name9: ");
            textformat::Field::format(&self.field_Name9, ctx, pad, out)?;
            out.push('\n');
        }
        if self.Field_Name10 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("Field_Name10: ");
            textformat::Field::format(&self.Field_Name10, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FIELD_NAME11 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("FIELD_NAME11: ");
            textformat::Field::format(&self.FIELD_NAME11, ctx, pad, out)?;
            out.push('\n');
        }
        if self.FIELD_name12 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("FIELD_name12: ");
            textformat::Field::format(&self.FIELD_name12, ctx, pad, out)?;
            out.push('\n');
        }
        if self.__field_name13 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("__field_name13: ");
            textformat::Field::format(&self.__field_name13, ctx, pad, out)?;
            out.push('\n');
        }
        if self.__Field_name14 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("__Field_name14: ");
            textformat::Field::format(&self.__Field_name14, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__name15 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field__name15: ");
            textformat::Field::format(&self.field__name15, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field__Name16 != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field__Name16: ");
            textformat::Field::format(&self.field__Name16, ctx, pad, out)?;
            out.push('\n');
        }
        if self.field_name17__ != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("field_name17__: ");
            textformat::Field::format(&self.field_name17__, ctx, pad, out)?;
            out.push('\n');
        }
        if self.Field_name18__ != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("Field_name18__: ");
            textformat::Field::format(&self.Field_name18__, ctx, pad, out)?;
            out.push('\n');
        }
        match &self.oneof_field {
            TestAllTypesProto3OneOfOneofField::OneofUint32(value) => {
                out.indent(pad);
                out.push_str("oneof_uint32: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofNestedMessage(value) => {
                out.indent(pad);
                out.push_str("oneof_nested_message ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofString(value) => {
                out.indent(pad);
                out.push_str("oneof_string: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofBytes(value) => {
                out.indent(pad);
                out.push_str("oneof_bytes: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofBool(value) => {
                out.indent(pad);
                out.push_str("oneof_bool: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofUint64(value) => {
                out.indent(pad);
                out.push_str("oneof_uint64: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofFloat(value) => {
                out.indent(pad);
                out.push_str("oneof_float: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofDouble(value) => {
                out.indent(pad);
                out.push_str("oneof_double: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofEnum(value) => {
                out.indent(pad);
                out.push_str("oneof_enum: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::OneofNullValue(value) => {
                out.indent(pad);
                out.push_str("oneof_null_value: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            TestAllTypesProto3OneOfOneofField::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto3 {
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
            184u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_aliased_enum, buf)?;
            }
            186u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_aliased_enum, buf)?;
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
            1610u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_bool_wrapper, buf)?;
            }
            1618u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_int32_wrapper, buf)?;
            }
            1626u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_int64_wrapper, buf)?;
            }
            1634u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_uint32_wrapper, buf)?;
            }
            1642u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_uint64_wrapper, buf)?;
            }
            1650u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_float_wrapper, buf)?;
            }
            1658u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_double_wrapper, buf)?;
            }
            1666u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_string_wrapper, buf)?;
            }
            1674u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_bytes_wrapper, buf)?;
            }
            1690u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_bool_wrapper, buf)?;
            }
            1698u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_int32_wrapper, buf)?;
            }
            1706u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_int64_wrapper, buf)?;
            }
            1714u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_uint32_wrapper, buf)?;
            }
            1722u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_uint64_wrapper, buf)?;
            }
            1730u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_float_wrapper, buf)?;
            }
            1738u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_double_wrapper, buf)?;
            }
            1746u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_string_wrapper, buf)?;
            }
            1754u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_bytes_wrapper, buf)?;
            }
            2410u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_duration, buf)?;
            }
            2418u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_timestamp, buf)?;
            }
            2426u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_field_mask, buf)?;
            }
            2434u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_struct, buf)?;
            }
            2442u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_any, buf)?;
            }
            2450u32 => {
                buf = Format::<Nest>::decode(&mut self.optional_value, buf)?;
            }
            2456u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_null_value, buf)?;
            }
            2458u32 => {
                buf = Format::<Enum>::decode(&mut self.optional_null_value, buf)?;
            }
            2490u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_duration, buf)?;
            }
            2498u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_timestamp, buf)?;
            }
            2506u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_fieldmask, buf)?;
            }
            2594u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.repeated_struct, buf)?;
            }
            2522u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.repeated_any, buf)?;
            }
            2530u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.repeated_value, buf)?;
            }
            2538u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.repeated_list_value, buf)?;
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
            888u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint32(tmp);
            }
            890u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint32(tmp);
            }
            898u32 => {
                let mut tmp = Default::default();
                buf = Format::<Nest>::decode(&mut tmp, buf)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofNestedMessage(
                    tmp,
                );
            }
            906u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofString(tmp);
            }
            914u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBytes(tmp);
            }
            920u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBool(tmp);
            }
            922u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofBool(tmp);
            }
            928u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint64(tmp);
            }
            930u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofUint64(tmp);
            }
            941u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofFloat(tmp);
            }
            938u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofFloat(tmp);
            }
            945u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofDouble(tmp);
            }
            946u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofDouble(tmp);
            }
            952u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofEnum(tmp);
            }
            954u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.oneof_field = TestAllTypesProto3OneOfOneofField::OneofEnum(tmp);
            }
            960u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofNullValue(
                    tmp,
                );
            }
            962u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self
                    .oneof_field = TestAllTypesProto3OneOfOneofField::OneofNullValue(
                    tmp,
                );
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TestAllTypesProto3 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto3.TestAllTypesProto3"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<i32>::eq(&self.optional_int32, &Default::default()) {
            Format::<VInt>::encode(&self.optional_int32, 1u32, buf)?;
        }
        if !PartialEq::<i64>::eq(&self.optional_int64, &Default::default()) {
            Format::<VInt>::encode(&self.optional_int64, 2u32, buf)?;
        }
        if !PartialEq::<u32>::eq(&self.optional_uint32, &Default::default()) {
            Format::<VInt>::encode(&self.optional_uint32, 3u32, buf)?;
        }
        if !PartialEq::<u64>::eq(&self.optional_uint64, &Default::default()) {
            Format::<VInt>::encode(&self.optional_uint64, 4u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.optional_sint32, &Default::default()) {
            Format::<SInt>::encode(&self.optional_sint32, 5u32, buf)?;
        }
        if !PartialEq::<i64>::eq(&self.optional_sint64, &Default::default()) {
            Format::<SInt>::encode(&self.optional_sint64, 6u32, buf)?;
        }
        if !PartialEq::<u32>::eq(&self.optional_fixed32, &Default::default()) {
            Format::<Fix>::encode(&self.optional_fixed32, 7u32, buf)?;
        }
        if !PartialEq::<u64>::eq(&self.optional_fixed64, &Default::default()) {
            Format::<Fix>::encode(&self.optional_fixed64, 8u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.optional_sfixed32, &Default::default()) {
            Format::<Fix>::encode(&self.optional_sfixed32, 9u32, buf)?;
        }
        if !PartialEq::<i64>::eq(&self.optional_sfixed64, &Default::default()) {
            Format::<Fix>::encode(&self.optional_sfixed64, 10u32, buf)?;
        }
        if !PartialEq::<f32>::eq(&self.optional_float, &Default::default()) {
            Format::<Fix>::encode(&self.optional_float, 11u32, buf)?;
        }
        if !PartialEq::<f64>::eq(&self.optional_double, &Default::default()) {
            Format::<Fix>::encode(&self.optional_double, 12u32, buf)?;
        }
        if !PartialEq::<bool>::eq(&self.optional_bool, &Default::default()) {
            Format::<Fix>::encode(&self.optional_bool, 13u32, buf)?;
        }
        if !PartialEq::<String>::eq(&self.optional_string, &Default::default()) {
            Format::<Bytes>::encode(&self.optional_string, 14u32, buf)?;
        }
        if !PartialEq::<Vec<u8>>::eq(&self.optional_bytes, &Default::default()) {
            Format::<Bytes>::encode(&self.optional_bytes, 15u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<TestAllTypesProto3NestedMessage>>,
        >::eq(&self.optional_nested_message, &Default::default()) {
            Format::<Nest>::encode(&self.optional_nested_message, 18u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<ForeignMessage>>,
        >::eq(&self.optional_foreign_message, &Default::default()) {
            Format::<Nest>::encode(&self.optional_foreign_message, 19u32, buf)?;
        }
        if !PartialEq::<
            TestAllTypesProto3NestedEnum,
        >::eq(&self.optional_nested_enum, &Default::default()) {
            Format::<Enum>::encode(&self.optional_nested_enum, 21u32, buf)?;
        }
        if !PartialEq::<
            ForeignEnum,
        >::eq(&self.optional_foreign_enum, &Default::default()) {
            Format::<Enum>::encode(&self.optional_foreign_enum, 22u32, buf)?;
        }
        if !PartialEq::<
            TestAllTypesProto3AliasedEnum,
        >::eq(&self.optional_aliased_enum, &Default::default()) {
            Format::<Enum>::encode(&self.optional_aliased_enum, 23u32, buf)?;
        }
        if !PartialEq::<String>::eq(&self.optional_string_piece, &Default::default()) {
            Format::<Bytes>::encode(&self.optional_string_piece, 24u32, buf)?;
        }
        if !PartialEq::<String>::eq(&self.optional_cord, &Default::default()) {
            Format::<Bytes>::encode(&self.optional_cord, 25u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<TestAllTypesProto3>>,
        >::eq(&self.recursive_message, &Default::default()) {
            Format::<Nest>::encode(&self.recursive_message, 27u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.repeated_int32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.repeated_int32, 31u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.repeated_int64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.repeated_int64, 32u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.repeated_uint32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.repeated_uint32, 33u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.repeated_uint64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.repeated_uint64, 34u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.repeated_sint32, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.repeated_sint32, 35u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.repeated_sint64, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.repeated_sint64, 36u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.repeated_fixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_fixed32, 37u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.repeated_fixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_fixed64, 38u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.repeated_sfixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_sfixed32, 39u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.repeated_sfixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_sfixed64, 40u32, buf)?;
        }
        if !PartialEq::<Vec<f32>>::eq(&self.repeated_float, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_float, 41u32, buf)?;
        }
        if !PartialEq::<Vec<f64>>::eq(&self.repeated_double, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_double, 42u32, buf)?;
        }
        if !PartialEq::<Vec<bool>>::eq(&self.repeated_bool, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.repeated_bool, 43u32, buf)?;
        }
        if !PartialEq::<Vec<String>>::eq(&self.repeated_string, &Default::default()) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_string, 44u32, buf)?;
        }
        if !PartialEq::<Vec<Vec<u8>>>::eq(&self.repeated_bytes, &Default::default()) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_bytes, 45u32, buf)?;
        }
        if !PartialEq::<
            Vec<TestAllTypesProto3NestedMessage>,
        >::eq(&self.repeated_nested_message, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_nested_message, 48u32, buf)?;
        }
        if !PartialEq::<
            Vec<ForeignMessage>,
        >::eq(&self.repeated_foreign_message, &Default::default()) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_foreign_message, 49u32, buf)?;
        }
        if !PartialEq::<
            Vec<TestAllTypesProto3NestedEnum>,
        >::eq(&self.repeated_nested_enum, &Default::default()) {
            Format::<Repeat::<Enum>>::encode(&self.repeated_nested_enum, 51u32, buf)?;
        }
        if !PartialEq::<
            Vec<ForeignEnum>,
        >::eq(&self.repeated_foreign_enum, &Default::default()) {
            Format::<Repeat::<Enum>>::encode(&self.repeated_foreign_enum, 52u32, buf)?;
        }
        if !PartialEq::<
            Vec<String>,
        >::eq(&self.repeated_string_piece, &Default::default()) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_string_piece, 54u32, buf)?;
        }
        if !PartialEq::<Vec<String>>::eq(&self.repeated_cord, &Default::default()) {
            Format::<Repeat::<Bytes>>::encode(&self.repeated_cord, 55u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.packed_int32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.packed_int32, 75u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.packed_int64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.packed_int64, 76u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.packed_uint32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.packed_uint32, 77u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.packed_uint64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.packed_uint64, 78u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.packed_sint32, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.packed_sint32, 79u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.packed_sint64, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.packed_sint64, 80u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.packed_fixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_fixed32, 81u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.packed_fixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_fixed64, 82u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.packed_sfixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_sfixed32, 83u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.packed_sfixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_sfixed64, 84u32, buf)?;
        }
        if !PartialEq::<Vec<f32>>::eq(&self.packed_float, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_float, 85u32, buf)?;
        }
        if !PartialEq::<Vec<f64>>::eq(&self.packed_double, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_double, 86u32, buf)?;
        }
        if !PartialEq::<Vec<bool>>::eq(&self.packed_bool, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.packed_bool, 87u32, buf)?;
        }
        if !PartialEq::<
            Vec<TestAllTypesProto3NestedEnum>,
        >::eq(&self.packed_nested_enum, &Default::default()) {
            Format::<Repeat::<Enum>>::encode(&self.packed_nested_enum, 88u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.unpacked_int32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.unpacked_int32, 89u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.unpacked_int64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.unpacked_int64, 90u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.unpacked_uint32, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.unpacked_uint32, 91u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.unpacked_uint64, &Default::default()) {
            Format::<Pack::<VInt>>::encode(&self.unpacked_uint64, 92u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.unpacked_sint32, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.unpacked_sint32, 93u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.unpacked_sint64, &Default::default()) {
            Format::<Pack::<SInt>>::encode(&self.unpacked_sint64, 94u32, buf)?;
        }
        if !PartialEq::<Vec<u32>>::eq(&self.unpacked_fixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_fixed32, 95u32, buf)?;
        }
        if !PartialEq::<Vec<u64>>::eq(&self.unpacked_fixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_fixed64, 96u32, buf)?;
        }
        if !PartialEq::<Vec<i32>>::eq(&self.unpacked_sfixed32, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_sfixed32, 97u32, buf)?;
        }
        if !PartialEq::<Vec<i64>>::eq(&self.unpacked_sfixed64, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_sfixed64, 98u32, buf)?;
        }
        if !PartialEq::<Vec<f32>>::eq(&self.unpacked_float, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_float, 99u32, buf)?;
        }
        if !PartialEq::<Vec<f64>>::eq(&self.unpacked_double, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_double, 100u32, buf)?;
        }
        if !PartialEq::<Vec<bool>>::eq(&self.unpacked_bool, &Default::default()) {
            Format::<Pack::<Fix>>::encode(&self.unpacked_bool, 101u32, buf)?;
        }
        if !PartialEq::<
            Vec<TestAllTypesProto3NestedEnum>,
        >::eq(&self.unpacked_nested_enum, &Default::default()) {
            Format::<Repeat::<Enum>>::encode(&self.unpacked_nested_enum, 102u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i32, i32>,
        >::eq(&self.map_int32_int32, &Default::default()) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_int32_int32, 56u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i64, i64>,
        >::eq(&self.map_int64_int64, &Default::default()) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_int64_int64, 57u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<u32, u32>,
        >::eq(&self.map_uint32_uint32, &Default::default()) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_uint32_uint32, 58u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<u64, u64>,
        >::eq(&self.map_uint64_uint64, &Default::default()) {
            Format::<Map::<VInt, VInt>>::encode(&self.map_uint64_uint64, 59u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i32, i32>,
        >::eq(&self.map_sint32_sint32, &Default::default()) {
            Format::<Map::<SInt, SInt>>::encode(&self.map_sint32_sint32, 60u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i64, i64>,
        >::eq(&self.map_sint64_sint64, &Default::default()) {
            Format::<Map::<SInt, SInt>>::encode(&self.map_sint64_sint64, 61u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<u32, u32>,
        >::eq(&self.map_fixed32_fixed32, &Default::default()) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_fixed32_fixed32, 62u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<u64, u64>,
        >::eq(&self.map_fixed64_fixed64, &Default::default()) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_fixed64_fixed64, 63u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i32, i32>,
        >::eq(&self.map_sfixed32_sfixed32, &Default::default()) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_sfixed32_sfixed32, 64u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i64, i64>,
        >::eq(&self.map_sfixed64_sfixed64, &Default::default()) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_sfixed64_sfixed64, 65u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i32, f32>,
        >::eq(&self.map_int32_float, &Default::default()) {
            Format::<Map::<VInt, Fix>>::encode(&self.map_int32_float, 66u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<i32, f64>,
        >::eq(&self.map_int32_double, &Default::default()) {
            Format::<Map::<VInt, Fix>>::encode(&self.map_int32_double, 67u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<bool, bool>,
        >::eq(&self.map_bool_bool, &Default::default()) {
            Format::<Map::<Fix, Fix>>::encode(&self.map_bool_bool, 68u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, String>,
        >::eq(&self.map_string_string, &Default::default()) {
            Format::<Map::<Bytes, Bytes>>::encode(&self.map_string_string, 69u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, Vec<u8>>,
        >::eq(&self.map_string_bytes, &Default::default()) {
            Format::<Map::<Bytes, Bytes>>::encode(&self.map_string_bytes, 70u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, TestAllTypesProto3NestedMessage>,
        >::eq(&self.map_string_nested_message, &Default::default()) {
            Format::<
                Map::<Bytes, Nest>,
            >::encode(&self.map_string_nested_message, 71u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, ForeignMessage>,
        >::eq(&self.map_string_foreign_message, &Default::default()) {
            Format::<
                Map::<Bytes, Nest>,
            >::encode(&self.map_string_foreign_message, 72u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, TestAllTypesProto3NestedEnum>,
        >::eq(&self.map_string_nested_enum, &Default::default()) {
            Format::<
                Map::<Bytes, Enum>,
            >::encode(&self.map_string_nested_enum, 73u32, buf)?;
        }
        if !PartialEq::<
            ::std::collections::HashMap<String, ForeignEnum>,
        >::eq(&self.map_string_foreign_enum, &Default::default()) {
            Format::<
                Map::<Bytes, Enum>,
            >::encode(&self.map_string_foreign_enum, 74u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<BoolValue>>,
        >::eq(&self.optional_bool_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_bool_wrapper, 201u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Int32Value>>,
        >::eq(&self.optional_int32_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_int32_wrapper, 202u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Int64Value>>,
        >::eq(&self.optional_int64_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_int64_wrapper, 203u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<UInt32Value>>,
        >::eq(&self.optional_uint32_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_uint32_wrapper, 204u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<UInt64Value>>,
        >::eq(&self.optional_uint64_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_uint64_wrapper, 205u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<FloatValue>>,
        >::eq(&self.optional_float_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_float_wrapper, 206u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<DoubleValue>>,
        >::eq(&self.optional_double_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_double_wrapper, 207u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<StringValue>>,
        >::eq(&self.optional_string_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_string_wrapper, 208u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<BytesValue>>,
        >::eq(&self.optional_bytes_wrapper, &Default::default()) {
            Format::<Nest>::encode(&self.optional_bytes_wrapper, 209u32, buf)?;
        }
        if !PartialEq::<
            Vec<BoolValue>,
        >::eq(&self.repeated_bool_wrapper, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_bool_wrapper, 211u32, buf)?;
        }
        if !PartialEq::<
            Vec<Int32Value>,
        >::eq(&self.repeated_int32_wrapper, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_int32_wrapper, 212u32, buf)?;
        }
        if !PartialEq::<
            Vec<Int64Value>,
        >::eq(&self.repeated_int64_wrapper, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_int64_wrapper, 213u32, buf)?;
        }
        if !PartialEq::<
            Vec<UInt32Value>,
        >::eq(&self.repeated_uint32_wrapper, &Default::default()) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_uint32_wrapper, 214u32, buf)?;
        }
        if !PartialEq::<
            Vec<UInt64Value>,
        >::eq(&self.repeated_uint64_wrapper, &Default::default()) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_uint64_wrapper, 215u32, buf)?;
        }
        if !PartialEq::<
            Vec<FloatValue>,
        >::eq(&self.repeated_float_wrapper, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_float_wrapper, 216u32, buf)?;
        }
        if !PartialEq::<
            Vec<DoubleValue>,
        >::eq(&self.repeated_double_wrapper, &Default::default()) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_double_wrapper, 217u32, buf)?;
        }
        if !PartialEq::<
            Vec<StringValue>,
        >::eq(&self.repeated_string_wrapper, &Default::default()) {
            Format::<
                Repeat::<Nest>,
            >::encode(&self.repeated_string_wrapper, 218u32, buf)?;
        }
        if !PartialEq::<
            Vec<BytesValue>,
        >::eq(&self.repeated_bytes_wrapper, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_bytes_wrapper, 219u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Duration>>,
        >::eq(&self.optional_duration, &Default::default()) {
            Format::<Nest>::encode(&self.optional_duration, 301u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Timestamp>>,
        >::eq(&self.optional_timestamp, &Default::default()) {
            Format::<Nest>::encode(&self.optional_timestamp, 302u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<FieldMask>>,
        >::eq(&self.optional_field_mask, &Default::default()) {
            Format::<Nest>::encode(&self.optional_field_mask, 303u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Struct>>,
        >::eq(&self.optional_struct, &Default::default()) {
            Format::<Nest>::encode(&self.optional_struct, 304u32, buf)?;
        }
        if !PartialEq::<Option<Box<Any>>>::eq(&self.optional_any, &Default::default()) {
            Format::<Nest>::encode(&self.optional_any, 305u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<Value>>,
        >::eq(&self.optional_value, &Default::default()) {
            Format::<Nest>::encode(&self.optional_value, 306u32, buf)?;
        }
        if !PartialEq::<NullValue>::eq(&self.optional_null_value, &Default::default()) {
            Format::<Enum>::encode(&self.optional_null_value, 307u32, buf)?;
        }
        if !PartialEq::<
            Vec<Duration>,
        >::eq(&self.repeated_duration, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_duration, 311u32, buf)?;
        }
        if !PartialEq::<
            Vec<Timestamp>,
        >::eq(&self.repeated_timestamp, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_timestamp, 312u32, buf)?;
        }
        if !PartialEq::<
            Vec<FieldMask>,
        >::eq(&self.repeated_fieldmask, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_fieldmask, 313u32, buf)?;
        }
        if !PartialEq::<Vec<Struct>>::eq(&self.repeated_struct, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_struct, 324u32, buf)?;
        }
        if !PartialEq::<Vec<Any>>::eq(&self.repeated_any, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_any, 315u32, buf)?;
        }
        if !PartialEq::<Vec<Value>>::eq(&self.repeated_value, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_value, 316u32, buf)?;
        }
        if !PartialEq::<
            Vec<ListValue>,
        >::eq(&self.repeated_list_value, &Default::default()) {
            Format::<Repeat::<Nest>>::encode(&self.repeated_list_value, 317u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.fieldname1, &Default::default()) {
            Format::<VInt>::encode(&self.fieldname1, 401u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field_name2, &Default::default()) {
            Format::<VInt>::encode(&self.field_name2, 402u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self._field_name3, &Default::default()) {
            Format::<VInt>::encode(&self._field_name3, 403u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field__name4_, &Default::default()) {
            Format::<VInt>::encode(&self.field__name4_, 404u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field0name5, &Default::default()) {
            Format::<VInt>::encode(&self.field0name5, 405u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field_0_name6, &Default::default()) {
            Format::<VInt>::encode(&self.field_0_name6, 406u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.fieldName7, &Default::default()) {
            Format::<VInt>::encode(&self.fieldName7, 407u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.FieldName8, &Default::default()) {
            Format::<VInt>::encode(&self.FieldName8, 408u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field_Name9, &Default::default()) {
            Format::<VInt>::encode(&self.field_Name9, 409u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.Field_Name10, &Default::default()) {
            Format::<VInt>::encode(&self.Field_Name10, 410u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.FIELD_NAME11, &Default::default()) {
            Format::<VInt>::encode(&self.FIELD_NAME11, 411u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.FIELD_name12, &Default::default()) {
            Format::<VInt>::encode(&self.FIELD_name12, 412u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.__field_name13, &Default::default()) {
            Format::<VInt>::encode(&self.__field_name13, 413u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.__Field_name14, &Default::default()) {
            Format::<VInt>::encode(&self.__Field_name14, 414u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field__name15, &Default::default()) {
            Format::<VInt>::encode(&self.field__name15, 415u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field__Name16, &Default::default()) {
            Format::<VInt>::encode(&self.field__Name16, 416u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.field_name17__, &Default::default()) {
            Format::<VInt>::encode(&self.field_name17__, 417u32, buf)?;
        }
        if !PartialEq::<i32>::eq(&self.Field_name18__, &Default::default()) {
            Format::<VInt>::encode(&self.Field_name18__, 418u32, buf)?;
        }
        match &self.oneof_field {
            TestAllTypesProto3OneOfOneofField::OneofUint32(value) => {
                Format::<VInt>::encode(value, 111u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofNestedMessage(value) => {
                Format::<Nest>::encode(value, 112u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofString(value) => {
                Format::<Bytes>::encode(value, 113u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofBytes(value) => {
                Format::<Bytes>::encode(value, 114u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofBool(value) => {
                Format::<Fix>::encode(value, 115u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofUint64(value) => {
                Format::<VInt>::encode(value, 116u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofFloat(value) => {
                Format::<Fix>::encode(value, 117u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofDouble(value) => {
                Format::<Fix>::encode(value, 118u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofEnum(value) => {
                Format::<Enum>::encode(value, 119u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::OneofNullValue(value) => {
                Format::<Enum>::encode(value, 120u32, buf)?;
            }
            TestAllTypesProto3OneOfOneofField::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum TestAllTypesProto3OneOfOneofField {
    OneofUint32(u32),
    OneofNestedMessage(TestAllTypesProto3NestedMessage),
    OneofString(String),
    OneofBytes(Vec<u8>),
    OneofBool(bool),
    OneofUint64(u64),
    OneofFloat(f32),
    OneofDouble(f64),
    OneofEnum(TestAllTypesProto3NestedEnum),
    OneofNullValue(NullValue),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for TestAllTypesProto3OneOfOneofField {
    fn default() -> Self {
        TestAllTypesProto3OneOfOneofField::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestAllTypesProto3NestedMessage {
    pub a: i32,
    pub corecursive: Option<Box<TestAllTypesProto3>>,
    pub _unknown: (),
}
impl TestAllTypesProto3NestedMessage {
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
    pub fn r#with_corecursive(mut self, it: TestAllTypesProto3) -> Self {
        self.r#set_corecursive(it);
        self
    }
    #[inline(always)]
    pub fn r#set_corecursive(&mut self, it: TestAllTypesProto3) -> &mut Self {
        self.corecursive = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for TestAllTypesProto3NestedMessage {
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
impl textformat::Encodable for TestAllTypesProto3NestedMessage {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.a != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("a: ");
            textformat::Field::format(&self.a, ctx, pad, out)?;
            out.push('\n');
        }
        if self.corecursive != <Option<Box<TestAllTypesProto3>> as Default>::default() {
            out.indent(pad);
            out.push_str("corecursive ");
            textformat::Field::format(&self.corecursive, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TestAllTypesProto3NestedMessage {
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
impl binformat::Encodable for TestAllTypesProto3NestedMessage {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto3.TestAllTypesProto3.NestedMessage"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<i32>::eq(&self.a, &Default::default()) {
            Format::<VInt>::encode(&self.a, 1u32, buf)?;
        }
        if !PartialEq::<
            Option<Box<TestAllTypesProto3>>,
        >::eq(&self.corecursive, &Default::default()) {
            Format::<Nest>::encode(&self.corecursive, 2u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ForeignMessage {
    pub c: i32,
    pub _unknown: (),
}
impl ForeignMessage {
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
impl textformat::Decodable for ForeignMessage {
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
impl textformat::Encodable for ForeignMessage {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.c != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("c: ");
            textformat::Field::format(&self.c, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ForeignMessage {
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
impl binformat::Encodable for ForeignMessage {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto3.ForeignMessage"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<i32>::eq(&self.c, &Default::default()) {
            Format::<VInt>::encode(&self.c, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NullHypothesisProto3 {
    pub _unknown: (),
}
impl NullHypothesisProto3 {}
impl textformat::Decodable for NullHypothesisProto3 {
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
impl textformat::Encodable for NullHypothesisProto3 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        Ok(())
    }
}
impl binformat::Decodable for NullHypothesisProto3 {
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
impl binformat::Encodable for NullHypothesisProto3 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto3.NullHypothesisProto3"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumOnlyProto3 {
    pub _unknown: (),
}
impl EnumOnlyProto3 {}
impl textformat::Decodable for EnumOnlyProto3 {
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
impl textformat::Encodable for EnumOnlyProto3 {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        Ok(())
    }
}
impl binformat::Decodable for EnumOnlyProto3 {
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
impl binformat::Encodable for EnumOnlyProto3 {
    fn qualified_name(&self) -> &'static str {
        "protobuf_test_messages.proto3.EnumOnlyProto3"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TestAllTypesProto3NestedEnum {
    FOO,
    BAR,
    BAZ,
    NEG,
    Unknown(u32),
}
impl Default for TestAllTypesProto3NestedEnum {
    fn default() -> TestAllTypesProto3NestedEnum {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for TestAllTypesProto3NestedEnum {}
impl From<u32> for TestAllTypesProto3NestedEnum {
    fn from(v: u32) -> TestAllTypesProto3NestedEnum {
        match v {
            0u32 => TestAllTypesProto3NestedEnum::FOO,
            1u32 => TestAllTypesProto3NestedEnum::BAR,
            2u32 => TestAllTypesProto3NestedEnum::BAZ,
            4294967295u32 => TestAllTypesProto3NestedEnum::NEG,
            other => TestAllTypesProto3NestedEnum::Unknown(other),
        }
    }
}
impl From<TestAllTypesProto3NestedEnum> for u32 {
    fn from(v: TestAllTypesProto3NestedEnum) -> u32 {
        match v {
            TestAllTypesProto3NestedEnum::FOO => 0u32,
            TestAllTypesProto3NestedEnum::BAR => 1u32,
            TestAllTypesProto3NestedEnum::BAZ => 2u32,
            TestAllTypesProto3NestedEnum::NEG => 4294967295u32,
            TestAllTypesProto3NestedEnum::Unknown(other) => other,
        }
    }
}
impl textformat::Field for TestAllTypesProto3NestedEnum {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            TestAllTypesProto3NestedEnum::FOO => "FOO",
            TestAllTypesProto3NestedEnum::BAR => "BAR",
            TestAllTypesProto3NestedEnum::BAZ => "BAZ",
            TestAllTypesProto3NestedEnum::NEG => "NEG",
            TestAllTypesProto3NestedEnum::Unknown(n) => {
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
                *self = TestAllTypesProto3NestedEnum::FOO;
            }
            textformat::ast::Literal::Identifier("BAR") => {
                *self = TestAllTypesProto3NestedEnum::BAR;
            }
            textformat::ast::Literal::Identifier("BAZ") => {
                *self = TestAllTypesProto3NestedEnum::BAZ;
            }
            textformat::ast::Literal::Identifier("NEG") => {
                *self = TestAllTypesProto3NestedEnum::NEG;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TestAllTypesProto3AliasedEnum {
    ALIAS_FOO,
    ALIAS_BAR,
    ALIAS_BAZ,
    MOO,
    moo,
    bAz,
    Unknown(u32),
}
impl Default for TestAllTypesProto3AliasedEnum {
    fn default() -> TestAllTypesProto3AliasedEnum {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for TestAllTypesProto3AliasedEnum {}
impl From<u32> for TestAllTypesProto3AliasedEnum {
    fn from(v: u32) -> TestAllTypesProto3AliasedEnum {
        match v {
            0u32 => TestAllTypesProto3AliasedEnum::ALIAS_FOO,
            1u32 => TestAllTypesProto3AliasedEnum::ALIAS_BAR,
            2u32 => TestAllTypesProto3AliasedEnum::ALIAS_BAZ,
            2u32 => TestAllTypesProto3AliasedEnum::MOO,
            2u32 => TestAllTypesProto3AliasedEnum::moo,
            2u32 => TestAllTypesProto3AliasedEnum::bAz,
            other => TestAllTypesProto3AliasedEnum::Unknown(other),
        }
    }
}
impl From<TestAllTypesProto3AliasedEnum> for u32 {
    fn from(v: TestAllTypesProto3AliasedEnum) -> u32 {
        match v {
            TestAllTypesProto3AliasedEnum::ALIAS_FOO => 0u32,
            TestAllTypesProto3AliasedEnum::ALIAS_BAR => 1u32,
            TestAllTypesProto3AliasedEnum::ALIAS_BAZ => 2u32,
            TestAllTypesProto3AliasedEnum::MOO => 2u32,
            TestAllTypesProto3AliasedEnum::moo => 2u32,
            TestAllTypesProto3AliasedEnum::bAz => 2u32,
            TestAllTypesProto3AliasedEnum::Unknown(other) => other,
        }
    }
}
impl textformat::Field for TestAllTypesProto3AliasedEnum {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            TestAllTypesProto3AliasedEnum::ALIAS_FOO => "ALIAS_FOO",
            TestAllTypesProto3AliasedEnum::ALIAS_BAR => "ALIAS_BAR",
            TestAllTypesProto3AliasedEnum::ALIAS_BAZ => "ALIAS_BAZ",
            TestAllTypesProto3AliasedEnum::MOO => "MOO",
            TestAllTypesProto3AliasedEnum::moo => "moo",
            TestAllTypesProto3AliasedEnum::bAz => "bAz",
            TestAllTypesProto3AliasedEnum::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("ALIAS_FOO") => {
                *self = TestAllTypesProto3AliasedEnum::ALIAS_FOO;
            }
            textformat::ast::Literal::Identifier("ALIAS_BAR") => {
                *self = TestAllTypesProto3AliasedEnum::ALIAS_BAR;
            }
            textformat::ast::Literal::Identifier("ALIAS_BAZ") => {
                *self = TestAllTypesProto3AliasedEnum::ALIAS_BAZ;
            }
            textformat::ast::Literal::Identifier("MOO") => {
                *self = TestAllTypesProto3AliasedEnum::MOO;
            }
            textformat::ast::Literal::Identifier("moo") => {
                *self = TestAllTypesProto3AliasedEnum::moo;
            }
            textformat::ast::Literal::Identifier("bAz") => {
                *self = TestAllTypesProto3AliasedEnum::bAz;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignEnum {
    FOREIGN_FOO,
    FOREIGN_BAR,
    FOREIGN_BAZ,
    Unknown(u32),
}
impl Default for ForeignEnum {
    fn default() -> ForeignEnum {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ForeignEnum {}
impl From<u32> for ForeignEnum {
    fn from(v: u32) -> ForeignEnum {
        match v {
            0u32 => ForeignEnum::FOREIGN_FOO,
            1u32 => ForeignEnum::FOREIGN_BAR,
            2u32 => ForeignEnum::FOREIGN_BAZ,
            other => ForeignEnum::Unknown(other),
        }
    }
}
impl From<ForeignEnum> for u32 {
    fn from(v: ForeignEnum) -> u32 {
        match v {
            ForeignEnum::FOREIGN_FOO => 0u32,
            ForeignEnum::FOREIGN_BAR => 1u32,
            ForeignEnum::FOREIGN_BAZ => 2u32,
            ForeignEnum::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ForeignEnum {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ForeignEnum::FOREIGN_FOO => "FOREIGN_FOO",
            ForeignEnum::FOREIGN_BAR => "FOREIGN_BAR",
            ForeignEnum::FOREIGN_BAZ => "FOREIGN_BAZ",
            ForeignEnum::Unknown(n) => {
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
                *self = ForeignEnum::FOREIGN_FOO;
            }
            textformat::ast::Literal::Identifier("FOREIGN_BAR") => {
                *self = ForeignEnum::FOREIGN_BAR;
            }
            textformat::ast::Literal::Identifier("FOREIGN_BAZ") => {
                *self = ForeignEnum::FOREIGN_BAZ;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum EnumOnlyProto3Bool {
    kFalse,
    kTrue,
    Unknown(u32),
}
impl Default for EnumOnlyProto3Bool {
    fn default() -> EnumOnlyProto3Bool {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for EnumOnlyProto3Bool {}
impl From<u32> for EnumOnlyProto3Bool {
    fn from(v: u32) -> EnumOnlyProto3Bool {
        match v {
            0u32 => EnumOnlyProto3Bool::kFalse,
            1u32 => EnumOnlyProto3Bool::kTrue,
            other => EnumOnlyProto3Bool::Unknown(other),
        }
    }
}
impl From<EnumOnlyProto3Bool> for u32 {
    fn from(v: EnumOnlyProto3Bool) -> u32 {
        match v {
            EnumOnlyProto3Bool::kFalse => 0u32,
            EnumOnlyProto3Bool::kTrue => 1u32,
            EnumOnlyProto3Bool::Unknown(other) => other,
        }
    }
}
impl textformat::Field for EnumOnlyProto3Bool {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            EnumOnlyProto3Bool::kFalse => "kFalse",
            EnumOnlyProto3Bool::kTrue => "kTrue",
            EnumOnlyProto3Bool::Unknown(n) => {
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
                *self = EnumOnlyProto3Bool::kFalse;
            }
            textformat::ast::Literal::Identifier("kTrue") => {
                *self = EnumOnlyProto3Bool::kTrue;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
