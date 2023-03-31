pub mod plugin;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    plugin::register_types(registry);
}
