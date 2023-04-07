pub mod book;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    book::register_types(registry);
}
