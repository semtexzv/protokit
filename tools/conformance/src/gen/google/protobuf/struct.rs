#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use protokit::*;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&Struct::default());
    registry.register(&Value::default());
    registry.register(&ListValue::default());
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NullValue(pub u32);
#[protoenum]
impl NullValue {
    #[var(0u32, "NULL_VALUE")]
    pub const NULL_VALUE: NullValue = NullValue(0u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Struct", package = "google.protobuf")]
pub struct Struct {
    #[field(1u32, "fields", map(string, nested), singular)]
    pub fields: ::protokit::IndexMap<String, Value>,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum ValueOneOfKind {
    #[field(1u32, "null_value", protoenum, raw)]
    NullValue(NullValue),
    #[field(2u32, "number_value", fixed64, raw)]
    NumberValue(f64),
    #[field(3u32, "string_value", string, raw)]
    StringValue(String),
    #[field(4u32, "bool_value", bool, raw)]
    BoolValue(bool),
    #[field(5u32, "struct_value", nested, raw)]
    StructValue(Struct),
    #[field(6u32, "list_value", nested, raw)]
    ListValue(ListValue),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ValueOneOfKind {
    fn default() -> Self {
        Self::NullValue(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Value", package = "google.protobuf")]
pub struct Value {
    #[oneof(
        [1u32,
        2u32,
        3u32,
        4u32,
        5u32,
        6u32,
        ],
        ["null_value",
        "number_value",
        "string_value",
        "bool_value",
        "struct_value",
        "list_value",
        ]
    )]
    pub kind: Option<ValueOneOfKind>,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "ListValue", package = "google.protobuf")]
pub struct ListValue {
    #[field(1u32, "values", nested, repeated)]
    pub values: Vec<Value>,
    #[unknown]
    pub unknown: ::protokit::binformat::UnknownFieldsOwned,
}
