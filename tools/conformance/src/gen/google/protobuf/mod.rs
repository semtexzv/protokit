pub mod wrappers;
pub mod r#struct;
pub mod duration;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    wrappers::register_types(registry);
    r#struct::register_types(registry);
    duration::register_types(registry);
}
