pub mod conformance;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    conformance::register_types(registry);
}
