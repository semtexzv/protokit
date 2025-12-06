#![allow(clippy::wrong_self_convention, clippy::new_ret_no_self)]

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use binformat::{BinProto, InputStream, OutputStream, SizeStack};

use crate::TextProto;

#[derive(Default)]
pub struct Registry {
    pub messages: BTreeMap<&'static str, Box<dyn AnyMessage>>,
    pub proxies: BTreeMap<&'static str, Box<dyn crate::TextFormatProxy>>,
    // Map from "ExtendingMessageName" -> "ExtensionName" -> ExtensionInfo
    pub extensions: BTreeMap<String, BTreeMap<String, ExtensionInfo>>,
    // Map from "ExtendingMessageName" -> FieldNumber -> ExtensionInfo (with name)
    pub extensions_by_number: BTreeMap<String, BTreeMap<u32, ExtensionInfo>>,
}

#[derive(Clone, Debug)]
pub struct ExtensionInfo {
    pub field_number: u32,
    pub field_type: u32,
    pub is_repeated: bool,
    pub type_name: String,
    // We need name here for reverse lookup
    pub name: String,
}

impl Debug for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Registry")
            .field("messages", &self.messages)
            .field("proxies", &self.proxies.keys())
            .field("extensions", &self.extensions)
            .finish()
    }
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

    pub fn register_proxy(&mut self, name: &'static str, proxy: Box<dyn crate::TextFormatProxy>) {
        self.proxies.insert(name, proxy);
    }

    pub fn find(&self, name: &str) -> Option<&dyn AnyMessage> {
        self.messages.get(name).map(|v| v.deref())
    }

    pub fn find_proxy(&self, name: &str) -> Option<&dyn crate::TextFormatProxy> {
        self.proxies.get(name).map(|b| b.as_ref())
    }

    pub fn register_extension(
        &mut self,
        extendee: &str,
        field_name: &str,
        number: u32,
        field_type: u32,
        is_repeated: bool,
        type_name: &str,
    ) {
        let info = ExtensionInfo {
            field_number: number,
            field_type,
            is_repeated,
            type_name: type_name.to_string(),
            name: field_name.to_string(),
        };

        self.extensions
            .entry(extendee.to_string())
            .or_default()
            .insert(field_name.to_string(), info.clone());

        self.extensions_by_number
            .entry(extendee.to_string())
            .or_default()
            .insert(number, info);
    }

    pub fn find_extension(&self, extendee: &str, field_name: &str) -> Option<ExtensionInfo> {
        self.extensions.get(extendee)?.get(field_name).cloned()
    }

    pub fn find_extension_by_number(&self, extendee: &str, number: u32) -> Option<ExtensionInfo> {
        self.extensions_by_number.get(extendee)?.get(&number).cloned()
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
        Box::<T>::default()
    }

    fn qualified_name(&self) -> &'static str {
        <T as binformat::ProtoName>::qualified_name(self)
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

impl binformat::ProtoName for dyn AnyMessage {
    fn qualified_name(&self) -> &'static str {
        self.as_bin().qualified_name()
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

    fn decode(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        self.as_text_mut().decode(stream)
    }
}
