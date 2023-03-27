pub mod protobuf;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    protobuf::register_types(registry);
}
