pub mod protobuf;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    protobuf::register_types(registry);
}
