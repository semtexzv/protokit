use ::protokit::*;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FieldMask {
    #[field(1u32, "paths", string, repeated)]
    pub paths: Vec<String>,
    #[unknown]
    pub unknown: protokit::binformat::UnknownFields,
}
