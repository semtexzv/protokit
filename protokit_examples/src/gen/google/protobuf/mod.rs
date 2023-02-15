pub mod duration;
pub mod compiler;
pub mod source_context;
pub mod api;
pub mod r#struct;
pub mod r#type;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    duration::register_types(registry);
    compiler::register_types(registry);
    source_context::register_types(registry);
    api::register_types(registry);
    r#struct::register_types(registry);
    r#type::register_types(registry);
}
