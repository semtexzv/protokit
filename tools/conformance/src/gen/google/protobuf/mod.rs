pub mod field_mask;
pub mod any;
pub mod wrappers;
pub mod r#struct;
pub mod timestamp;
pub mod duration;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    field_mask::register_types(registry);
    any::register_types(registry);
    wrappers::register_types(registry);
    r#struct::register_types(registry);
    timestamp::register_types(registry);
    duration::register_types(registry);
}
