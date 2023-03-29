extern crate core;

use std::collections::BTreeMap;
use std::mem::size_of;
pub use anyhow::Result;
pub use binformat::{BinProto, Bytes, Fixed, Sigint, Varint};
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use textformat::{TextField as _, TextProto};
pub use {binformat, textformat};
use textformat::reflect::Registry;


#[derive(Debug, Default, PartialEq, Eq, Proto)]
#[proto(borrow = 'a)]
struct Proto1<'a> {
    #[field(1, "rep", string, singular)]
    rep: &'a str,
    #[field(2, "map", map(varint, varint), repeated)]
    map: BTreeMap<u32, u32>,
    #[oneof([3], ["nest"])]
    oneof: Option<ProtoOneOfFields<'a>>,
}

#[derive(Debug, PartialEq, Eq, Proto)]
#[proto(borrow = 'a)]
enum ProtoOneOfFields<'a> {
    #[field(3, "nest", nested, singular)]
    Nest(Box<Proto1<'a>>),
}

impl<'a> Default for ProtoOneOfFields<'a> {
    fn default() -> Self {
        Self::Nest(Default::default())
    }
}

#[test]
fn test_simple() {
    let mut map = BTreeMap::new();
    map.insert(0, 0);
    let orig = Proto1 {
        rep: "Hello",
        map,
        oneof: Some(ProtoOneOfFields::Nest(Proto1 {
            rep: "World",
            ..Default::default()
        }.into())),
    };
    // panic!("{:?}", size_of::<Proto1>());
    let enc = binformat::encode(&orig).unwrap();
    let dec = binformat::decode(&enc).unwrap();

    assert_eq!(orig, dec);
    // let txt = textformat::encode(&dec, &Registry::default()).unwrap();
    // panic!("{}", txt)
}
