pub mod protobuf_test_messages;
pub mod conformance;
pub mod google;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    protobuf_test_messages::register_types(registry);
    conformance::register_types(registry);
    google::register_types(registry);
}
