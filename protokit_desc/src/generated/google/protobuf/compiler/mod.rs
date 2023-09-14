use crate as protokit;
pub mod plugin;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    plugin::register_types(registry);
}
