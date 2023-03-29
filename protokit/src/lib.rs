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
    #[field(1, "rep", string, singular)]
    rep: String,
    #[field(2, "map", map(varint, varint), repeated)]
    map: BTreeMap<u32, u32>,
    #[oneof([3], ["nest"])]
    oneof: Option<ProtoOneOfFields>,
}

#[derive(Debug, PartialEq, Eq, Proto)]
enum ProtoOneOfFields {
    #[field(3, "nest", nested, singular)]
    Nest(Box<Proto1>),
}

impl Default for ProtoOneOfFields {
    fn default() -> Self {
        Self::Nest(Default::default())
    }
}

#[test]
fn test_simple() {
    let mut map = BTreeMap::new();
    map.insert(0, 0);
    let orig = Proto1 {
        rep: vec![1],
        map,
        oneof: Some(ProtoOneOfFields::Nest(Proto1 {
            rep: vec![1],
            ..Default::default()
        }.into())),
    };
    let enc = binformat::encode(&orig).unwrap();
    let dec = binformat::decode(&enc).unwrap();
    assert_eq!(orig, dec)
}
