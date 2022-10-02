#![feature(const_ptr_offset_from, arbitrary_enum_discriminant)]

use std::io::{stdin, BufRead};

use crate::gen::com::book::test1::book::Book;

pub mod gen;

fn main() {
    let mut stdin = stdin().lock();
    let b = stdin.fill_buf().unwrap();
    let book: Book = protokit::binformat::from_slice(b).unwrap();
    println!("{book:?}");
}

#[test]
fn test_textformat() {
    // use protokit::util::{Base, Bits};
    // let text = include_str!("../samples/book.textproto");
    // let mut book = Book::default();
    //
    // protokit::textformat::decode_into(text, &Default::default(), &mut book).unwrap();
    // assert_eq!(
    //     book,
    //     Book {
    //         base: Base {
    //             unknown: Default::default(),
    //             hasbits: Bits::from([0b01000011]),
    //         },
    //         id: id_oneof::local(1),
    //         category: Default::default(),
    //         title: "Zen and the art of motorcycle maintenance".into(),
    //         author: "John Pirsig".into(),
    //         test1: Default::default(),
    //         sections: vec![],
    //         other: Default::default(),
    //     }
    // )
}
