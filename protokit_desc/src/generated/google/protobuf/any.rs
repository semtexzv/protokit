#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;

use crate as root;
use crate::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Any::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Any {
    pub _marker: ::core::marker::PhantomData<()>,
    pub type_url: String,
    pub value: Vec<u8>,
    pub _unknown: (),
}
impl Any {
    #[inline(always)]
    pub fn r#with_type_url(mut self, it: String) -> Self {
        self.r#set_type_url(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type_url(&mut self, it: String) -> &mut Self {
        self.type_url = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_value(mut self, it: Vec<u8>) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: Vec<u8>) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for Any {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("type_url") => {
                textformat::Field::merge(&mut self.type_url, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Any {
    fn encode(&self, ctx: &textformat::Context, pad: usize, out: &mut std::string::String) -> textformat::Result<()> {
        if self.type_url != <String as Default>::default() {
            out.indent(pad);
            out.push_str("type_url: ");
            textformat::Field::format(&self.type_url, ctx, pad, out)?;
            out.push('\n');
        }
        if self.value != <Vec<u8> as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Any {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: ReadBuffer<'b>) -> binformat::Result<ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.type_url, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Any {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Any"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.type_url, 10u32, buf)?;
        Format::<Bytes>::encode(&self.value, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
