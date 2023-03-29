extern crate core;

use std::collections::BTreeMap;
pub use anyhow::Result;
pub use binformat::{BinProto, Bytes, Fixed, Sigint, Varint};
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use textformat::{TextField as _, TextProto};
pub use {binformat, textformat};


#[derive(Debug, Default, PartialEq, Eq, Proto)]
struct Proto1 {
    #[field(1, "rep", varint, repeated)]
    rep: Vec<u32>,
    #[field(2, "map", map(varint, varint), repeated)]
    map: BTreeMap<u32, u32>,
}

#[test]
fn test_simple() {
    let mut map = BTreeMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    let orig = Proto1 {
        rep: vec![255, 255, 0, u32::MAX],
        map
    };
    let enc = binformat::encode(&orig).unwrap();
    let dec = binformat::decode(&enc).unwrap();
    assert_eq!(orig, dec)
}