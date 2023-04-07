pub use anyhow::Result;
pub use binformat::{BinProto, Bytes, Fixed, Sigint, Varint};
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use textformat::{TextField as _, TextProto};
pub use {binformat, textformat};

#[cfg(test)]
mod test {
    use core::collections::BTreeMap;
    use std::collections::BTreeMap;

    use textformat::reflect::Registry;

    use crate::{Proto1, ProtoOneOfFields};

    #[test]
    fn test_simple() {
        let mut map = BTreeMap::new();
        map.insert(0, 0);
        let orig = Proto1 {
            rep: "Hello",
            map,
            oneof: Some(ProtoOneOfFields::Nest(
                Proto1 {
                    rep: "World",
                    ..Default::default()
                }
                .into(),
            )),
        };
        let enc = binformat::encode(&orig).unwrap();
        let dec = binformat::decode(&enc).unwrap();

        assert_eq!(orig, dec);
        let txt = textformat::encode(&dec, &Registry::default()).unwrap();
        panic!("{}", txt)
    }

    #[test]
    fn test_text() {
        let mut txt = r#"
nest {
    rep: "World"
}
"#;
        textformat::decode::<Proto1>(&txt, &Registry::default()).unwrap();
    }
}
