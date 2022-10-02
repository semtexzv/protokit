use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

use crate::ast::{FieldName, FieldValue};
use crate::Context;

#[derive(Default, Debug)]
pub struct Registry {
    pub messages: BTreeMap<&'static str, Box<dyn AnyMessage>>,
}

impl Registry {
    pub fn register(&mut self, msg: &dyn AnyMessage) {
        self.messages.insert(msg.qualified_name(), msg.new());
    }
}

/// type-erased message that supports all formats of decoding & encoding
pub trait AnyMessage: Send + Sync + 'static {
    fn new(&self) -> Box<dyn AnyMessage + 'static>;
    fn qualified_name(&self) -> &'static str;

    fn as_bin_encodable(&self) -> &dyn binformat::Encodable;
    fn as_bin_decodable(&mut self) -> &mut dyn binformat::Decodable;

    fn as_text_encodable(&self) -> &dyn crate::Encodable;
    fn as_text_decodable(&mut self) -> &mut dyn crate::Decodable;
    fn as_debug(&self) -> &dyn Debug;
}

impl Debug for dyn AnyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyMessage").field(&self.qualified_name()).finish()
    }
}

impl<T> AnyMessage for T
where
    T: Send
        + Sync
        + Default
        + Debug
        + binformat::Encodable
        + binformat::Decodable
        + crate::Encodable
        + crate::Decodable
        + 'static,
{
    fn new(&self) -> Box<dyn AnyMessage> {
        Box::new(Self::default())
    }

    fn qualified_name(&self) -> &'static str {
        binformat::Encodable::qualified_name(self)
    }

    fn as_bin_encodable(&self) -> &dyn binformat::Encodable {
        self
    }

    fn as_bin_decodable(&mut self) -> &mut dyn binformat::Decodable {
        self
    }

    fn as_text_encodable(&self) -> &dyn crate::Encodable {
        self
    }

    fn as_text_decodable(&mut self) -> &mut dyn crate::Decodable {
        self
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }
}

impl binformat::Decodable for dyn AnyMessage {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, buf: &'b [u8]) -> crate::Result<&'b [u8]> {
        self.as_bin_decodable().merge_field(tag, buf)
    }
}
impl binformat::Encodable for dyn AnyMessage {
    fn qualified_name(&self) -> &'static str {
        self.as_bin_encodable().qualified_name()
    }

    fn encode(&self, buf: &mut binformat::Buffer) -> crate::Result<()> {
        self.as_bin_encodable().encode(buf)
    }
}

impl crate::Decodable for dyn AnyMessage {
    fn merge_field(&mut self, ctx: &Context, name: &FieldName, value: &FieldValue) -> crate::Result<()> {
        self.as_text_decodable().merge_field(ctx, name, value)
    }
}

impl crate::Encodable for dyn AnyMessage {
    fn encode(&self, ctx: &Context, pad: usize, out: &mut String) -> crate::Result<()> {
        self.as_text_encodable().encode(ctx, pad, out)
    }
}
