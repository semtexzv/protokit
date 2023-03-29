pub mod google;
pub mod validate;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    google::register_types(registry);
    validate::register_types(registry);
}
