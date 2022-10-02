#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::any::*;
use super::source_context::*;
use super::source_context::*;
use root::types::any::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Type::default());
    registry.register(&Field::default());
    registry.register(&Enum::default());
    registry.register(&EnumValue::default());
    registry.register(&ProtoOption::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    pub fields: Vec<Field>,
    pub oneofs: Vec<String>,
    pub options: Vec<ProtoOption>,
    pub source_context: Option<Box<SourceContext>>,
    pub syntax: Syntax,
    pub _unknown: (),
}
impl Type {
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
    pub fn r#with_fields(mut self, it: Field) -> Self {
        self.r#add_fields(it);
        self
    }
    #[inline(always)]
    pub fn r#add_fields(&mut self, it: Field) -> &mut Self {
        self.fields.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneofs(mut self, it: String) -> Self {
        self.r#add_oneofs(it);
        self
    }
    #[inline(always)]
    pub fn r#add_oneofs(&mut self, it: String) -> &mut Self {
        self.oneofs.push(it);
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
impl textformat::Decodable for Type {
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
            textformat::ast::FieldName::Normal("fields") => {
                textformat::Field::merge(&mut self.fields, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oneofs") => {
                textformat::Field::merge(&mut self.oneofs, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("source_context") => {
                textformat::Field::merge(&mut self.source_context, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("syntax") => {
                textformat::Field::merge(&mut self.syntax, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Type {
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
        if self.fields != <Vec<Field> as Default>::default() {
            out.indent(pad);
            out.push_str("fields ");
            textformat::Field::format(&self.fields, ctx, pad, out)?;
            out.push('\n');
        }
        if self.oneofs != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("oneofs: ");
            textformat::Field::format(&self.oneofs, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.source_context != <Option<Box<SourceContext>> as Default>::default() {
            out.indent(pad);
            out.push_str("source_context ");
            textformat::Field::format(&self.source_context, ctx, pad, out)?;
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
impl binformat::Decodable for Type {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.fields, buf)?;
            }
            26u32 => {
                buf = Decode::<Repeat::<Bytes>>::decode(&mut self.oneofs, buf)?;
            }
            34u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            42u32 => {
                buf = Decode::<Nest>::decode(&mut self.source_context, buf)?;
            }
            48u32 => {
                buf = Decode::<Enum>::decode(&mut self.syntax, buf)?;
            }
            50u32 => {
                buf = Decode::<Enum>::decode(&mut self.syntax, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Type {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Type"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.name, 10u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.fields, 18u32, buf)?;
        Decode::<Repeat::<Bytes>>::encode(&self.oneofs, 26u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.options, 34u32, buf)?;
        Decode::<Nest>::encode(&self.source_context, 42u32, buf)?;
        Decode::<Enum>::encode(&self.syntax, 48u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pub kind: FieldKind,
    pub cardinality: FieldCardinality,
    pub number: i32,
    pub name: String,
    pub type_url: String,
    pub oneof_index: i32,
    pub packed: bool,
    pub options: Vec<ProtoOption>,
    pub json_name: String,
    pub default_value: String,
    pub _unknown: (),
}
impl Field {
    #[inline(always)]
    pub fn r#with_kind(mut self, it: FieldKind) -> Self {
        self.r#set_kind(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind(&mut self, it: FieldKind) -> &mut Self {
        self.kind = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_cardinality(mut self, it: FieldCardinality) -> Self {
        self.r#set_cardinality(it);
        self
    }
    #[inline(always)]
    pub fn r#set_cardinality(&mut self, it: FieldCardinality) -> &mut Self {
        self.cardinality = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_number(mut self, it: i32) -> Self {
        self.r#set_number(it);
        self
    }
    #[inline(always)]
    pub fn r#set_number(&mut self, it: i32) -> &mut Self {
        self.number = it.into();
        self
    }
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
    pub fn r#with_oneof_index(mut self, it: i32) -> Self {
        self.r#set_oneof_index(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oneof_index(&mut self, it: i32) -> &mut Self {
        self.oneof_index = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_packed(mut self, it: bool) -> Self {
        self.r#set_packed(it);
        self
    }
    #[inline(always)]
    pub fn r#set_packed(&mut self, it: bool) -> &mut Self {
        self.packed = it.into();
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
    pub fn r#with_json_name(mut self, it: String) -> Self {
        self.r#set_json_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_json_name(&mut self, it: String) -> &mut Self {
        self.json_name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_default_value(mut self, it: String) -> Self {
        self.r#set_default_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_default_value(&mut self, it: String) -> &mut Self {
        self.default_value = it.into();
        self
    }
}
impl textformat::Decodable for Field {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("kind") => {
                textformat::Field::merge(&mut self.kind, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("cardinality") => {
                textformat::Field::merge(&mut self.cardinality, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("number") => {
                textformat::Field::merge(&mut self.number, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type_url") => {
                textformat::Field::merge(&mut self.type_url, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oneof_index") => {
                textformat::Field::merge(&mut self.oneof_index, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed") => {
                textformat::Field::merge(&mut self.packed, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("json_name") => {
                textformat::Field::merge(&mut self.json_name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_value") => {
                textformat::Field::merge(&mut self.default_value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Field {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.kind != <FieldKind as Default>::default() {
            out.indent(pad);
            out.push_str("kind: ");
            textformat::Field::format(&self.kind, ctx, pad, out)?;
            out.push('\n');
        }
        if self.cardinality != <FieldCardinality as Default>::default() {
            out.indent(pad);
            out.push_str("cardinality: ");
            textformat::Field::format(&self.cardinality, ctx, pad, out)?;
            out.push('\n');
        }
        if self.number != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("number: ");
            textformat::Field::format(&self.number, ctx, pad, out)?;
            out.push('\n');
        }
        if self.name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.type_url != <String as Default>::default() {
            out.indent(pad);
            out.push_str("type_url: ");
            textformat::Field::format(&self.type_url, ctx, pad, out)?;
            out.push('\n');
        }
        if self.oneof_index != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("oneof_index: ");
            textformat::Field::format(&self.oneof_index, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("packed: ");
            textformat::Field::format(&self.packed, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.json_name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("json_name: ");
            textformat::Field::format(&self.json_name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_value != <String as Default>::default() {
            out.indent(pad);
            out.push_str("default_value: ");
            textformat::Field::format(&self.default_value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Field {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<Enum>::decode(&mut self.kind, buf)?;
            }
            10u32 => {
                buf = Decode::<Enum>::decode(&mut self.kind, buf)?;
            }
            16u32 => {
                buf = Decode::<Enum>::decode(&mut self.cardinality, buf)?;
            }
            18u32 => {
                buf = Decode::<Enum>::decode(&mut self.cardinality, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.number, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.number, buf)?;
            }
            34u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            50u32 => {
                buf = Decode::<Bytes>::decode(&mut self.type_url, buf)?;
            }
            56u32 => {
                buf = Decode::<VInt>::decode(&mut self.oneof_index, buf)?;
            }
            58u32 => {
                buf = Decode::<VInt>::decode(&mut self.oneof_index, buf)?;
            }
            64u32 => {
                buf = Decode::<Fix>::decode(&mut self.packed, buf)?;
            }
            66u32 => {
                buf = Decode::<Fix>::decode(&mut self.packed, buf)?;
            }
            74u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            82u32 => {
                buf = Decode::<Bytes>::decode(&mut self.json_name, buf)?;
            }
            90u32 => {
                buf = Decode::<Bytes>::decode(&mut self.default_value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Field {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Field"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Enum>::encode(&self.kind, 8u32, buf)?;
        Decode::<Enum>::encode(&self.cardinality, 16u32, buf)?;
        Decode::<VInt>::encode(&self.number, 24u32, buf)?;
        Decode::<Bytes>::encode(&self.name, 34u32, buf)?;
        Decode::<Bytes>::encode(&self.type_url, 50u32, buf)?;
        Decode::<VInt>::encode(&self.oneof_index, 56u32, buf)?;
        Decode::<Fix>::encode(&self.packed, 64u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.options, 74u32, buf)?;
        Decode::<Bytes>::encode(&self.json_name, 82u32, buf)?;
        Decode::<Bytes>::encode(&self.default_value, 90u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub enumvalue: Vec<EnumValue>,
    pub options: Vec<ProtoOption>,
    pub source_context: Option<Box<SourceContext>>,
    pub syntax: Syntax,
    pub _unknown: (),
}
impl Enum {
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
    pub fn r#with_enumvalue(mut self, it: EnumValue) -> Self {
        self.r#add_enumvalue(it);
        self
    }
    #[inline(always)]
    pub fn r#add_enumvalue(&mut self, it: EnumValue) -> &mut Self {
        self.enumvalue.push(it);
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
impl textformat::Decodable for Enum {
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
            textformat::ast::FieldName::Normal("enumvalue") => {
                textformat::Field::merge(&mut self.enumvalue, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("source_context") => {
                textformat::Field::merge(&mut self.source_context, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("syntax") => {
                textformat::Field::merge(&mut self.syntax, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Enum {
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
        if self.enumvalue != <Vec<EnumValue> as Default>::default() {
            out.indent(pad);
            out.push_str("enumvalue ");
            textformat::Field::format(&self.enumvalue, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.source_context != <Option<Box<SourceContext>> as Default>::default() {
            out.indent(pad);
            out.push_str("source_context ");
            textformat::Field::format(&self.source_context, ctx, pad, out)?;
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
impl binformat::Decodable for Enum {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.enumvalue, buf)?;
            }
            26u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            34u32 => {
                buf = Decode::<Nest>::decode(&mut self.source_context, buf)?;
            }
            40u32 => {
                buf = Decode::<Enum>::decode(&mut self.syntax, buf)?;
            }
            42u32 => {
                buf = Decode::<Enum>::decode(&mut self.syntax, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Enum {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Enum"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.name, 10u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.enumvalue, 18u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.options, 26u32, buf)?;
        Decode::<Nest>::encode(&self.source_context, 34u32, buf)?;
        Decode::<Enum>::encode(&self.syntax, 40u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
    pub options: Vec<ProtoOption>,
    pub _unknown: (),
}
impl EnumValue {
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
    pub fn r#with_number(mut self, it: i32) -> Self {
        self.r#set_number(it);
        self
    }
    #[inline(always)]
    pub fn r#set_number(&mut self, it: i32) -> &mut Self {
        self.number = it.into();
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
}
impl textformat::Decodable for EnumValue {
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
            textformat::ast::FieldName::Normal("number") => {
                textformat::Field::merge(&mut self.number, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumValue {
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
        if self.number != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("number: ");
            textformat::Field::format(&self.number, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Vec<ProtoOption> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            16u32 => {
                buf = Decode::<VInt>::decode(&mut self.number, buf)?;
            }
            18u32 => {
                buf = Decode::<VInt>::decode(&mut self.number, buf)?;
            }
            26u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.options, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.name, 10u32, buf)?;
        Decode::<VInt>::encode(&self.number, 16u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.options, 26u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProtoOption {
    pub name: String,
    pub value: Option<Box<Any>>,
    pub _unknown: (),
}
impl ProtoOption {
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
    pub fn r#with_value(mut self, it: Any) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: Any) -> &mut Self {
        self.value = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for ProtoOption {
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
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ProtoOption {
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
        if self.value != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("value ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ProtoOption {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ProtoOption {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Option"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.name, 10u32, buf)?;
        Decode::<Nest>::encode(&self.value, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    TYPE_UNKNOWN,
    TYPE_DOUBLE,
    TYPE_FLOAT,
    TYPE_INT64,
    TYPE_UINT64,
    TYPE_INT32,
    TYPE_FIXED64,
    TYPE_FIXED32,
    TYPE_BOOL,
    TYPE_STRING,
    TYPE_GROUP,
    TYPE_MESSAGE,
    TYPE_BYTES,
    TYPE_UINT32,
    TYPE_ENUM,
    TYPE_SFIXED32,
    TYPE_SFIXED64,
    TYPE_SINT32,
    TYPE_SINT64,
    Unknown(u32),
}
impl Default for FieldKind {
    fn default() -> FieldKind {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldKind {}
impl From<u32> for FieldKind {
    fn from(v: u32) -> FieldKind {
        match v {
            0u32 => FieldKind::TYPE_UNKNOWN,
            1u32 => FieldKind::TYPE_DOUBLE,
            2u32 => FieldKind::TYPE_FLOAT,
            3u32 => FieldKind::TYPE_INT64,
            4u32 => FieldKind::TYPE_UINT64,
            5u32 => FieldKind::TYPE_INT32,
            6u32 => FieldKind::TYPE_FIXED64,
            7u32 => FieldKind::TYPE_FIXED32,
            8u32 => FieldKind::TYPE_BOOL,
            9u32 => FieldKind::TYPE_STRING,
            10u32 => FieldKind::TYPE_GROUP,
            11u32 => FieldKind::TYPE_MESSAGE,
            12u32 => FieldKind::TYPE_BYTES,
            13u32 => FieldKind::TYPE_UINT32,
            14u32 => FieldKind::TYPE_ENUM,
            15u32 => FieldKind::TYPE_SFIXED32,
            16u32 => FieldKind::TYPE_SFIXED64,
            17u32 => FieldKind::TYPE_SINT32,
            18u32 => FieldKind::TYPE_SINT64,
            other => FieldKind::Unknown(other),
        }
    }
}
impl From<FieldKind> for u32 {
    fn from(v: FieldKind) -> u32 {
        match v {
            FieldKind::TYPE_UNKNOWN => 0u32,
            FieldKind::TYPE_DOUBLE => 1u32,
            FieldKind::TYPE_FLOAT => 2u32,
            FieldKind::TYPE_INT64 => 3u32,
            FieldKind::TYPE_UINT64 => 4u32,
            FieldKind::TYPE_INT32 => 5u32,
            FieldKind::TYPE_FIXED64 => 6u32,
            FieldKind::TYPE_FIXED32 => 7u32,
            FieldKind::TYPE_BOOL => 8u32,
            FieldKind::TYPE_STRING => 9u32,
            FieldKind::TYPE_GROUP => 10u32,
            FieldKind::TYPE_MESSAGE => 11u32,
            FieldKind::TYPE_BYTES => 12u32,
            FieldKind::TYPE_UINT32 => 13u32,
            FieldKind::TYPE_ENUM => 14u32,
            FieldKind::TYPE_SFIXED32 => 15u32,
            FieldKind::TYPE_SFIXED64 => 16u32,
            FieldKind::TYPE_SINT32 => 17u32,
            FieldKind::TYPE_SINT64 => 18u32,
            FieldKind::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldKind {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldKind::TYPE_UNKNOWN => "TYPE_UNKNOWN",
            FieldKind::TYPE_DOUBLE => "TYPE_DOUBLE",
            FieldKind::TYPE_FLOAT => "TYPE_FLOAT",
            FieldKind::TYPE_INT64 => "TYPE_INT64",
            FieldKind::TYPE_UINT64 => "TYPE_UINT64",
            FieldKind::TYPE_INT32 => "TYPE_INT32",
            FieldKind::TYPE_FIXED64 => "TYPE_FIXED64",
            FieldKind::TYPE_FIXED32 => "TYPE_FIXED32",
            FieldKind::TYPE_BOOL => "TYPE_BOOL",
            FieldKind::TYPE_STRING => "TYPE_STRING",
            FieldKind::TYPE_GROUP => "TYPE_GROUP",
            FieldKind::TYPE_MESSAGE => "TYPE_MESSAGE",
            FieldKind::TYPE_BYTES => "TYPE_BYTES",
            FieldKind::TYPE_UINT32 => "TYPE_UINT32",
            FieldKind::TYPE_ENUM => "TYPE_ENUM",
            FieldKind::TYPE_SFIXED32 => "TYPE_SFIXED32",
            FieldKind::TYPE_SFIXED64 => "TYPE_SFIXED64",
            FieldKind::TYPE_SINT32 => "TYPE_SINT32",
            FieldKind::TYPE_SINT64 => "TYPE_SINT64",
            FieldKind::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("TYPE_UNKNOWN") => {
                *self = FieldKind::TYPE_UNKNOWN;
            }
            textformat::ast::Literal::Identifier("TYPE_DOUBLE") => {
                *self = FieldKind::TYPE_DOUBLE;
            }
            textformat::ast::Literal::Identifier("TYPE_FLOAT") => {
                *self = FieldKind::TYPE_FLOAT;
            }
            textformat::ast::Literal::Identifier("TYPE_INT64") => {
                *self = FieldKind::TYPE_INT64;
            }
            textformat::ast::Literal::Identifier("TYPE_UINT64") => {
                *self = FieldKind::TYPE_UINT64;
            }
            textformat::ast::Literal::Identifier("TYPE_INT32") => {
                *self = FieldKind::TYPE_INT32;
            }
            textformat::ast::Literal::Identifier("TYPE_FIXED64") => {
                *self = FieldKind::TYPE_FIXED64;
            }
            textformat::ast::Literal::Identifier("TYPE_FIXED32") => {
                *self = FieldKind::TYPE_FIXED32;
            }
            textformat::ast::Literal::Identifier("TYPE_BOOL") => {
                *self = FieldKind::TYPE_BOOL;
            }
            textformat::ast::Literal::Identifier("TYPE_STRING") => {
                *self = FieldKind::TYPE_STRING;
            }
            textformat::ast::Literal::Identifier("TYPE_GROUP") => {
                *self = FieldKind::TYPE_GROUP;
            }
            textformat::ast::Literal::Identifier("TYPE_MESSAGE") => {
                *self = FieldKind::TYPE_MESSAGE;
            }
            textformat::ast::Literal::Identifier("TYPE_BYTES") => {
                *self = FieldKind::TYPE_BYTES;
            }
            textformat::ast::Literal::Identifier("TYPE_UINT32") => {
                *self = FieldKind::TYPE_UINT32;
            }
            textformat::ast::Literal::Identifier("TYPE_ENUM") => {
                *self = FieldKind::TYPE_ENUM;
            }
            textformat::ast::Literal::Identifier("TYPE_SFIXED32") => {
                *self = FieldKind::TYPE_SFIXED32;
            }
            textformat::ast::Literal::Identifier("TYPE_SFIXED64") => {
                *self = FieldKind::TYPE_SFIXED64;
            }
            textformat::ast::Literal::Identifier("TYPE_SINT32") => {
                *self = FieldKind::TYPE_SINT32;
            }
            textformat::ast::Literal::Identifier("TYPE_SINT64") => {
                *self = FieldKind::TYPE_SINT64;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldCardinality {
    CARDINALITY_UNKNOWN,
    CARDINALITY_OPTIONAL,
    CARDINALITY_REQUIRED,
    CARDINALITY_REPEATED,
    Unknown(u32),
}
impl Default for FieldCardinality {
    fn default() -> FieldCardinality {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldCardinality {}
impl From<u32> for FieldCardinality {
    fn from(v: u32) -> FieldCardinality {
        match v {
            0u32 => FieldCardinality::CARDINALITY_UNKNOWN,
            1u32 => FieldCardinality::CARDINALITY_OPTIONAL,
            2u32 => FieldCardinality::CARDINALITY_REQUIRED,
            3u32 => FieldCardinality::CARDINALITY_REPEATED,
            other => FieldCardinality::Unknown(other),
        }
    }
}
impl From<FieldCardinality> for u32 {
    fn from(v: FieldCardinality) -> u32 {
        match v {
            FieldCardinality::CARDINALITY_UNKNOWN => 0u32,
            FieldCardinality::CARDINALITY_OPTIONAL => 1u32,
            FieldCardinality::CARDINALITY_REQUIRED => 2u32,
            FieldCardinality::CARDINALITY_REPEATED => 3u32,
            FieldCardinality::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldCardinality {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldCardinality::CARDINALITY_UNKNOWN => "CARDINALITY_UNKNOWN",
            FieldCardinality::CARDINALITY_OPTIONAL => "CARDINALITY_OPTIONAL",
            FieldCardinality::CARDINALITY_REQUIRED => "CARDINALITY_REQUIRED",
            FieldCardinality::CARDINALITY_REPEATED => "CARDINALITY_REPEATED",
            FieldCardinality::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("CARDINALITY_UNKNOWN") => {
                *self = FieldCardinality::CARDINALITY_UNKNOWN;
            }
            textformat::ast::Literal::Identifier("CARDINALITY_OPTIONAL") => {
                *self = FieldCardinality::CARDINALITY_OPTIONAL;
            }
            textformat::ast::Literal::Identifier("CARDINALITY_REQUIRED") => {
                *self = FieldCardinality::CARDINALITY_REQUIRED;
            }
            textformat::ast::Literal::Identifier("CARDINALITY_REPEATED") => {
                *self = FieldCardinality::CARDINALITY_REPEATED;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    SYNTAX_PROTO2,
    SYNTAX_PROTO3,
    Unknown(u32),
}
impl Default for Syntax {
    fn default() -> Syntax {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for Syntax {}
impl From<u32> for Syntax {
    fn from(v: u32) -> Syntax {
        match v {
            0u32 => Syntax::SYNTAX_PROTO2,
            1u32 => Syntax::SYNTAX_PROTO3,
            other => Syntax::Unknown(other),
        }
    }
}
impl From<Syntax> for u32 {
    fn from(v: Syntax) -> u32 {
        match v {
            Syntax::SYNTAX_PROTO2 => 0u32,
            Syntax::SYNTAX_PROTO3 => 1u32,
            Syntax::Unknown(other) => other,
        }
    }
}
impl textformat::Field for Syntax {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            Syntax::SYNTAX_PROTO2 => "SYNTAX_PROTO2",
            Syntax::SYNTAX_PROTO3 => "SYNTAX_PROTO3",
            Syntax::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("SYNTAX_PROTO2") => {
                *self = Syntax::SYNTAX_PROTO2;
            }
            textformat::ast::Literal::Identifier("SYNTAX_PROTO3") => {
                *self = Syntax::SYNTAX_PROTO3;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
