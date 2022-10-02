pub mod validate;
pub mod google;
pub fn register_types(registry: &mut crate::reflect::Registry) {
    validate::register_types(registry);
    google::register_types(registry);
}
