#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Empty::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Empty {
    pub _marker: ::core::marker::PhantomData<()>,
    pub _unknown: (),
}
impl Empty {}
impl textformat::Decodable for Empty {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Empty {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        Ok(())
    }
}
impl binformat::Decodable for Empty {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Empty {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Empty"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
