pub mod google;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    google::register_types(registry);
}
