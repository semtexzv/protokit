#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&DoubleValue::default());
    registry.register(&FloatValue::default());
    registry.register(&Int64Value::default());
    registry.register(&UInt64Value::default());
    registry.register(&Int32Value::default());
    registry.register(&UInt32Value::default());
    registry.register(&BoolValue::default());
    registry.register(&StringValue::default());
    registry.register(&BytesValue::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DoubleValue {
    pub value: f64,
    pub _unknown: (),
}
impl DoubleValue {
    #[inline(always)]
    pub fn r#with_value(mut self, it: f64) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: f64) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for DoubleValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DoubleValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <f64 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DoubleValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            9u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DoubleValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.DoubleValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<f64>::eq(&self.value, &Default::default()) {
            Format::<Fix>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatValue {
    pub value: f32,
    pub _unknown: (),
}
impl FloatValue {
    #[inline(always)]
    pub fn r#with_value(mut self, it: f32) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: f32) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for FloatValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FloatValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <f32 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FloatValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            13u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FloatValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FloatValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<f32>::eq(&self.value, &Default::default()) {
            Format::<Fix>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Int64Value {
    pub value: i64,
    pub _unknown: (),
}
impl Int64Value {
    #[inline(always)]
    pub fn r#with_value(mut self, it: i64) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: i64) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for Int64Value {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Int64Value {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <i64 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Int64Value {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Int64Value {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Int64Value"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<i64>::eq(&self.value, &Default::default()) {
            Format::<VInt>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UInt64Value {
    pub value: u64,
    pub _unknown: (),
}
impl UInt64Value {
    #[inline(always)]
    pub fn r#with_value(mut self, it: u64) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: u64) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for UInt64Value {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UInt64Value {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UInt64Value {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UInt64Value {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.UInt64Value"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<u64>::eq(&self.value, &Default::default()) {
            Format::<VInt>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Int32Value {
    pub value: i32,
    pub _unknown: (),
}
impl Int32Value {
    #[inline(always)]
    pub fn r#with_value(mut self, it: i32) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: i32) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for Int32Value {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Int32Value {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Int32Value {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Int32Value {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Int32Value"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<i32>::eq(&self.value, &Default::default()) {
            Format::<VInt>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UInt32Value {
    pub value: u32,
    pub _unknown: (),
}
impl UInt32Value {
    #[inline(always)]
    pub fn r#with_value(mut self, it: u32) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: u32) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for UInt32Value {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UInt32Value {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <u32 as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UInt32Value {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UInt32Value {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.UInt32Value"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<u32>::eq(&self.value, &Default::default()) {
            Format::<VInt>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoolValue {
    pub value: bool,
    pub _unknown: (),
}
impl BoolValue {
    #[inline(always)]
    pub fn r#with_value(mut self, it: bool) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: bool) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for BoolValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for BoolValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for BoolValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for BoolValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.BoolValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<bool>::eq(&self.value, &Default::default()) {
            Format::<Fix>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringValue {
    pub value: String,
    pub _unknown: (),
}
impl StringValue {
    #[inline(always)]
    pub fn r#with_value(mut self, it: String) -> Self {
        self.r#set_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_value(&mut self, it: String) -> &mut Self {
        self.value = it.into();
        self
    }
}
impl textformat::Decodable for StringValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for StringValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <String as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for StringValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for StringValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.StringValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<String>::eq(&self.value, &Default::default()) {
            Format::<Bytes>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BytesValue {
    pub value: Vec<u8>,
    pub _unknown: (),
}
impl BytesValue {
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
impl textformat::Decodable for BytesValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("value") => {
                textformat::Field::merge(&mut self.value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for BytesValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.value != <Vec<u8> as Default>::default() {
            out.indent(pad);
            out.push_str("value: ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for BytesValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for BytesValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.BytesValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        if !PartialEq::<Vec<u8>>::eq(&self.value, &Default::default()) {
            Format::<Bytes>::encode(&self.value, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
