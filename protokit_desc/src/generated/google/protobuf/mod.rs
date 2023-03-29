pub mod timestamp;
pub mod duration;
pub mod descriptor;
pub mod field_mask;
pub mod empty;
pub fn register_types(registry: &mut crate::textformat::reflect::Registry) {
    timestamp::register_types(registry);
    duration::register_types(registry);
    descriptor::register_types(registry);
    field_mask::register_types(registry);
    empty::register_types(registry);
}
