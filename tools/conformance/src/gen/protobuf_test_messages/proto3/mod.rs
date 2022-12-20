pub mod test_messages_proto3;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    test_messages_proto3::register_types(registry);
}
