pub mod empty;
pub mod timestamp;
pub mod compiler;
pub mod duration;
pub mod field_mask;
pub mod descriptor;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    empty::register_types(registry);
    timestamp::register_types(registry);
    compiler::register_types(registry);
    duration::register_types(registry);
    field_mask::register_types(registry);
    descriptor::register_types(registry);
}
