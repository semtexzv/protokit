#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Timestamp::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Timestamp {
    pub _marker: ::core::marker::PhantomData<()>,
    pub seconds: i64,
    pub nanos: i32,
    pub _unknown: (),
}
impl Timestamp {
    #[inline(always)]
    pub fn r#with_seconds(mut self, it: i64) -> Self {
        self.r#set_seconds(it);
        self
    }
    #[inline(always)]
    pub fn r#set_seconds(&mut self, it: i64) -> &mut Self {
        self.seconds = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_nanos(mut self, it: i32) -> Self {
        self.r#set_nanos(it);
        self
    }
    #[inline(always)]
    pub fn r#set_nanos(&mut self, it: i32) -> &mut Self {
        self.nanos = it.into();
        self
    }
}
impl textformat::Decodable for Timestamp {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("seconds") => {
                textformat::Field::merge(&mut self.seconds, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("nanos") => {
                textformat::Field::merge(&mut self.nanos, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Timestamp {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.seconds != <i64 as Default>::default() {
            out.indent(pad);
            out.push_str("seconds: ");
            textformat::Field::format(&self.seconds, ctx, pad, out)?;
            out.push('\n');
        }
        if self.nanos != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("nanos: ");
            textformat::Field::format(&self.nanos, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Timestamp {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<VInt>::decode(&mut self.seconds, buf)?;
            }
            10u32 => {
                buf = Decode::<VInt>::decode(&mut self.seconds, buf)?;
            }
            16u32 => {
                buf = Decode::<VInt>::decode(&mut self.nanos, buf)?;
            }
            18u32 => {
                buf = Decode::<VInt>::decode(&mut self.nanos, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Timestamp {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Timestamp"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<VInt>::encode(&self.seconds, 8u32, buf)?;
        Decode::<VInt>::encode(&self.nanos, 16u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
