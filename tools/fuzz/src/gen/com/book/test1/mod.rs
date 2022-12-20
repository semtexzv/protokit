pub mod fuzz;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    fuzz::register_types(registry);
}
