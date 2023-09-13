pub mod any;
pub mod compiler;
pub mod descriptor;
pub mod empty;
pub mod field_mask;
pub mod timestamp;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    any::register_types(registry);
    compiler::register_types(registry);
    descriptor::register_types(registry);
    empty::register_types(registry);
    field_mask::register_types(registry);
    timestamp::register_types(registry);
}
