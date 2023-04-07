pub mod com;
pub mod google;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    com::register_types(registry);
    google::register_types(registry);
}
