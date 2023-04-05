pub mod google;
pub mod protobuf_test_messages;
pub mod conformance;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    google::register_types(registry);
    protobuf_test_messages::register_types(registry);
    conformance::register_types(registry);
}
