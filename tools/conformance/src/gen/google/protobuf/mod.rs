pub mod duration;
pub mod wrappers;
pub mod r#struct;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    duration::register_types(registry);
    wrappers::register_types(registry);
    r#struct::register_types(registry);
}
