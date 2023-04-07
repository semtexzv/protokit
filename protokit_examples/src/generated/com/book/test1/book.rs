#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::super::super::super::google::protobuf::any::*;
use super::super::super::super::google::protobuf::empty::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category(pub u32);
#[protoenum]
impl Category {
    #[var(0u32, "ADULT")]
    pub const ADULT: Category = Category(0u32);
    #[var(1u32, "CHILDREN")]
    pub const CHILDREN: Category = Category(1u32);
    #[var(2u32, "OTHER")]
    pub const OTHER: Category = Category(2u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct BookSection {
    #[field(1u32, "contents", string, optional)]
    pub contents: Option<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct BookTest1Entry {
    #[field(1u32, "key", string, optional)]
    pub key: Option<String>,
    #[field(2u32, "value", string, optional)]
    pub value: Option<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum BookOneOfId {
    #[field(1u32, "local", varint, raw)]
    Local(i32),
    #[field(2u32, "isbn", string, raw)]
    Isbn(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for BookOneOfId {
    fn default() -> Self {
        Self::Local(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum BookOneOfAuthor {
    #[field(4u32, "author", string, raw)]
    Author(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for BookOneOfAuthor {
    fn default() -> Self {
        Self::Author(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum BookOneOfX {
    #[field(231u32, "x", varint, raw)]
    X(i32),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for BookOneOfX {
    fn default() -> Self {
        Self::X(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Book {
    #[field(3u32, "title", string, optional)]
    pub title: Option<String>,
    #[field(323u32, "pack", varint, packed)]
    pub pack: Vec<i32>,
    #[field(3233u32, "pack2", fixed32, packed)]
    pub pack2: Vec<f32>,
    #[field(32u32, "category", protoenum, packed)]
    pub category: Vec<Category>,
    #[field(80u32, "sections", nested, packed)]
    pub sections: Vec<Section>,
    #[field(322u32, "test1", nested, packed)]
    pub test1: Vec<Test1Entry>,
    #[field(35u32, "other", nested, optional)]
    pub other: Option<Box<Any>>,
    #[field(321321u32, "book", nested, optional)]
    pub book: Option<Box<Book>>,
    #[oneof([1u32, 2u32], ["local", "isbn"])]
    pub id: Option<BookOneOfId>,
    #[oneof([4u32], ["author"])]
    pub _author: Option<BookOneOfAuthor>,
    #[oneof([231u32], ["x"])]
    pub _x: Option<BookOneOfX>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
