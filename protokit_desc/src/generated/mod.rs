pub mod validate;
pub mod google;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    validate::register_types(registry);
    google::register_types(registry);
}
