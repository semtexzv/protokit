pub mod validate;
pub mod com;
pub mod google;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    validate::register_types(registry);
    com::register_types(registry);
    google::register_types(registry);
}
