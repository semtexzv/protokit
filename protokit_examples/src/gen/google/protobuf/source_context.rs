#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&SourceContext::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SourceContext {
    pub file_name: String,
    pub _unknown: (),
}
impl SourceContext {
    #[inline(always)]
    pub fn r#with_file_name(mut self, it: String) -> Self {
        self.r#set_file_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_file_name(&mut self, it: String) -> &mut Self {
        self.file_name = it.into();
        self
    }
}
impl textformat::Decodable for SourceContext {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("file_name") => {
                textformat::Field::merge(&mut self.file_name, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SourceContext {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.file_name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("file_name: ");
            textformat::Field::format(&self.file_name, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SourceContext {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.file_name, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SourceContext {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.SourceContext"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.file_name, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
