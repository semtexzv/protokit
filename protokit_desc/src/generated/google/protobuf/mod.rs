pub mod timestamp;
pub mod descriptor;
pub mod duration;
pub mod empty;
pub mod field_mask;
pub fn register_types(registry: &mut crate::reflect::Registry) {
    timestamp::register_types(registry);
    descriptor::register_types(registry);
    duration::register_types(registry);
    empty::register_types(registry);
    field_mask::register_types(registry);
}
