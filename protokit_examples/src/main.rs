use std::io::{stdin, BufRead};

// pub mod gen {
//     include!(concat!(env!("OUT_DIR"), "/mod.rs"));
// }
pub mod gen;
use crate::gen::com::book::test1::book::Book;

fn main() {
    let mut stdin = stdin().lock();
    let b = stdin.fill_buf().unwrap();

    let book: Book = protokit::binformat::decode(b).unwrap();
    let out = protokit::textformat::encode(&book).unwrap();
    println!("{out:?}");
}
