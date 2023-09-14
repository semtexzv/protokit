#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    registry.register(&DoubleValue::default());
    registry.register(&FloatValue::default());
    registry.register(&Int64Value::default());
    registry.register(&UInt64Value::default());
    registry.register(&Int32Value::default());
    registry.register(&UInt32Value::default());
    registry.register(&BoolValue::default());
    registry.register(&StringValue::default());
    registry.register(&BytesValue::default());
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "DoubleValue", package = "google.protobuf")]
pub struct DoubleValue {
    #[field(1u32, "value", fixed64, singular)]
    pub value: f64,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "FloatValue", package = "google.protobuf")]
pub struct FloatValue {
    #[field(1u32, "value", fixed32, singular)]
    pub value: f32,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Int64Value", package = "google.protobuf")]
pub struct Int64Value {
    #[field(1u32, "value", varint, singular)]
    pub value: i64,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "UInt64Value", package = "google.protobuf")]
pub struct UInt64Value {
    #[field(1u32, "value", varint, singular)]
    pub value: u64,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Int32Value", package = "google.protobuf")]
pub struct Int32Value {
    #[field(1u32, "value", varint, singular)]
    pub value: i32,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "UInt32Value", package = "google.protobuf")]
pub struct UInt32Value {
    #[field(1u32, "value", varint, singular)]
    pub value: u32,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "BoolValue", package = "google.protobuf")]
pub struct BoolValue {
    #[field(1u32, "value", bool, singular)]
    pub value: bool,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "StringValue", package = "google.protobuf")]
pub struct StringValue {
    #[field(1u32, "value", string, singular)]
    pub value: String,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "BytesValue", package = "google.protobuf")]
pub struct BytesValue {
    #[field(1u32, "value", bytes, singular)]
    pub value: Vec<u8>,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
