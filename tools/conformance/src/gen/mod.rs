pub mod conformance;
pub mod protobuf_test_messages;
pub mod validate;
pub mod google;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    conformance::register_types(registry);
    protobuf_test_messages::register_types(registry);
    validate::register_types(registry);
    google::register_types(registry);
}
