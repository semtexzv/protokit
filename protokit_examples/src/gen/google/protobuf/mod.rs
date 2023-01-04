pub mod r#type;
pub mod compiler;
pub mod source_context;
pub mod api;
pub mod r#struct;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    r#type::register_types(registry);
    compiler::register_types(registry);
    source_context::register_types(registry);
    api::register_types(registry);
    r#struct::register_types(registry);
}
