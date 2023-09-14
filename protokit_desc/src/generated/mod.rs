use crate as protokit;
pub mod google;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    google::register_types(registry);
}
