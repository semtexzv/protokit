#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::descriptor::*;
use super::super::google::protobuf::duration::*;
use root::types::timestamp::*;
use root::types::timestamp::*;
use super::super::google::protobuf::duration::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&FieldRules::default());
    registry.register(&FloatRules::default());
    registry.register(&DoubleRules::default());
    registry.register(&Int32Rules::default());
    registry.register(&Int64Rules::default());
    registry.register(&UInt32Rules::default());
    registry.register(&UInt64Rules::default());
    registry.register(&SInt32Rules::default());
    registry.register(&SInt64Rules::default());
    registry.register(&Fixed32Rules::default());
    registry.register(&Fixed64Rules::default());
    registry.register(&SFixed32Rules::default());
    registry.register(&SFixed64Rules::default());
    registry.register(&BoolRules::default());
    registry.register(&StringRules::default());
    registry.register(&BytesRules::default());
    registry.register(&EnumRules::default());
    registry.register(&MessageRules::default());
    registry.register(&RepeatedRules::default());
    registry.register(&MapRules::default());
    registry.register(&AnyRules::default());
    registry.register(&DurationRules::default());
    registry.register(&TimestampRules::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldRules {
    pub message: Option<Box<MessageRules>>,
    pub _unknown: (),
}
impl FieldRules {
    #[inline(always)]
    pub fn r#with_message(mut self, it: MessageRules) -> Self {
        self.r#set_message(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message(&mut self, it: MessageRules) -> &mut Self {
        self.message = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for FieldRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("message") => {
                textformat::Field::merge(&mut self.message, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FieldRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.message != <Option<Box<MessageRules>> as Default>::default() {
            out.indent(pad);
            out.push_str("message ");
            textformat::Field::format(&self.message, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            138u32 => {
                buf = Format::<Nest>::decode(&mut self.message, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldRules {
    fn qualified_name(&self) -> &'static str {
        "validate.FieldRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.message.should_encode(false) {
            Format::<Nest>::encode(&self.message, 17u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatRules {
    pub r#const: Option<f32>,
    pub lt: Option<f32>,
    pub lte: Option<f32>,
    pub gt: Option<f32>,
    pub gte: Option<f32>,
    pub r#in: Vec<f32>,
    pub not_in: Vec<f32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl FloatRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: f32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: f32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: f32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: f32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: f32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: f32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: f32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: f32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: f32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: f32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: f32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: f32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: f32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: f32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for FloatRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FloatRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FloatRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            13u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            21u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            29u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            37u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            45u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            53u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            61u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FloatRules {
    fn qualified_name(&self) -> &'static str {
        "validate.FloatRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DoubleRules {
    pub r#const: Option<f64>,
    pub lt: Option<f64>,
    pub lte: Option<f64>,
    pub gt: Option<f64>,
    pub gte: Option<f64>,
    pub r#in: Vec<f64>,
    pub not_in: Vec<f64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl DoubleRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: f64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: f64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: f64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: f64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: f64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: f64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: f64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: f64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: f64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: f64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: f64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: f64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: f64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: f64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for DoubleRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DoubleRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DoubleRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            9u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            17u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            25u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            33u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            41u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            49u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            57u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DoubleRules {
    fn qualified_name(&self) -> &'static str {
        "validate.DoubleRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Int32Rules {
    pub r#const: Option<i32>,
    pub lt: Option<i32>,
    pub lte: Option<i32>,
    pub gt: Option<i32>,
    pub gte: Option<i32>,
    pub r#in: Vec<i32>,
    pub not_in: Vec<i32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl Int32Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for Int32Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Int32Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Int32Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Int32Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.Int32Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<VInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<VInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<VInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<VInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<VInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Int64Rules {
    pub r#const: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub r#in: Vec<i64>,
    pub not_in: Vec<i64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl Int64Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for Int64Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Int64Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Int64Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Int64Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.Int64Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<VInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<VInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<VInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<VInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<VInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UInt32Rules {
    pub r#const: Option<u32>,
    pub lt: Option<u32>,
    pub lte: Option<u32>,
    pub gt: Option<u32>,
    pub gte: Option<u32>,
    pub r#in: Vec<u32>,
    pub not_in: Vec<u32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl UInt32Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: u32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: u32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: u32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: u32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: u32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: u32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: u32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: u32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: u32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: u32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: u32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: u32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: u32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: u32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for UInt32Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UInt32Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UInt32Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UInt32Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.UInt32Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<VInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<VInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<VInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<VInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<VInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UInt64Rules {
    pub r#const: Option<u64>,
    pub lt: Option<u64>,
    pub lte: Option<u64>,
    pub gt: Option<u64>,
    pub gte: Option<u64>,
    pub r#in: Vec<u64>,
    pub not_in: Vec<u64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl UInt64Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: u64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: u64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: u64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: u64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: u64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: u64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: u64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: u64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: u64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: u64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: u64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: u64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: u64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: u64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for UInt64Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UInt64Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UInt64Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UInt64Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.UInt64Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<VInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<VInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<VInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<VInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<VInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SInt32Rules {
    pub r#const: Option<i32>,
    pub lt: Option<i32>,
    pub lte: Option<i32>,
    pub gt: Option<i32>,
    pub gte: Option<i32>,
    pub r#in: Vec<i32>,
    pub not_in: Vec<i32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl SInt32Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for SInt32Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SInt32Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SInt32Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<SInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<SInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<SInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<SInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<SInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<SInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<SInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<SInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<SInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<SInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SInt32Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.SInt32Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<SInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<SInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<SInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<SInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<SInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SInt64Rules {
    pub r#const: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub r#in: Vec<i64>,
    pub not_in: Vec<i64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl SInt64Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for SInt64Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SInt64Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SInt64Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<SInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<SInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<SInt>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<SInt>::decode(&mut self.lt, buf)?;
            }
            24u32 => {
                buf = Format::<SInt>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<SInt>::decode(&mut self.lte, buf)?;
            }
            32u32 => {
                buf = Format::<SInt>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<SInt>::decode(&mut self.gt, buf)?;
            }
            40u32 => {
                buf = Format::<SInt>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<SInt>::decode(&mut self.gte, buf)?;
            }
            48u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.r#in, buf)?;
            }
            56u32 => {
                buf = Format::<Repeat::<SInt>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<SInt>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SInt64Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.SInt64Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<SInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<SInt>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<SInt>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<SInt>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<SInt>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<SInt>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Fixed32Rules {
    pub r#const: Option<u32>,
    pub lt: Option<u32>,
    pub lte: Option<u32>,
    pub gt: Option<u32>,
    pub gte: Option<u32>,
    pub r#in: Vec<u32>,
    pub not_in: Vec<u32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl Fixed32Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: u32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: u32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: u32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: u32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: u32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: u32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: u32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: u32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: u32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: u32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: u32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: u32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: u32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: u32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for Fixed32Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Fixed32Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<u32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Fixed32Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            13u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            21u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            29u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            37u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            45u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            53u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            61u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Fixed32Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.Fixed32Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Fixed64Rules {
    pub r#const: Option<u64>,
    pub lt: Option<u64>,
    pub lte: Option<u64>,
    pub gt: Option<u64>,
    pub gte: Option<u64>,
    pub r#in: Vec<u64>,
    pub not_in: Vec<u64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl Fixed64Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: u64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: u64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: u64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: u64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: u64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: u64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: u64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: u64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: u64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: u64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: u64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: u64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: u64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: u64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for Fixed64Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Fixed64Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Fixed64Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            9u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            17u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            25u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            33u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            41u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            49u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            57u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Fixed64Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.Fixed64Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SFixed32Rules {
    pub r#const: Option<i32>,
    pub lt: Option<i32>,
    pub lte: Option<i32>,
    pub gt: Option<i32>,
    pub gte: Option<i32>,
    pub r#in: Vec<i32>,
    pub not_in: Vec<i32>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl SFixed32Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i32) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i32) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i32) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i32) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i32) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i32) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i32) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i32) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i32) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for SFixed32Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SFixed32Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SFixed32Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            13u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            21u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            29u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            37u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            45u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            53u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            61u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SFixed32Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.SFixed32Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SFixed64Rules {
    pub r#const: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub r#in: Vec<i64>,
    pub not_in: Vec<i64>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl SFixed64Rules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i64) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i64) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: i64) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: i64) -> &mut Self {
        self.lt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: i64) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: i64) -> &mut Self {
        self.lte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: i64) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: i64) -> &mut Self {
        self.gt = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: i64) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: i64) -> &mut Self {
        self.gte = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i64) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i64) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i64) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i64) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for SFixed64Rules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SFixed64Rules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lt: ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("lte: ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gt: ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("gte: ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SFixed64Rules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            9u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            17u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.lt, buf)?;
            }
            25u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.lte, buf)?;
            }
            33u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            34u32 => {
                buf = Format::<Fix>::decode(&mut self.gt, buf)?;
            }
            41u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.gte, buf)?;
            }
            49u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            50u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.r#in, buf)?;
            }
            57u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            58u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.not_in, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SFixed64Rules {
    fn qualified_name(&self) -> &'static str {
        "validate.SFixed64Rules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Fix>::encode(&self.lt, 2u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Fix>::encode(&self.lte, 3u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Fix>::encode(&self.gt, 4u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Fix>::encode(&self.gte, 5u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.r#in, 6u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Fix>>::encode(&self.not_in, 7u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoolRules {
    pub r#const: Option<bool>,
    pub _unknown: (),
}
impl BoolRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: bool) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: bool) -> &mut Self {
        self.r#const = it.into();
        self
    }
}
impl textformat::Decodable for BoolRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for BoolRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for BoolRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.r#const, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for BoolRules {
    fn qualified_name(&self) -> &'static str {
        "validate.BoolRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Fix>::encode(&self.r#const, 1u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringRules {
    pub r#const: Option<String>,
    pub len: Option<u64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub len_bytes: Option<u64>,
    pub min_bytes: Option<u64>,
    pub max_bytes: Option<u64>,
    pub pattern: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub contains: Option<String>,
    pub not_contains: Option<String>,
    pub r#in: Vec<String>,
    pub not_in: Vec<String>,
    pub strict: Option<bool>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl StringRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: String) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: String) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_len(mut self, it: u64) -> Self {
        self.r#set_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_len(&mut self, it: u64) -> &mut Self {
        self.len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_min_len(mut self, it: u64) -> Self {
        self.r#set_min_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_min_len(&mut self, it: u64) -> &mut Self {
        self.min_len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_max_len(mut self, it: u64) -> Self {
        self.r#set_max_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_max_len(&mut self, it: u64) -> &mut Self {
        self.max_len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_len_bytes(mut self, it: u64) -> Self {
        self.r#set_len_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_len_bytes(&mut self, it: u64) -> &mut Self {
        self.len_bytes = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_min_bytes(mut self, it: u64) -> Self {
        self.r#set_min_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_min_bytes(&mut self, it: u64) -> &mut Self {
        self.min_bytes = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_max_bytes(mut self, it: u64) -> Self {
        self.r#set_max_bytes(it);
        self
    }
    #[inline(always)]
    pub fn r#set_max_bytes(&mut self, it: u64) -> &mut Self {
        self.max_bytes = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_pattern(mut self, it: String) -> Self {
        self.r#set_pattern(it);
        self
    }
    #[inline(always)]
    pub fn r#set_pattern(&mut self, it: String) -> &mut Self {
        self.pattern = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_prefix(mut self, it: String) -> Self {
        self.r#set_prefix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_prefix(&mut self, it: String) -> &mut Self {
        self.prefix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_suffix(mut self, it: String) -> Self {
        self.r#set_suffix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_suffix(&mut self, it: String) -> &mut Self {
        self.suffix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_contains(mut self, it: String) -> Self {
        self.r#set_contains(it);
        self
    }
    #[inline(always)]
    pub fn r#set_contains(&mut self, it: String) -> &mut Self {
        self.contains = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_not_contains(mut self, it: String) -> Self {
        self.r#set_not_contains(it);
        self
    }
    #[inline(always)]
    pub fn r#set_not_contains(&mut self, it: String) -> &mut Self {
        self.not_contains = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: String) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: String) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: String) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: String) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_strict(mut self, it: bool) -> Self {
        self.r#set_strict(it);
        self
    }
    #[inline(always)]
    pub fn r#set_strict(&mut self, it: bool) -> &mut Self {
        self.strict = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for StringRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("len") => {
                textformat::Field::merge(&mut self.len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("min_len") => {
                textformat::Field::merge(&mut self.min_len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("max_len") => {
                textformat::Field::merge(&mut self.max_len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("len_bytes") => {
                textformat::Field::merge(&mut self.len_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("min_bytes") => {
                textformat::Field::merge(&mut self.min_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("max_bytes") => {
                textformat::Field::merge(&mut self.max_bytes, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pattern") => {
                textformat::Field::merge(&mut self.pattern, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("prefix") => {
                textformat::Field::merge(&mut self.prefix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("suffix") => {
                textformat::Field::merge(&mut self.suffix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("contains") => {
                textformat::Field::merge(&mut self.contains, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_contains") => {
                textformat::Field::merge(&mut self.not_contains, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("strict") => {
                textformat::Field::merge(&mut self.strict, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for StringRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("len: ");
            textformat::Field::format(&self.len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.min_len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("min_len: ");
            textformat::Field::format(&self.min_len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.max_len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("max_len: ");
            textformat::Field::format(&self.max_len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.len_bytes != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("len_bytes: ");
            textformat::Field::format(&self.len_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.min_bytes != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("min_bytes: ");
            textformat::Field::format(&self.min_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.max_bytes != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("max_bytes: ");
            textformat::Field::format(&self.max_bytes, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pattern != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("pattern: ");
            textformat::Field::format(&self.pattern, ctx, pad, out)?;
            out.push('\n');
        }
        if self.prefix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("prefix: ");
            textformat::Field::format(&self.prefix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.suffix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("suffix: ");
            textformat::Field::format(&self.suffix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.contains != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("contains: ");
            textformat::Field::format(&self.contains, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_contains != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("not_contains: ");
            textformat::Field::format(&self.not_contains, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.strict != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("strict: ");
            textformat::Field::format(&self.strict, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for StringRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.r#const, buf)?;
            }
            152u32 => {
                buf = Format::<VInt>::decode(&mut self.len, buf)?;
            }
            154u32 => {
                buf = Format::<VInt>::decode(&mut self.len, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.min_len, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.min_len, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.max_len, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.max_len, buf)?;
            }
            160u32 => {
                buf = Format::<VInt>::decode(&mut self.len_bytes, buf)?;
            }
            162u32 => {
                buf = Format::<VInt>::decode(&mut self.len_bytes, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.min_bytes, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.min_bytes, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.max_bytes, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.max_bytes, buf)?;
            }
            50u32 => {
                buf = Format::<Bytes>::decode(&mut self.pattern, buf)?;
            }
            58u32 => {
                buf = Format::<Bytes>::decode(&mut self.prefix, buf)?;
            }
            66u32 => {
                buf = Format::<Bytes>::decode(&mut self.suffix, buf)?;
            }
            74u32 => {
                buf = Format::<Bytes>::decode(&mut self.contains, buf)?;
            }
            186u32 => {
                buf = Format::<Bytes>::decode(&mut self.not_contains, buf)?;
            }
            82u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.r#in, buf)?;
            }
            90u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.not_in, buf)?;
            }
            200u32 => {
                buf = Format::<Fix>::decode(&mut self.strict, buf)?;
            }
            202u32 => {
                buf = Format::<Fix>::decode(&mut self.strict, buf)?;
            }
            208u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            210u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for StringRules {
    fn qualified_name(&self) -> &'static str {
        "validate.StringRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Bytes>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.len.should_encode(false) {
            Format::<VInt>::encode(&self.len, 19u32, buf)?;
        }
        if self.min_len.should_encode(false) {
            Format::<VInt>::encode(&self.min_len, 2u32, buf)?;
        }
        if self.max_len.should_encode(false) {
            Format::<VInt>::encode(&self.max_len, 3u32, buf)?;
        }
        if self.len_bytes.should_encode(false) {
            Format::<VInt>::encode(&self.len_bytes, 20u32, buf)?;
        }
        if self.min_bytes.should_encode(false) {
            Format::<VInt>::encode(&self.min_bytes, 4u32, buf)?;
        }
        if self.max_bytes.should_encode(false) {
            Format::<VInt>::encode(&self.max_bytes, 5u32, buf)?;
        }
        if self.pattern.should_encode(false) {
            Format::<Bytes>::encode(&self.pattern, 6u32, buf)?;
        }
        if self.prefix.should_encode(false) {
            Format::<Bytes>::encode(&self.prefix, 7u32, buf)?;
        }
        if self.suffix.should_encode(false) {
            Format::<Bytes>::encode(&self.suffix, 8u32, buf)?;
        }
        if self.contains.should_encode(false) {
            Format::<Bytes>::encode(&self.contains, 9u32, buf)?;
        }
        if self.not_contains.should_encode(false) {
            Format::<Bytes>::encode(&self.not_contains, 23u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.r#in, 10u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.not_in, 11u32, buf)?;
        }
        if self.strict.should_encode(false) {
            Format::<Fix>::encode(&self.strict, 25u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 26u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BytesRules {
    pub r#const: Option<Vec<u8>>,
    pub len: Option<u64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub pattern: Option<String>,
    pub prefix: Option<Vec<u8>>,
    pub suffix: Option<Vec<u8>>,
    pub contains: Option<Vec<u8>>,
    pub r#in: Vec<Vec<u8>>,
    pub not_in: Vec<Vec<u8>>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl BytesRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: Vec<u8>) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: Vec<u8>) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_len(mut self, it: u64) -> Self {
        self.r#set_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_len(&mut self, it: u64) -> &mut Self {
        self.len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_min_len(mut self, it: u64) -> Self {
        self.r#set_min_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_min_len(&mut self, it: u64) -> &mut Self {
        self.min_len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_max_len(mut self, it: u64) -> Self {
        self.r#set_max_len(it);
        self
    }
    #[inline(always)]
    pub fn r#set_max_len(&mut self, it: u64) -> &mut Self {
        self.max_len = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_pattern(mut self, it: String) -> Self {
        self.r#set_pattern(it);
        self
    }
    #[inline(always)]
    pub fn r#set_pattern(&mut self, it: String) -> &mut Self {
        self.pattern = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_prefix(mut self, it: Vec<u8>) -> Self {
        self.r#set_prefix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_prefix(&mut self, it: Vec<u8>) -> &mut Self {
        self.prefix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_suffix(mut self, it: Vec<u8>) -> Self {
        self.r#set_suffix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_suffix(&mut self, it: Vec<u8>) -> &mut Self {
        self.suffix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_contains(mut self, it: Vec<u8>) -> Self {
        self.r#set_contains(it);
        self
    }
    #[inline(always)]
    pub fn r#set_contains(&mut self, it: Vec<u8>) -> &mut Self {
        self.contains = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: Vec<u8>) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: Vec<u8>) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: Vec<u8>) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: Vec<u8>) -> &mut Self {
        self.not_in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for BytesRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("len") => {
                textformat::Field::merge(&mut self.len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("min_len") => {
                textformat::Field::merge(&mut self.min_len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("max_len") => {
                textformat::Field::merge(&mut self.max_len, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pattern") => {
                textformat::Field::merge(&mut self.pattern, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("prefix") => {
                textformat::Field::merge(&mut self.prefix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("suffix") => {
                textformat::Field::merge(&mut self.suffix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("contains") => {
                textformat::Field::merge(&mut self.contains, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for BytesRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("len: ");
            textformat::Field::format(&self.len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.min_len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("min_len: ");
            textformat::Field::format(&self.min_len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.max_len != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("max_len: ");
            textformat::Field::format(&self.max_len, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pattern != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("pattern: ");
            textformat::Field::format(&self.pattern, ctx, pad, out)?;
            out.push('\n');
        }
        if self.prefix != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("prefix: ");
            textformat::Field::format(&self.prefix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.suffix != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("suffix: ");
            textformat::Field::format(&self.suffix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.contains != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("contains: ");
            textformat::Field::format(&self.contains, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for BytesRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.r#const, buf)?;
            }
            104u32 => {
                buf = Format::<VInt>::decode(&mut self.len, buf)?;
            }
            106u32 => {
                buf = Format::<VInt>::decode(&mut self.len, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.min_len, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.min_len, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.max_len, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.max_len, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.pattern, buf)?;
            }
            42u32 => {
                buf = Format::<Bytes>::decode(&mut self.prefix, buf)?;
            }
            50u32 => {
                buf = Format::<Bytes>::decode(&mut self.suffix, buf)?;
            }
            58u32 => {
                buf = Format::<Bytes>::decode(&mut self.contains, buf)?;
            }
            66u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.r#in, buf)?;
            }
            74u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.not_in, buf)?;
            }
            112u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            114u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for BytesRules {
    fn qualified_name(&self) -> &'static str {
        "validate.BytesRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<Bytes>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.len.should_encode(false) {
            Format::<VInt>::encode(&self.len, 13u32, buf)?;
        }
        if self.min_len.should_encode(false) {
            Format::<VInt>::encode(&self.min_len, 2u32, buf)?;
        }
        if self.max_len.should_encode(false) {
            Format::<VInt>::encode(&self.max_len, 3u32, buf)?;
        }
        if self.pattern.should_encode(false) {
            Format::<Bytes>::encode(&self.pattern, 4u32, buf)?;
        }
        if self.prefix.should_encode(false) {
            Format::<Bytes>::encode(&self.prefix, 5u32, buf)?;
        }
        if self.suffix.should_encode(false) {
            Format::<Bytes>::encode(&self.suffix, 6u32, buf)?;
        }
        if self.contains.should_encode(false) {
            Format::<Bytes>::encode(&self.contains, 7u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.r#in, 8u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.not_in, 9u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 14u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumRules {
    pub r#const: Option<i32>,
    pub defined_only: Option<bool>,
    pub r#in: Vec<i32>,
    pub not_in: Vec<i32>,
    pub _unknown: (),
}
impl EnumRules {
    #[inline(always)]
    pub fn r#with_const(mut self, it: i32) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: i32) -> &mut Self {
        self.r#const = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_defined_only(mut self, it: bool) -> Self {
        self.r#set_defined_only(it);
        self
    }
    #[inline(always)]
    pub fn r#set_defined_only(&mut self, it: bool) -> &mut Self {
        self.defined_only = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: i32) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: i32) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: i32) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: i32) -> &mut Self {
        self.not_in.push(it);
        self
    }
}
impl textformat::Decodable for EnumRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("defined_only") => {
                textformat::Field::merge(&mut self.defined_only, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#const != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("const: ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.defined_only != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("defined_only: ");
            textformat::Field::format(&self.defined_only, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.r#const, buf)?;
            }
            16u32 => {
                buf = Format::<Fix>::decode(&mut self.defined_only, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.defined_only, buf)?;
            }
            24u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            26u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.r#in, buf)?;
            }
            32u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            34u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.not_in, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumRules {
    fn qualified_name(&self) -> &'static str {
        "validate.EnumRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.r#const.should_encode(false) {
            Format::<VInt>::encode(&self.r#const, 1u32, buf)?;
        }
        if self.defined_only.should_encode(false) {
            Format::<Fix>::encode(&self.defined_only, 2u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.r#in, 3u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<VInt>>::encode(&self.not_in, 4u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageRules {
    pub skip: Option<bool>,
    pub required: Option<bool>,
    pub _unknown: (),
}
impl MessageRules {
    #[inline(always)]
    pub fn r#with_skip(mut self, it: bool) -> Self {
        self.r#set_skip(it);
        self
    }
    #[inline(always)]
    pub fn r#set_skip(&mut self, it: bool) -> &mut Self {
        self.skip = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_required(mut self, it: bool) -> Self {
        self.r#set_required(it);
        self
    }
    #[inline(always)]
    pub fn r#set_required(&mut self, it: bool) -> &mut Self {
        self.required = it.into();
        self
    }
}
impl textformat::Decodable for MessageRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("skip") => {
                textformat::Field::merge(&mut self.skip, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("required") => {
                textformat::Field::merge(&mut self.required, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for MessageRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.skip != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("skip: ");
            textformat::Field::format(&self.skip, ctx, pad, out)?;
            out.push('\n');
        }
        if self.required != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("required: ");
            textformat::Field::format(&self.required, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for MessageRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.skip, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.skip, buf)?;
            }
            16u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for MessageRules {
    fn qualified_name(&self) -> &'static str {
        "validate.MessageRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.skip.should_encode(false) {
            Format::<Fix>::encode(&self.skip, 1u32, buf)?;
        }
        if self.required.should_encode(false) {
            Format::<Fix>::encode(&self.required, 2u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RepeatedRules {
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
    pub unique: Option<bool>,
    pub items: Option<Box<FieldRules>>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl RepeatedRules {
    #[inline(always)]
    pub fn r#with_min_items(mut self, it: u64) -> Self {
        self.r#set_min_items(it);
        self
    }
    #[inline(always)]
    pub fn r#set_min_items(&mut self, it: u64) -> &mut Self {
        self.min_items = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_max_items(mut self, it: u64) -> Self {
        self.r#set_max_items(it);
        self
    }
    #[inline(always)]
    pub fn r#set_max_items(&mut self, it: u64) -> &mut Self {
        self.max_items = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_unique(mut self, it: bool) -> Self {
        self.r#set_unique(it);
        self
    }
    #[inline(always)]
    pub fn r#set_unique(&mut self, it: bool) -> &mut Self {
        self.unique = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_items(mut self, it: FieldRules) -> Self {
        self.r#set_items(it);
        self
    }
    #[inline(always)]
    pub fn r#set_items(&mut self, it: FieldRules) -> &mut Self {
        self.items = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for RepeatedRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("min_items") => {
                textformat::Field::merge(&mut self.min_items, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("max_items") => {
                textformat::Field::merge(&mut self.max_items, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("unique") => {
                textformat::Field::merge(&mut self.unique, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("items") => {
                textformat::Field::merge(&mut self.items, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for RepeatedRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.min_items != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("min_items: ");
            textformat::Field::format(&self.min_items, ctx, pad, out)?;
            out.push('\n');
        }
        if self.max_items != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("max_items: ");
            textformat::Field::format(&self.max_items, ctx, pad, out)?;
            out.push('\n');
        }
        if self.unique != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("unique: ");
            textformat::Field::format(&self.unique, ctx, pad, out)?;
            out.push('\n');
        }
        if self.items != <Option<Box<FieldRules>> as Default>::default() {
            out.indent(pad);
            out.push_str("items ");
            textformat::Field::format(&self.items, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for RepeatedRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.min_items, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.min_items, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.max_items, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.max_items, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.unique, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.unique, buf)?;
            }
            34u32 => {
                buf = Format::<Nest>::decode(&mut self.items, buf)?;
            }
            40u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for RepeatedRules {
    fn qualified_name(&self) -> &'static str {
        "validate.RepeatedRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.min_items.should_encode(false) {
            Format::<VInt>::encode(&self.min_items, 1u32, buf)?;
        }
        if self.max_items.should_encode(false) {
            Format::<VInt>::encode(&self.max_items, 2u32, buf)?;
        }
        if self.unique.should_encode(false) {
            Format::<Fix>::encode(&self.unique, 3u32, buf)?;
        }
        if self.items.should_encode(false) {
            Format::<Nest>::encode(&self.items, 4u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 5u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapRules {
    pub min_pairs: Option<u64>,
    pub max_pairs: Option<u64>,
    pub no_sparse: Option<bool>,
    pub keys: Option<Box<FieldRules>>,
    pub values: Option<Box<FieldRules>>,
    pub ignore_empty: Option<bool>,
    pub _unknown: (),
}
impl MapRules {
    #[inline(always)]
    pub fn r#with_min_pairs(mut self, it: u64) -> Self {
        self.r#set_min_pairs(it);
        self
    }
    #[inline(always)]
    pub fn r#set_min_pairs(&mut self, it: u64) -> &mut Self {
        self.min_pairs = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_max_pairs(mut self, it: u64) -> Self {
        self.r#set_max_pairs(it);
        self
    }
    #[inline(always)]
    pub fn r#set_max_pairs(&mut self, it: u64) -> &mut Self {
        self.max_pairs = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_no_sparse(mut self, it: bool) -> Self {
        self.r#set_no_sparse(it);
        self
    }
    #[inline(always)]
    pub fn r#set_no_sparse(&mut self, it: bool) -> &mut Self {
        self.no_sparse = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_keys(mut self, it: FieldRules) -> Self {
        self.r#set_keys(it);
        self
    }
    #[inline(always)]
    pub fn r#set_keys(&mut self, it: FieldRules) -> &mut Self {
        self.keys = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_values(mut self, it: FieldRules) -> Self {
        self.r#set_values(it);
        self
    }
    #[inline(always)]
    pub fn r#set_values(&mut self, it: FieldRules) -> &mut Self {
        self.values = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_ignore_empty(mut self, it: bool) -> Self {
        self.r#set_ignore_empty(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignore_empty(&mut self, it: bool) -> &mut Self {
        self.ignore_empty = it.into();
        self
    }
}
impl textformat::Decodable for MapRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("min_pairs") => {
                textformat::Field::merge(&mut self.min_pairs, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("max_pairs") => {
                textformat::Field::merge(&mut self.max_pairs, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("no_sparse") => {
                textformat::Field::merge(&mut self.no_sparse, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("keys") => {
                textformat::Field::merge(&mut self.keys, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("values") => {
                textformat::Field::merge(&mut self.values, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ignore_empty") => {
                textformat::Field::merge(&mut self.ignore_empty, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for MapRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.min_pairs != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("min_pairs: ");
            textformat::Field::format(&self.min_pairs, ctx, pad, out)?;
            out.push('\n');
        }
        if self.max_pairs != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("max_pairs: ");
            textformat::Field::format(&self.max_pairs, ctx, pad, out)?;
            out.push('\n');
        }
        if self.no_sparse != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("no_sparse: ");
            textformat::Field::format(&self.no_sparse, ctx, pad, out)?;
            out.push('\n');
        }
        if self.keys != <Option<Box<FieldRules>> as Default>::default() {
            out.indent(pad);
            out.push_str("keys ");
            textformat::Field::format(&self.keys, ctx, pad, out)?;
            out.push('\n');
        }
        if self.values != <Option<Box<FieldRules>> as Default>::default() {
            out.indent(pad);
            out.push_str("values ");
            textformat::Field::format(&self.values, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignore_empty != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("ignore_empty: ");
            textformat::Field::format(&self.ignore_empty, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for MapRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.min_pairs, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.min_pairs, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.max_pairs, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.max_pairs, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.no_sparse, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.no_sparse, buf)?;
            }
            34u32 => {
                buf = Format::<Nest>::decode(&mut self.keys, buf)?;
            }
            42u32 => {
                buf = Format::<Nest>::decode(&mut self.values, buf)?;
            }
            48u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            50u32 => {
                buf = Format::<Fix>::decode(&mut self.ignore_empty, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for MapRules {
    fn qualified_name(&self) -> &'static str {
        "validate.MapRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.min_pairs.should_encode(false) {
            Format::<VInt>::encode(&self.min_pairs, 1u32, buf)?;
        }
        if self.max_pairs.should_encode(false) {
            Format::<VInt>::encode(&self.max_pairs, 2u32, buf)?;
        }
        if self.no_sparse.should_encode(false) {
            Format::<Fix>::encode(&self.no_sparse, 3u32, buf)?;
        }
        if self.keys.should_encode(false) {
            Format::<Nest>::encode(&self.keys, 4u32, buf)?;
        }
        if self.values.should_encode(false) {
            Format::<Nest>::encode(&self.values, 5u32, buf)?;
        }
        if self.ignore_empty.should_encode(false) {
            Format::<Fix>::encode(&self.ignore_empty, 6u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AnyRules {
    pub required: Option<bool>,
    pub r#in: Vec<String>,
    pub not_in: Vec<String>,
    pub _unknown: (),
}
impl AnyRules {
    #[inline(always)]
    pub fn r#with_required(mut self, it: bool) -> Self {
        self.r#set_required(it);
        self
    }
    #[inline(always)]
    pub fn r#set_required(&mut self, it: bool) -> &mut Self {
        self.required = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: String) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: String) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: String) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: String) -> &mut Self {
        self.not_in.push(it);
        self
    }
}
impl textformat::Decodable for AnyRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("required") => {
                textformat::Field::merge(&mut self.required, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for AnyRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.required != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("required: ");
            textformat::Field::format(&self.required, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("in: ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in: ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for AnyRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            18u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.r#in, buf)?;
            }
            26u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.not_in, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for AnyRules {
    fn qualified_name(&self) -> &'static str {
        "validate.AnyRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.required.should_encode(false) {
            Format::<Fix>::encode(&self.required, 1u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.r#in, 2u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.not_in, 3u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DurationRules {
    pub required: Option<bool>,
    pub r#const: Option<Box<Duration>>,
    pub lt: Option<Box<Duration>>,
    pub lte: Option<Box<Duration>>,
    pub gt: Option<Box<Duration>>,
    pub gte: Option<Box<Duration>>,
    pub r#in: Vec<Duration>,
    pub not_in: Vec<Duration>,
    pub _unknown: (),
}
impl DurationRules {
    #[inline(always)]
    pub fn r#with_required(mut self, it: bool) -> Self {
        self.r#set_required(it);
        self
    }
    #[inline(always)]
    pub fn r#set_required(&mut self, it: bool) -> &mut Self {
        self.required = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_const(mut self, it: Duration) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: Duration) -> &mut Self {
        self.r#const = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: Duration) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: Duration) -> &mut Self {
        self.lt = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: Duration) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: Duration) -> &mut Self {
        self.lte = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: Duration) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: Duration) -> &mut Self {
        self.gt = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: Duration) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: Duration) -> &mut Self {
        self.gte = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_in(mut self, it: Duration) -> Self {
        self.r#add_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_in(&mut self, it: Duration) -> &mut Self {
        self.r#in.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_not_in(mut self, it: Duration) -> Self {
        self.r#add_not_in(it);
        self
    }
    #[inline(always)]
    pub fn r#add_not_in(&mut self, it: Duration) -> &mut Self {
        self.not_in.push(it);
        self
    }
}
impl textformat::Decodable for DurationRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("required") => {
                textformat::Field::merge(&mut self.required, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("in") => {
                textformat::Field::merge(&mut self.r#in, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("not_in") => {
                textformat::Field::merge(&mut self.not_in, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DurationRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.required != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("required: ");
            textformat::Field::format(&self.required, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#const != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("const ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("lt ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("lte ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("gt ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("gte ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#in != <Vec<Duration> as Default>::default() {
            out.indent(pad);
            out.push_str("in ");
            textformat::Field::format(&self.r#in, ctx, pad, out)?;
            out.push('\n');
        }
        if self.not_in != <Vec<Duration> as Default>::default() {
            out.indent(pad);
            out.push_str("not_in ");
            textformat::Field::format(&self.not_in, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DurationRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            18u32 => {
                buf = Format::<Nest>::decode(&mut self.r#const, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.lt, buf)?;
            }
            34u32 => {
                buf = Format::<Nest>::decode(&mut self.lte, buf)?;
            }
            42u32 => {
                buf = Format::<Nest>::decode(&mut self.gt, buf)?;
            }
            50u32 => {
                buf = Format::<Nest>::decode(&mut self.gte, buf)?;
            }
            58u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.r#in, buf)?;
            }
            66u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.not_in, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DurationRules {
    fn qualified_name(&self) -> &'static str {
        "validate.DurationRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.required.should_encode(false) {
            Format::<Fix>::encode(&self.required, 1u32, buf)?;
        }
        if self.r#const.should_encode(false) {
            Format::<Nest>::encode(&self.r#const, 2u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Nest>::encode(&self.lt, 3u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Nest>::encode(&self.lte, 4u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Nest>::encode(&self.gt, 5u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Nest>::encode(&self.gte, 6u32, buf)?;
        }
        if self.r#in.should_encode(false) {
            Format::<Repeat::<Nest>>::encode(&self.r#in, 7u32, buf)?;
        }
        if self.not_in.should_encode(false) {
            Format::<Repeat::<Nest>>::encode(&self.not_in, 8u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TimestampRules {
    pub required: Option<bool>,
    pub r#const: Option<Box<Timestamp>>,
    pub lt: Option<Box<Timestamp>>,
    pub lte: Option<Box<Timestamp>>,
    pub gt: Option<Box<Timestamp>>,
    pub gte: Option<Box<Timestamp>>,
    pub lt_now: Option<bool>,
    pub gt_now: Option<bool>,
    pub within: Option<Box<Duration>>,
    pub _unknown: (),
}
impl TimestampRules {
    #[inline(always)]
    pub fn r#with_required(mut self, it: bool) -> Self {
        self.r#set_required(it);
        self
    }
    #[inline(always)]
    pub fn r#set_required(&mut self, it: bool) -> &mut Self {
        self.required = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_const(mut self, it: Timestamp) -> Self {
        self.r#set_const(it);
        self
    }
    #[inline(always)]
    pub fn r#set_const(&mut self, it: Timestamp) -> &mut Self {
        self.r#const = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt(mut self, it: Timestamp) -> Self {
        self.r#set_lt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt(&mut self, it: Timestamp) -> &mut Self {
        self.lt = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_lte(mut self, it: Timestamp) -> Self {
        self.r#set_lte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lte(&mut self, it: Timestamp) -> &mut Self {
        self.lte = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt(mut self, it: Timestamp) -> Self {
        self.r#set_gt(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt(&mut self, it: Timestamp) -> &mut Self {
        self.gt = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_gte(mut self, it: Timestamp) -> Self {
        self.r#set_gte(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gte(&mut self, it: Timestamp) -> &mut Self {
        self.gte = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_lt_now(mut self, it: bool) -> Self {
        self.r#set_lt_now(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lt_now(&mut self, it: bool) -> &mut Self {
        self.lt_now = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_gt_now(mut self, it: bool) -> Self {
        self.r#set_gt_now(it);
        self
    }
    #[inline(always)]
    pub fn r#set_gt_now(&mut self, it: bool) -> &mut Self {
        self.gt_now = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_within(mut self, it: Duration) -> Self {
        self.r#set_within(it);
        self
    }
    #[inline(always)]
    pub fn r#set_within(&mut self, it: Duration) -> &mut Self {
        self.within = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for TimestampRules {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("required") => {
                textformat::Field::merge(&mut self.required, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("const") => {
                textformat::Field::merge(&mut self.r#const, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt") => {
                textformat::Field::merge(&mut self.lt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lte") => {
                textformat::Field::merge(&mut self.lte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt") => {
                textformat::Field::merge(&mut self.gt, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gte") => {
                textformat::Field::merge(&mut self.gte, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lt_now") => {
                textformat::Field::merge(&mut self.lt_now, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("gt_now") => {
                textformat::Field::merge(&mut self.gt_now, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("within") => {
                textformat::Field::merge(&mut self.within, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TimestampRules {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.required != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("required: ");
            textformat::Field::format(&self.required, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#const != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("const ");
            textformat::Field::format(&self.r#const, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("lt ");
            textformat::Field::format(&self.lt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lte != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("lte ");
            textformat::Field::format(&self.lte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("gt ");
            textformat::Field::format(&self.gt, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gte != <Option<Box<Timestamp>> as Default>::default() {
            out.indent(pad);
            out.push_str("gte ");
            textformat::Field::format(&self.gte, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lt_now != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("lt_now: ");
            textformat::Field::format(&self.lt_now, ctx, pad, out)?;
            out.push('\n');
        }
        if self.gt_now != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("gt_now: ");
            textformat::Field::format(&self.gt_now, ctx, pad, out)?;
            out.push('\n');
        }
        if self.within != <Option<Box<Duration>> as Default>::default() {
            out.indent(pad);
            out.push_str("within ");
            textformat::Field::format(&self.within, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TimestampRules {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            18u32 => {
                buf = Format::<Nest>::decode(&mut self.r#const, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.lt, buf)?;
            }
            34u32 => {
                buf = Format::<Nest>::decode(&mut self.lte, buf)?;
            }
            42u32 => {
                buf = Format::<Nest>::decode(&mut self.gt, buf)?;
            }
            50u32 => {
                buf = Format::<Nest>::decode(&mut self.gte, buf)?;
            }
            56u32 => {
                buf = Format::<Fix>::decode(&mut self.lt_now, buf)?;
            }
            58u32 => {
                buf = Format::<Fix>::decode(&mut self.lt_now, buf)?;
            }
            64u32 => {
                buf = Format::<Fix>::decode(&mut self.gt_now, buf)?;
            }
            66u32 => {
                buf = Format::<Fix>::decode(&mut self.gt_now, buf)?;
            }
            74u32 => {
                buf = Format::<Nest>::decode(&mut self.within, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TimestampRules {
    fn qualified_name(&self) -> &'static str {
        "validate.TimestampRules"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.required.should_encode(false) {
            Format::<Fix>::encode(&self.required, 1u32, buf)?;
        }
        if self.r#const.should_encode(false) {
            Format::<Nest>::encode(&self.r#const, 2u32, buf)?;
        }
        if self.lt.should_encode(false) {
            Format::<Nest>::encode(&self.lt, 3u32, buf)?;
        }
        if self.lte.should_encode(false) {
            Format::<Nest>::encode(&self.lte, 4u32, buf)?;
        }
        if self.gt.should_encode(false) {
            Format::<Nest>::encode(&self.gt, 5u32, buf)?;
        }
        if self.gte.should_encode(false) {
            Format::<Nest>::encode(&self.gte, 6u32, buf)?;
        }
        if self.lt_now.should_encode(false) {
            Format::<Fix>::encode(&self.lt_now, 7u32, buf)?;
        }
        if self.gt_now.should_encode(false) {
            Format::<Fix>::encode(&self.gt_now, 8u32, buf)?;
        }
        if self.within.should_encode(false) {
            Format::<Nest>::encode(&self.within, 9u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum KnownRegex {
    UNKNOWN,
    HTTP_HEADER_NAME,
    HTTP_HEADER_VALUE,
    Unknown(u32),
}
impl Default for KnownRegex {
    fn default() -> KnownRegex {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for KnownRegex {}
impl binformat::ShouldEncode for KnownRegex {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for KnownRegex {
    fn from(v: u32) -> KnownRegex {
        match v {
            0u32 => KnownRegex::UNKNOWN,
            1u32 => KnownRegex::HTTP_HEADER_NAME,
            2u32 => KnownRegex::HTTP_HEADER_VALUE,
            other => KnownRegex::Unknown(other),
        }
    }
}
impl From<KnownRegex> for u32 {
    fn from(v: KnownRegex) -> u32 {
        match v {
            KnownRegex::UNKNOWN => 0u32,
            KnownRegex::HTTP_HEADER_NAME => 1u32,
            KnownRegex::HTTP_HEADER_VALUE => 2u32,
            KnownRegex::Unknown(other) => other,
        }
    }
}
impl textformat::Field for KnownRegex {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            KnownRegex::UNKNOWN => "UNKNOWN",
            KnownRegex::HTTP_HEADER_NAME => "HTTP_HEADER_NAME",
            KnownRegex::HTTP_HEADER_VALUE => "HTTP_HEADER_VALUE",
            KnownRegex::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("UNKNOWN") => {
                *self = KnownRegex::UNKNOWN;
            }
            textformat::ast::Literal::Identifier("HTTP_HEADER_NAME") => {
                *self = KnownRegex::HTTP_HEADER_NAME;
            }
            textformat::ast::Literal::Identifier("HTTP_HEADER_VALUE") => {
                *self = KnownRegex::HTTP_HEADER_VALUE;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
