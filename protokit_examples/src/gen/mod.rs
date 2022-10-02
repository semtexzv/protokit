pub mod google;
pub mod com;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    google::register_types(registry);
    com::register_types(registry);
}
