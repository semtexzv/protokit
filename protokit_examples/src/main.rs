use std::io::{stdin, BufRead};

pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use crate::gen::com::book::test1::book::Book;

fn main() {
    let mut stdin = stdin().lock();
    let b = stdin.fill_buf().unwrap();
    let book: Book = protokit::binformat::decode(b).unwrap();
    println!("{book:?}");
}

#[test]
fn test_roundtrip() {
    let book = Book {
        title: "Blabla".to_string(),
        author: Some("Other".to_string()),
        x: None,
        pack: vec![0, 1, 2],
        pack2: vec![1.0, 2.0, 3.0],
        category: vec![],
        sections: vec![],
        test1: [("a".to_string(), "b".to_string())].into_iter().collect(),
        other: None,
        extfield: "".to_string(),
        id: Default::default(),
        book: None,
        _unknown: (),
    };

    let v = protokit::binformat::encode(&book).unwrap();
    let dec = protokit::binformat::decode(&v).unwrap();
    assert_eq!(book, dec);
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
