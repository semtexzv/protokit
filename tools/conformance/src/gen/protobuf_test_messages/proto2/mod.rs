pub mod test_messages_proto2;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    test_messages_proto2::register_types(registry);
}
