pub mod r#struct;
pub mod wrappers;
pub mod duration;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    r#struct::register_types(registry);
    wrappers::register_types(registry);
    duration::register_types(registry);
}
