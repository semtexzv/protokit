pub mod validate;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    validate::register_types(registry);
}
