pub mod proto2;
pub mod proto3;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    proto2::register_types(registry);
    proto3::register_types(registry);
}
