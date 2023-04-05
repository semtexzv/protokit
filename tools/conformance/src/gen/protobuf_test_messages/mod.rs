pub mod proto3;
pub mod proto2;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    proto3::register_types(registry);
    proto2::register_types(registry);
}
