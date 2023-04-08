pub mod empty;
pub mod descriptor;
pub mod compiler;
pub mod timestamp;
pub mod any;
pub mod duration;
pub mod field_mask;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    empty::register_types(registry);
    descriptor::register_types(registry);
    compiler::register_types(registry);
    timestamp::register_types(registry);
    any::register_types(registry);
    duration::register_types(registry);
    field_mask::register_types(registry);
}
