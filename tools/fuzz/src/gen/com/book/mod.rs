pub mod test1;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    test1::register_types(registry);
}
