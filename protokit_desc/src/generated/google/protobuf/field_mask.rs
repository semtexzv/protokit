#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use crate::*;
use crate as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&FieldMask::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldMask {
    pub paths: Vec<String>,
    pub _unknown: (),
}
impl FieldMask {
    #[inline(always)]
    pub fn r#with_paths(mut self, it: String) -> Self {
        self.r#add_paths(it);
        self
    }
    #[inline(always)]
    pub fn r#add_paths(&mut self, it: String) -> &mut Self {
        self.paths.push(it);
        self
    }
}
impl textformat::Decodable for FieldMask {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("paths") => {
                textformat::Field::merge(&mut self.paths, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FieldMask {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.paths != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("paths: ");
            textformat::Field::format(&self.paths, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldMask {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.paths, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldMask {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FieldMask"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Bytes>>::encode(&self.paths, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
