#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Struct::default());
    registry.register(&Value::default());
    registry.register(&ListValue::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct {
    pub fields: ::std::collections::HashMap<String, Value>,
    pub _unknown: (),
}
impl Struct {
    #[inline(always)]
    pub fn r#with_fields(mut self, k: String, v: Value) -> Self {
        self.r#add_fields(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_fields(&mut self, k: String, v: Value) -> &mut Self {
        let _ = self.fields.insert(k, v);
        self
    }
}
impl textformat::Decodable for Struct {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("fields") => {
                textformat::Field::merge(&mut self.fields, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Struct {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.fields
            != <::std::collections::HashMap<String, Value> as Default>::default()
        {
            out.indent(pad);
            out.push_str("fields ");
            textformat::Field::format(&self.fields, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Struct {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Map::<Bytes, Nest>>::decode(&mut self.fields, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Struct {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Struct"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.fields.should_encode(true) {
            Format::<Map::<Bytes, Nest>>::encode(&self.fields, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Value {
    pub kind: ValueOneOfKind,
    pub _unknown: (),
}
impl Value {
    #[inline(always)]
    pub fn r#with_kind_null_value(mut self, it: NullValue) -> Self {
        self.kind = ValueOneOfKind::NullValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_null_value(&mut self, it: NullValue) -> &mut Self {
        self.kind = ValueOneOfKind::NullValue(it);
        self
    }
    #[inline(always)]
    pub fn r#with_kind_number_value(mut self, it: f64) -> Self {
        self.kind = ValueOneOfKind::NumberValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_number_value(&mut self, it: f64) -> &mut Self {
        self.kind = ValueOneOfKind::NumberValue(it);
        self
    }
    #[inline(always)]
    pub fn r#with_kind_string_value(mut self, it: String) -> Self {
        self.kind = ValueOneOfKind::StringValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_string_value(&mut self, it: String) -> &mut Self {
        self.kind = ValueOneOfKind::StringValue(it);
        self
    }
    #[inline(always)]
    pub fn r#with_kind_bool_value(mut self, it: bool) -> Self {
        self.kind = ValueOneOfKind::BoolValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_bool_value(&mut self, it: bool) -> &mut Self {
        self.kind = ValueOneOfKind::BoolValue(it);
        self
    }
    #[inline(always)]
    pub fn r#with_kind_struct_value(mut self, it: Struct) -> Self {
        self.kind = ValueOneOfKind::StructValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_struct_value(&mut self, it: Struct) -> &mut Self {
        self.kind = ValueOneOfKind::StructValue(it);
        self
    }
    #[inline(always)]
    pub fn r#with_kind_list_value(mut self, it: ListValue) -> Self {
        self.kind = ValueOneOfKind::ListValue(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind_list_value(&mut self, it: ListValue) -> &mut Self {
        self.kind = ValueOneOfKind::ListValue(it);
        self
    }
}
impl textformat::Decodable for Value {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("null_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::NullValue(target);
            }
            textformat::ast::FieldName::Normal("number_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::NumberValue(target);
            }
            textformat::ast::FieldName::Normal("string_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::StringValue(target);
            }
            textformat::ast::FieldName::Normal("bool_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::BoolValue(target);
            }
            textformat::ast::FieldName::Normal("struct_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::StructValue(target);
            }
            textformat::ast::FieldName::Normal("list_value") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.kind = ValueOneOfKind::ListValue(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Value {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        match &self.kind {
            ValueOneOfKind::NullValue(value) => {
                out.indent(pad);
                out.push_str("null_value: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::NumberValue(value) => {
                out.indent(pad);
                out.push_str("number_value: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::StringValue(value) => {
                out.indent(pad);
                out.push_str("string_value: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::BoolValue(value) => {
                out.indent(pad);
                out.push_str("bool_value: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::StructValue(value) => {
                out.indent(pad);
                out.push_str("struct_value ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::ListValue(value) => {
                out.indent(pad);
                out.push_str("list_value ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ValueOneOfKind::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for Value {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::NullValue(tmp);
            }
            10u32 => {
                let mut tmp = Default::default();
                buf = Format::<Enum>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::NullValue(tmp);
            }
            17u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::NumberValue(tmp);
            }
            18u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::NumberValue(tmp);
            }
            26u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::StringValue(tmp);
            }
            32u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::BoolValue(tmp);
            }
            34u32 => {
                let mut tmp = Default::default();
                buf = Format::<Fix>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::BoolValue(tmp);
            }
            42u32 => {
                let mut tmp = Default::default();
                buf = Format::<Nest>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::StructValue(tmp);
            }
            50u32 => {
                let mut tmp = Default::default();
                buf = Format::<Nest>::decode(&mut tmp, buf)?;
                self.kind = ValueOneOfKind::ListValue(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Value {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Value"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        match &self.kind {
            ValueOneOfKind::NullValue(value) => {
                Format::<Enum>::encode(value, 1u32, buf)?;
            }
            ValueOneOfKind::NumberValue(value) => {
                Format::<Fix>::encode(value, 2u32, buf)?;
            }
            ValueOneOfKind::StringValue(value) => {
                Format::<Bytes>::encode(value, 3u32, buf)?;
            }
            ValueOneOfKind::BoolValue(value) => {
                Format::<Fix>::encode(value, 4u32, buf)?;
            }
            ValueOneOfKind::StructValue(value) => {
                Format::<Nest>::encode(value, 5u32, buf)?;
            }
            ValueOneOfKind::ListValue(value) => {
                Format::<Nest>::encode(value, 6u32, buf)?;
            }
            ValueOneOfKind::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueOneOfKind {
    NullValue(NullValue),
    NumberValue(f64),
    StringValue(String),
    BoolValue(bool),
    StructValue(Struct),
    ListValue(ListValue),
    Unknown(::core::marker::PhantomData<()>),
}
impl binformat::ShouldEncode for ValueOneOfKind {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl Default for ValueOneOfKind {
    fn default() -> Self {
        ValueOneOfKind::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListValue {
    pub values: Vec<Value>,
    pub _unknown: (),
}
impl ListValue {
    #[inline(always)]
    pub fn r#with_values(mut self, it: Value) -> Self {
        self.r#add_values(it);
        self
    }
    #[inline(always)]
    pub fn r#add_values(&mut self, it: Value) -> &mut Self {
        self.values.push(it);
        self
    }
}
impl textformat::Decodable for ListValue {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("values") => {
                textformat::Field::merge(&mut self.values, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ListValue {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.values != <Vec<Value> as Default>::default() {
            out.indent(pad);
            out.push_str("values ");
            textformat::Field::format(&self.values, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ListValue {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.values, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ListValue {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.ListValue"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.values.should_encode(true) {
            Format::<Repeat::<Nest>>::encode(&self.values, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum NullValue {
    NULL_VALUE,
    Unknown(u32),
}
impl Default for NullValue {
    fn default() -> NullValue {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for NullValue {}
impl binformat::ShouldEncode for NullValue {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for NullValue {
    fn from(v: u32) -> NullValue {
        match v {
            0u32 => NullValue::NULL_VALUE,
            other => NullValue::Unknown(other),
        }
    }
}
impl From<NullValue> for u32 {
    fn from(v: NullValue) -> u32 {
        match v {
            NullValue::NULL_VALUE => 0u32,
            NullValue::Unknown(other) => other,
        }
    }
}
impl textformat::Field for NullValue {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            NullValue::NULL_VALUE => "NULL_VALUE",
            NullValue::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("NULL_VALUE") => {
                *self = NullValue::NULL_VALUE;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
