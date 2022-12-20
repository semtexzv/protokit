#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use super::source_context::*;
use super::r#type::*;
use super::r#type::*;
use super::source_context::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Api::default());
    registry.register(&Method::default());
    registry.register(&Mixin::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Api {
    pub name: String,
    pub methods: Vec<Method>,
    pub options: Vec<ProtoOption>,
    pub version: String,
    pub source_context: Option<Box<SourceContext>>,
    pub mixins: Vec<Mixin>,
    pub syntax: Syntax,
    pub _unknown: (),
}
impl Api {
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_methods(mut self, it: Method) -> Self {
        self.r#add_methods(it);
        self
    }
    #[inline(always)]
    pub fn r#add_methods(&mut self, it: Method) -> &mut Self {
        self.methods.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: ProtoOption) -> Self {
        self.r#add_options(it);
        self
    }
    #[inline(always)]
    pub fn r#add_options(&mut self, it: ProtoOption) -> &mut Self {
        self.options.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_version(mut self, it: String) -> Self {
        self.r#set_version(it);
        self
    }
    #[inline(always)]
    pub fn r#set_version(&mut self, it: String) -> &mut Self {
        self.version = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_source_context(mut self, it: SourceContext) -> Self {
        self.r#set_source_context(it);
        self
    }
    #[inline(always)]
    pub fn r#set_source_context(&mut self, it: SourceContext) -> &mut Self {
        self.source_context = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_mixins(mut self, it: Mixin) -> Self {
        self.r#add_mixins(it);
        self
    }
    #[inline(always)]
    pub fn r#add_mixins(&mut self, it: Mixin) -> &mut Self {
        self.mixins.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_syntax(mut self, it: Syntax) -> Self {
        self.r#set_syntax(it);
        self
    }
    #[inline(always)]
    pub fn r#set_syntax(&mut self, it: Syntax) -> &mut Self {
        self.syntax = it.into();
        self
    }
}
impl textformat::Decodable for Api {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("methods") => {
                textformat::Field::merge(&mut self.methods, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("version") => {
                textformat::Field::merge(&mut self.version, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("source_context") => {
                textformat::Field::merge(&mut self.source_context, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("mixins") => {
                textformat::Field::merge(&mut self.mixins, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("syntax") => {
                textformat::Field::merge(&mut self.syntax, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Api {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.methods != <Vec<Method> as Default>::default() {
            out.indent(pad);
            out.push_str("methods ");
            textformat::Field::format(&self.methods, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.version != <String as Default>::default() {
            out.indent(pad);
            out.push_str("version: ");
            textformat::Field::format(&self.version, ctx, pad, out)?;
            out.push('\n');
        }
        if self.source_context != <Option<Box<SourceContext>> as Default>::default() {
            out.indent(pad);
            out.push_str("source_context ");
            textformat::Field::format(&self.source_context, ctx, pad, out)?;
            out.push('\n');
        }
        if self.mixins != <Vec<Mixin> as Default>::default() {
            out.indent(pad);
            out.push_str("mixins ");
            textformat::Field::format(&self.mixins, ctx, pad, out)?;
            out.push('\n');
        }
        if self.syntax != <Syntax as Default>::default() {
            out.indent(pad);
            out.push_str("syntax: ");
            textformat::Field::format(&self.syntax, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Api {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.methods, buf)?;
            }
            26u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.version, buf)?;
            }
            42u32 => {
                buf = Format::<Nest>::decode(&mut self.source_context, buf)?;
            }
            50u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.mixins, buf)?;
            }
            56u32 => {
                buf = Format::<Enum>::decode(&mut self.syntax, buf)?;
            }
            58u32 => {
                buf = Format::<Enum>::decode(&mut self.syntax, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Api {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Api"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.methods, 18u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.options, 26u32, buf)?;
        Format::<Bytes>::encode(&self.version, 34u32, buf)?;
        Format::<Nest>::encode(&self.source_context, 42u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.mixins, 50u32, buf)?;
        Format::<Enum>::encode(&self.syntax, 56u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub request_type_url: String,
    pub request_streaming: bool,
    pub response_type_url: String,
    pub response_streaming: bool,
    pub options: Vec<ProtoOption>,
    pub syntax: Syntax,
    pub _unknown: (),
}
impl Method {
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_request_type_url(mut self, it: String) -> Self {
        self.r#set_request_type_url(it);
        self
    }
    #[inline(always)]
    pub fn r#set_request_type_url(&mut self, it: String) -> &mut Self {
        self.request_type_url = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_request_streaming(mut self, it: bool) -> Self {
        self.r#set_request_streaming(it);
        self
    }
    #[inline(always)]
    pub fn r#set_request_streaming(&mut self, it: bool) -> &mut Self {
        self.request_streaming = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_response_type_url(mut self, it: String) -> Self {
        self.r#set_response_type_url(it);
        self
    }
    #[inline(always)]
    pub fn r#set_response_type_url(&mut self, it: String) -> &mut Self {
        self.response_type_url = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_response_streaming(mut self, it: bool) -> Self {
        self.r#set_response_streaming(it);
        self
    }
    #[inline(always)]
    pub fn r#set_response_streaming(&mut self, it: bool) -> &mut Self {
        self.response_streaming = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: ProtoOption) -> Self {
        self.r#add_options(it);
        self
    }
    #[inline(always)]
    pub fn r#add_options(&mut self, it: ProtoOption) -> &mut Self {
        self.options.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_syntax(mut self, it: Syntax) -> Self {
        self.r#set_syntax(it);
        self
    }
    #[inline(always)]
    pub fn r#set_syntax(&mut self, it: Syntax) -> &mut Self {
        self.syntax = it.into();
        self
    }
}
impl textformat::Decodable for Method {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("request_type_url") => {
                textformat::Field::merge(&mut self.request_type_url, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("request_streaming") => {
                textformat::Field::merge(&mut self.request_streaming, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("response_type_url") => {
                textformat::Field::merge(&mut self.response_type_url, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("response_streaming") => {
                textformat::Field::merge(&mut self.response_streaming, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("syntax") => {
                textformat::Field::merge(&mut self.syntax, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Method {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.request_type_url != <String as Default>::default() {
            out.indent(pad);
            out.push_str("request_type_url: ");
            textformat::Field::format(&self.request_type_url, ctx, pad, out)?;
            out.push('\n');
        }
        if self.request_streaming != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("request_streaming: ");
            textformat::Field::format(&self.request_streaming, ctx, pad, out)?;
            out.push('\n');
        }
        if self.response_type_url != <String as Default>::default() {
            out.indent(pad);
            out.push_str("response_type_url: ");
            textformat::Field::format(&self.response_type_url, ctx, pad, out)?;
            out.push('\n');
        }
        if self.response_streaming != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("response_streaming: ");
            textformat::Field::format(&self.response_streaming, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.syntax != <Syntax as Default>::default() {
            out.indent(pad);
            out.push_str("syntax: ");
            textformat::Field::format(&self.syntax, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Method {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.request_type_url, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.request_streaming, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.request_streaming, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.response_type_url, buf)?;
            }
            40u32 => {
                buf = Format::<Fix>::decode(&mut self.response_streaming, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.response_streaming, buf)?;
            }
            50u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            56u32 => {
                buf = Format::<Enum>::decode(&mut self.syntax, buf)?;
            }
            58u32 => {
                buf = Format::<Enum>::decode(&mut self.syntax, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Method {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Method"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Bytes>::encode(&self.request_type_url, 18u32, buf)?;
        Format::<Fix>::encode(&self.request_streaming, 24u32, buf)?;
        Format::<Bytes>::encode(&self.response_type_url, 34u32, buf)?;
        Format::<Fix>::encode(&self.response_streaming, 40u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.options, 50u32, buf)?;
        Format::<Enum>::encode(&self.syntax, 56u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mixin {
    pub name: String,
    pub root: String,
    pub _unknown: (),
}
impl Mixin {
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_root(mut self, it: String) -> Self {
        self.r#set_root(it);
        self
    }
    #[inline(always)]
    pub fn r#set_root(&mut self, it: String) -> &mut Self {
        self.root = it.into();
        self
    }
}
impl textformat::Decodable for Mixin {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("root") => {
                textformat::Field::merge(&mut self.root, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Mixin {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.root != <String as Default>::default() {
            out.indent(pad);
            out.push_str("root: ");
            textformat::Field::format(&self.root, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Mixin {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.root, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Mixin {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Mixin"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Bytes>::encode(&self.root, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
