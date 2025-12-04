use crate::binformat::{self, BinProto, OutputStream, SizeStack, TypedProtoName};
use crate::generated::google::protobuf::any::Any;
use crate::textformat::TextProto;

pub trait AnyExt {
    fn pack<'a, M: TextProto<'a> + BinProto<'a> + TypedProtoName>(&mut self, msg: &M) -> binformat::Result<()>;
    fn unpack<'a, M: TextProto<'a> + BinProto<'a> + TypedProtoName + Default>(&'a self)
        -> binformat::Result<Option<M>>;
}

impl AnyExt for Any {
    fn pack<'a, M: TextProto<'a> + BinProto<'a> + TypedProtoName>(&mut self, msg: &M) -> binformat::Result<()> {
        self.type_url = format!("type.googleapis.com/{}", <M as TypedProtoName>::qualified_name());
        let mut stack = SizeStack::default();
        let size = BinProto::size(msg, &mut stack);
        self.value = vec![0u8; size];
        let mut stream = OutputStream::new(stack, &mut self.value);
        BinProto::encode(msg, &mut stream);
        Ok(())
    }

    fn unpack<'a, M: TextProto<'a> + BinProto<'a> + TypedProtoName + Default>(
        &'a self,
    ) -> binformat::Result<Option<M>> {
        let type_name = if let Some(pos) = self.type_url.rfind('/') {
            &self.type_url[pos + 1..]
        } else {
            &self.type_url
        };

        if type_name != <M as TypedProtoName>::qualified_name() {
            return Ok(None);
        }

        let mut msg = M::default();
        let mut stream = binformat::InputStream::new(&self.value);
        while !stream.is_empty() {
            let tag = stream._varint::<u32>()?;
            BinProto::merge_field(&mut msg, tag, &mut stream)?;
        }
        Ok(Some(msg))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::generated::google::protobuf::timestamp::Timestamp;

    #[test]
    fn test_pack_unpack() {
        let mut any = Any::default();
        let mut ts = Timestamp::default();
        ts.seconds = 123;
        ts.nanos = 456;

        any.pack(&ts).unwrap();
        assert_eq!(any.type_url, "type.googleapis.com/google.protobuf.Timestamp");
        assert!(!any.value.is_empty());

        let unpacked: Option<Timestamp> = any.unpack().unwrap();
        assert!(unpacked.is_some());
        let unpacked = unpacked.unwrap();
        assert_eq!(unpacked.seconds, 123);
        assert_eq!(unpacked.nanos, 456);
    }
}
