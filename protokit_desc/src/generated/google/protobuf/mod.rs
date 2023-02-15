pub mod field_mask;
pub mod duration;
pub mod timestamp;
pub mod descriptor;
pub mod empty;
pub fn register_types(registry: &mut crate::reflect::Registry) {
    field_mask::register_types(registry);
    duration::register_types(registry);
    timestamp::register_types(registry);
    descriptor::register_types(registry);
    empty::register_types(registry);
}
