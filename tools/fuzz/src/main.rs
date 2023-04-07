use core::fs::{read_dir, File};
use core::io::Read;

use protokit::reflect::Registry;

pub mod gen;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        let mut reg = Registry::default();
        gen::register_types(&mut reg);
        if let Ok(s) = core::str::from_utf8(data) {
            protokit::textformat::decode::<crate::gen::com::book::test1::fuzz::Book>(s, &reg);
        }
    });
}

#[test]
fn test_crashes() {
    let mut reg = Registry::default();
    gen::register_types(&mut reg);

    for f in read_dir("./out/default/crashes").unwrap() {
        let f = f.unwrap();
        let mut vec = String::default();
        let d = File::open(f.path()).unwrap().read_to_string(&mut vec).unwrap();
        protokit::textformat::decode::<crate::gen::com::book::test1::fuzz::Book>(&vec, &reg);
    }
    // let dir = include_dir::include_dir!("../../out/default/crashes/");
}
