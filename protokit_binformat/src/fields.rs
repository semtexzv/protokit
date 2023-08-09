use derive::BinProto;

use crate as binformat;

#[derive(Default, Debug, Clone, BinProto)]
pub struct FieldPath {
    #[field(1, varint, packed)]
    pub fields: Vec<u32>,
}

#[derive(Default, Debug, Clone, BinProto)]
pub struct FieldMask {
    #[field(1, nested, repeated)]
    pub paths: Vec<FieldPath>,
}

/// A struct that can be used for filtering fields by number.
pub trait FieldFilter {
    /// Whether the `field` matches this filter
    fn matches(&self, field: u32) -> bool;
    /// We're serializing a nested message, and want the nested filter.
    fn nest(&self, field: u32) -> Option<&Self>;
}