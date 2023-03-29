use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};


#[derive(Default, Debug)]
pub struct Registry {
    pub messages: BTreeMap<&'static str, Box<dyn AnyMessage>>,
}

impl Registry {
    pub fn init(init: fn(&mut Self)) -> Self {
        let mut out = Self::default();
        init(&mut out);
        out
    }
    pub fn register(&mut self, msg: &dyn AnyMessage) {
        self.messages.insert(msg.qualified_name(), msg.new());
    }
}

/// type-erased message that supports all formats of decoding & encoding
pub trait AnyMessage: Send + Sync + 'static {
    fn new(&self) -> Box<dyn AnyMessage + 'static>;
    fn qualified_name(&self) -> &'static str;

    fn as_bin(&self) -> &dyn binformat::BinProto;
    fn as_text(&self) -> &dyn crate::TextProto;

    fn as_debug(&self) -> &dyn Debug;
}

impl Debug for dyn AnyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyMessage").field(&self.qualified_name()).finish()
    }
}

// impl<T> AnyMessage for T
//     where
//         T: Send
//         + Sync
//         + Default
//         + Debug
//         + binformat::BinProto
//         + crate::TextProto
//         + 'static,
// {
//     fn new(&self) -> Box<dyn AnyMessage> {
//         Box::<T>::default()
//     }
//
//     fn as_debug(&self) -> &dyn Debug {
//         self
//     }
// }
//
// impl binformat::BinProto for dyn AnyMessage {
//     fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream) -> binformat::Result<()> {
//
//     }
//
//     fn encode(&self, stream: &mut OutputStream) {
//         todo!()
//     }
// }
//
// impl binformat::Encodable for dyn AnyMessage {
//     fn qualified_name(&self) -> &'static str {
//         self.as_bin_encodable().qualified_name()
//     }
//
//     fn encode(&self, buf: &mut binformat::WriteBuffer) -> crate::Result<()> {
//         self.as_bin_encodable().encode(buf)
//     }
// }
//
// impl crate::Decodable for dyn AnyMessage {
//     fn merge_field(&mut self, ctx: &Context, name: &FieldName, value: &FieldValue) -> crate::Result<()> {
//         self.as_text_decodable().merge_field(ctx, name, value)
//     }
// }
//
// impl crate::Encodable for dyn AnyMessage {
//     fn encode(&self, ctx: &Context, pad: usize, out: &mut String) -> crate::Result<()> {
//         self.as_text_encodable().encode(ctx, pad, out)
//     }
// }
