pub mod plugin;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    plugin::register_types(registry);
}
