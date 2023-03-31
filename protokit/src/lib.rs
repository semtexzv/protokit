
use std::marker::PhantomData;
use std::ops::Deref;

pub use anyhow::Result;
pub use binformat::{BinProto, Bytes, Fixed, Sigint, Varint};
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use textformat::{TextField as _, TextProto};
pub use {binformat, textformat};

trait Config<'ptr> {
    /// A Pointer type for indirect values
    type Ptr<T: 'ptr>: Deref<Target=T>;
}


pub struct PtrRef<'ptr>(PhantomData<&'ptr ()>);

impl<'ptr> Config<'ptr> for PtrRef<'ptr> {
    type Ptr<T: 'ptr> = &'ptr T;
}

pub struct PtrBox;

impl<'ptr> Config<'ptr> for PtrBox {
    type Ptr<T: 'ptr> = Box<T>;
}


trait ConfigFrom<'ptr, F: Config<'ptr>>: Config<'ptr> {
    /// Convert one pointer from one pointer configuration to other
    /// The Clone bound is actually not needed
    fn config_from<T: Clone>(&self, from: &'ptr T) -> Self::Ptr<T>;
}

impl<'ptr> ConfigFrom<'ptr, PtrRef<'ptr>> for PtrBox {
    #[inline(always)]
    fn config_from<T: Clone>(&self, from: &'ptr T) -> Self::Ptr<T> {
        Self::Ptr::new(from.deref().clone())
    }
}

struct Msg<'a, C: Config<'a>> {
    a: C::Ptr<u32>,
    v: u32,
}

pub trait MapConfig<C> {
    type Output;
    fn ptrconv(&self, c: &C) -> Self::Output;
}

impl<'a, C> MapConfig<C> for Msg<'a, PtrRef<'a>>
    where C: ConfigFrom<'a, PtrRef<'a>>
{
    type Output = Msg<'a, C>;

    fn ptrconv(&self, c: &C) -> Self::Output {
        Msg {
            a: c.config_from(self.a),
            v: self.v,
        }
    }
}


#[cfg(test)]
mod test {
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
