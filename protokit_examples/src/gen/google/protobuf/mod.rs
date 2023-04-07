pub mod source_context;
pub mod descriptor;
pub mod empty;
pub mod timestamp;
pub mod any;
pub mod r#type;
pub mod compiler;
pub mod r#struct;
pub mod api;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    source_context::register_types(registry);
    descriptor::register_types(registry);
    empty::register_types(registry);
    timestamp::register_types(registry);
    any::register_types(registry);
    r#type::register_types(registry);
    compiler::register_types(registry);
    r#struct::register_types(registry);
    api::register_types(registry);
}
