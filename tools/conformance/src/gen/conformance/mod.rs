pub mod conformance;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    conformance::register_types(registry);
}
