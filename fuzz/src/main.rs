extern crate protokit;
extern crate protokit_examples;

use std::fs::{File, read_dir};
use std::io::Read;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        protokit::binformat::parse::<protokit_examples::gen::google::protobuf::api::Api>(data);
    });
}

#[test]
fn test_crashes() {
    for f in read_dir("./out/default/crashes").unwrap() {
        let f = f.unwrap();
        let mut vec = vec![];
        let d = File::open(f.path()).unwrap().read_to_end(&mut vec).unwrap();
        protokit::binformat::parse::<protokit_examples::gen::google::protobuf::api::Api>(&vec);
    }
    // let dir = include_dir::include_dir!("../../out/default/crashes/");
}