pub mod descriptor;
pub mod empty;
pub mod field_mask;
pub mod timestamp;
pub mod duration;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    descriptor::register_types(registry);
    empty::register_types(registry);
    field_mask::register_types(registry);
    timestamp::register_types(registry);
    duration::register_types(registry);
}
