pub mod empty;
pub mod field_mask;
pub mod descriptor;
pub mod timestamp;
pub mod duration;
pub fn register_types(registry: &mut crate::reflect::Registry) {
    empty::register_types(registry);
    field_mask::register_types(registry);
    descriptor::register_types(registry);
    timestamp::register_types(registry);
    duration::register_types(registry);
}
