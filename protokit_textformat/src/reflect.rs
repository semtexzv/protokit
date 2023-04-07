use core::fmt::{Debug, Formatter};
use std::collections::BTreeMap;

use binformat::{BinProto, InputStream, OutputStream, SizeStack};

use crate::TextProto;

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

    pub fn find(&self, name: &str) -> Option<&Box<dyn AnyMessage>> {
        self.messages.get(name)
    }
}

impl Clone for Box<dyn AnyMessage> {
    fn clone(&self) -> Self {
        self.new()
    }
}

/// type-erased message that supports all formats of decoding & encoding
pub trait AnyMessage: Send + Sync + 'static {
    fn new(&self) -> Box<dyn AnyMessage + 'static>;
    fn qualified_name(&self) -> &'static str;

    fn as_bin<'buf>(&self) -> &dyn binformat::BinProto<'buf>;
    fn as_bin_mut<'buf>(&mut self) -> &mut dyn binformat::BinProto<'buf>;

    fn as_text<'buf>(&self) -> &dyn crate::TextProto<'buf>;
    fn as_text_mut<'buf>(&mut self) -> &mut dyn crate::TextProto<'buf>;
}

impl Debug for dyn AnyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AnyMessage").field(&self.qualified_name()).finish()
    }
}

impl<T> AnyMessage for T
where
    T: for<'buf> BinProto<'buf> + for<'buf> TextProto<'buf>,
    T: Default + Debug + Send + Sync + 'static,
{
    fn new(&self) -> Box<dyn AnyMessage + 'static> {
        Box::new(T::default())
    }

    fn qualified_name(&self) -> &'static str {
        todo!()
    }

    fn as_bin<'buf>(&self) -> &dyn BinProto<'buf> {
        self
    }

    fn as_bin_mut<'buf>(&mut self) -> &mut dyn BinProto<'buf> {
        self
    }

    fn as_text<'buf>(&self) -> &dyn TextProto<'buf> {
        self
    }

    fn as_text_mut<'buf>(&mut self) -> &mut dyn TextProto<'buf> {
        self
    }
}

impl<'buf> BinProto<'buf> for dyn AnyMessage {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> binformat::Result<()> {
        self.as_bin_mut().merge_field(tag_wire, stream)
    }

    fn size(&self, stack: &mut SizeStack) -> usize {
        self.as_bin().size(stack)
    }

    fn encode(&self, stream: &mut OutputStream) {
        self.as_bin().encode(stream)
    }
}

impl<'buf> TextProto<'buf> for dyn AnyMessage {
    fn merge_field(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        self.as_text_mut().merge_field(stream)
    }

    fn encode(&self, stream: &mut crate::OutputStream) {
        self.as_text().encode(stream)
    }
}
