pub mod validate;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    validate::register_types(registry);
}
