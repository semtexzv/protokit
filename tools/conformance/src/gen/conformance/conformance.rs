#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&FailureSet::default());
    registry.register(&ConformanceRequest::default());
    registry.register(&ConformanceResponse::default());
    registry.register(&JspbEncodingConfig::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FailureSet {
    pub failure: Vec<String>,
    pub _unknown: (),
}
impl FailureSet {
    #[inline(always)]
    pub fn r#with_failure(mut self, it: String) -> Self {
        self.r#add_failure(it);
        self
    }
    #[inline(always)]
    pub fn r#add_failure(&mut self, it: String) -> &mut Self {
        self.failure.push(it);
        self
    }
}
impl textformat::Decodable for FailureSet {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("failure") => {
                textformat::Field::merge(&mut self.failure, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FailureSet {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.failure != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("failure: ");
            textformat::Field::format(&self.failure, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FailureSet {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.failure, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FailureSet {
    fn qualified_name(&self) -> &'static str {
        "conformance.FailureSet"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Bytes>>::encode(&self.failure, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConformanceRequest {
    pub requested_output_format: WireFormat,
    pub message_type: String,
    pub test_category: TestCategory,
    pub jspb_encoding_options: Option<Box<JspbEncodingConfig>>,
    pub print_unknown_fields: bool,
    pub payload: ConformanceRequestOneOfPayload,
    pub _unknown: (),
}
impl ConformanceRequest {
    #[inline(always)]
    pub fn r#with_requested_output_format(mut self, it: WireFormat) -> Self {
        self.r#set_requested_output_format(it);
        self
    }
    #[inline(always)]
    pub fn r#set_requested_output_format(&mut self, it: WireFormat) -> &mut Self {
        self.requested_output_format = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_message_type(mut self, it: String) -> Self {
        self.r#set_message_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message_type(&mut self, it: String) -> &mut Self {
        self.message_type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_test_category(mut self, it: TestCategory) -> Self {
        self.r#set_test_category(it);
        self
    }
    #[inline(always)]
    pub fn r#set_test_category(&mut self, it: TestCategory) -> &mut Self {
        self.test_category = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_jspb_encoding_options(mut self, it: JspbEncodingConfig) -> Self {
        self.r#set_jspb_encoding_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_jspb_encoding_options(&mut self, it: JspbEncodingConfig) -> &mut Self {
        self.jspb_encoding_options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_print_unknown_fields(mut self, it: bool) -> Self {
        self.r#set_print_unknown_fields(it);
        self
    }
    #[inline(always)]
    pub fn r#set_print_unknown_fields(&mut self, it: bool) -> &mut Self {
        self.print_unknown_fields = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_payload_protobuf_payload(mut self, it: Vec<u8>) -> Self {
        self.payload = ConformanceRequestOneOfPayload::ProtobufPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_payload_protobuf_payload(&mut self, it: Vec<u8>) -> &mut Self {
        self.payload = ConformanceRequestOneOfPayload::ProtobufPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_payload_json_payload(mut self, it: String) -> Self {
        self.payload = ConformanceRequestOneOfPayload::JsonPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_payload_json_payload(&mut self, it: String) -> &mut Self {
        self.payload = ConformanceRequestOneOfPayload::JsonPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_payload_jspb_payload(mut self, it: String) -> Self {
        self.payload = ConformanceRequestOneOfPayload::JspbPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_payload_jspb_payload(&mut self, it: String) -> &mut Self {
        self.payload = ConformanceRequestOneOfPayload::JspbPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_payload_text_payload(mut self, it: String) -> Self {
        self.payload = ConformanceRequestOneOfPayload::TextPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_payload_text_payload(&mut self, it: String) -> &mut Self {
        self.payload = ConformanceRequestOneOfPayload::TextPayload(it);
        self
    }
}
impl textformat::Decodable for ConformanceRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("requested_output_format") => {
                textformat::Field::merge(&mut self.requested_output_format, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("message_type") => {
                textformat::Field::merge(&mut self.message_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("test_category") => {
                textformat::Field::merge(&mut self.test_category, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("jspb_encoding_options") => {
                textformat::Field::merge(&mut self.jspb_encoding_options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("print_unknown_fields") => {
                textformat::Field::merge(&mut self.print_unknown_fields, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("protobuf_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.payload = ConformanceRequestOneOfPayload::ProtobufPayload(target);
            }
            textformat::ast::FieldName::Normal("json_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.payload = ConformanceRequestOneOfPayload::JsonPayload(target);
            }
            textformat::ast::FieldName::Normal("jspb_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.payload = ConformanceRequestOneOfPayload::JspbPayload(target);
            }
            textformat::ast::FieldName::Normal("text_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.payload = ConformanceRequestOneOfPayload::TextPayload(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ConformanceRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.requested_output_format != <WireFormat as Default>::default() {
            out.indent(pad);
            out.push_str("requested_output_format: ");
            textformat::Field::format(&self.requested_output_format, ctx, pad, out)?;
            out.push('\n');
        }
        if self.message_type != <String as Default>::default() {
            out.indent(pad);
            out.push_str("message_type: ");
            textformat::Field::format(&self.message_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.test_category != <TestCategory as Default>::default() {
            out.indent(pad);
            out.push_str("test_category: ");
            textformat::Field::format(&self.test_category, ctx, pad, out)?;
            out.push('\n');
        }
        if self.jspb_encoding_options
            != <Option<Box<JspbEncodingConfig>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("jspb_encoding_options ");
            textformat::Field::format(&self.jspb_encoding_options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.print_unknown_fields != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("print_unknown_fields: ");
            textformat::Field::format(&self.print_unknown_fields, ctx, pad, out)?;
            out.push('\n');
        }
        match &self.payload {
            ConformanceRequestOneOfPayload::ProtobufPayload(value) => {
                out.indent(pad);
                out.push_str("protobuf_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceRequestOneOfPayload::JsonPayload(value) => {
                out.indent(pad);
                out.push_str("json_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceRequestOneOfPayload::JspbPayload(value) => {
                out.indent(pad);
                out.push_str("jspb_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceRequestOneOfPayload::TextPayload(value) => {
                out.indent(pad);
                out.push_str("text_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceRequestOneOfPayload::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for ConformanceRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            24u32 => {
                buf = Format::<Enum>::decode(&mut self.requested_output_format, buf)?;
            }
            26u32 => {
                buf = Format::<Enum>::decode(&mut self.requested_output_format, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.message_type, buf)?;
            }
            40u32 => {
                buf = Format::<Enum>::decode(&mut self.test_category, buf)?;
            }
            42u32 => {
                buf = Format::<Enum>::decode(&mut self.test_category, buf)?;
            }
            50u32 => {
                buf = Format::<Nest>::decode(&mut self.jspb_encoding_options, buf)?;
            }
            72u32 => {
                buf = Format::<Fix>::decode(&mut self.print_unknown_fields, buf)?;
            }
            74u32 => {
                buf = Format::<Fix>::decode(&mut self.print_unknown_fields, buf)?;
            }
            10u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.payload = ConformanceRequestOneOfPayload::ProtobufPayload(tmp);
            }
            18u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.payload = ConformanceRequestOneOfPayload::JsonPayload(tmp);
            }
            58u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.payload = ConformanceRequestOneOfPayload::JspbPayload(tmp);
            }
            66u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.payload = ConformanceRequestOneOfPayload::TextPayload(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ConformanceRequest {
    fn qualified_name(&self) -> &'static str {
        "conformance.ConformanceRequest"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Enum>::encode(&self.requested_output_format, 24u32, buf)?;
        Format::<Bytes>::encode(&self.message_type, 34u32, buf)?;
        Format::<Enum>::encode(&self.test_category, 40u32, buf)?;
        Format::<Nest>::encode(&self.jspb_encoding_options, 50u32, buf)?;
        Format::<Fix>::encode(&self.print_unknown_fields, 72u32, buf)?;
        match &self.payload {
            ConformanceRequestOneOfPayload::ProtobufPayload(value) => {
                Format::<Bytes>::encode(value, 10u32, buf)?;
            }
            ConformanceRequestOneOfPayload::JsonPayload(value) => {
                Format::<Bytes>::encode(value, 18u32, buf)?;
            }
            ConformanceRequestOneOfPayload::JspbPayload(value) => {
                Format::<Bytes>::encode(value, 58u32, buf)?;
            }
            ConformanceRequestOneOfPayload::TextPayload(value) => {
                Format::<Bytes>::encode(value, 66u32, buf)?;
            }
            ConformanceRequestOneOfPayload::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConformanceRequestOneOfPayload {
    ProtobufPayload(Vec<u8>),
    JsonPayload(String),
    JspbPayload(String),
    TextPayload(String),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for ConformanceRequestOneOfPayload {
    fn default() -> Self {
        ConformanceRequestOneOfPayload::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConformanceResponse {
    pub result: ConformanceResponseOneOfResult,
    pub _unknown: (),
}
impl ConformanceResponse {
    #[inline(always)]
    pub fn r#with_result_parse_error(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::ParseError(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_parse_error(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::ParseError(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_serialize_error(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::SerializeError(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_serialize_error(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::SerializeError(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_timeout_error(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::TimeoutError(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_timeout_error(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::TimeoutError(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_runtime_error(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::RuntimeError(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_runtime_error(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::RuntimeError(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_protobuf_payload(mut self, it: Vec<u8>) -> Self {
        self.result = ConformanceResponseOneOfResult::ProtobufPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_protobuf_payload(&mut self, it: Vec<u8>) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::ProtobufPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_json_payload(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::JsonPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_json_payload(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::JsonPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_skipped(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::Skipped(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_skipped(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::Skipped(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_jspb_payload(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::JspbPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_jspb_payload(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::JspbPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#with_result_text_payload(mut self, it: String) -> Self {
        self.result = ConformanceResponseOneOfResult::TextPayload(it);
        self
    }
    #[inline(always)]
    pub fn r#set_result_text_payload(&mut self, it: String) -> &mut Self {
        self.result = ConformanceResponseOneOfResult::TextPayload(it);
        self
    }
}
impl textformat::Decodable for ConformanceResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("parse_error") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::ParseError(target);
            }
            textformat::ast::FieldName::Normal("serialize_error") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::SerializeError(target);
            }
            textformat::ast::FieldName::Normal("timeout_error") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::TimeoutError(target);
            }
            textformat::ast::FieldName::Normal("runtime_error") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::RuntimeError(target);
            }
            textformat::ast::FieldName::Normal("protobuf_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::ProtobufPayload(target);
            }
            textformat::ast::FieldName::Normal("json_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::JsonPayload(target);
            }
            textformat::ast::FieldName::Normal("skipped") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::Skipped(target);
            }
            textformat::ast::FieldName::Normal("jspb_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::JspbPayload(target);
            }
            textformat::ast::FieldName::Normal("text_payload") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.result = ConformanceResponseOneOfResult::TextPayload(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ConformanceResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        match &self.result {
            ConformanceResponseOneOfResult::ParseError(value) => {
                out.indent(pad);
                out.push_str("parse_error: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::SerializeError(value) => {
                out.indent(pad);
                out.push_str("serialize_error: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::TimeoutError(value) => {
                out.indent(pad);
                out.push_str("timeout_error: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::RuntimeError(value) => {
                out.indent(pad);
                out.push_str("runtime_error: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::ProtobufPayload(value) => {
                out.indent(pad);
                out.push_str("protobuf_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::JsonPayload(value) => {
                out.indent(pad);
                out.push_str("json_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::Skipped(value) => {
                out.indent(pad);
                out.push_str("skipped: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::JspbPayload(value) => {
                out.indent(pad);
                out.push_str("jspb_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::TextPayload(value) => {
                out.indent(pad);
                out.push_str("text_payload: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            ConformanceResponseOneOfResult::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for ConformanceResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::ParseError(tmp);
            }
            50u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::SerializeError(tmp);
            }
            74u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::TimeoutError(tmp);
            }
            18u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::RuntimeError(tmp);
            }
            26u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::ProtobufPayload(tmp);
            }
            34u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::JsonPayload(tmp);
            }
            42u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::Skipped(tmp);
            }
            58u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::JspbPayload(tmp);
            }
            66u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.result = ConformanceResponseOneOfResult::TextPayload(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ConformanceResponse {
    fn qualified_name(&self) -> &'static str {
        "conformance.ConformanceResponse"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        match &self.result {
            ConformanceResponseOneOfResult::ParseError(value) => {
                Format::<Bytes>::encode(value, 10u32, buf)?;
            }
            ConformanceResponseOneOfResult::SerializeError(value) => {
                Format::<Bytes>::encode(value, 50u32, buf)?;
            }
            ConformanceResponseOneOfResult::TimeoutError(value) => {
                Format::<Bytes>::encode(value, 74u32, buf)?;
            }
            ConformanceResponseOneOfResult::RuntimeError(value) => {
                Format::<Bytes>::encode(value, 18u32, buf)?;
            }
            ConformanceResponseOneOfResult::ProtobufPayload(value) => {
                Format::<Bytes>::encode(value, 26u32, buf)?;
            }
            ConformanceResponseOneOfResult::JsonPayload(value) => {
                Format::<Bytes>::encode(value, 34u32, buf)?;
            }
            ConformanceResponseOneOfResult::Skipped(value) => {
                Format::<Bytes>::encode(value, 42u32, buf)?;
            }
            ConformanceResponseOneOfResult::JspbPayload(value) => {
                Format::<Bytes>::encode(value, 58u32, buf)?;
            }
            ConformanceResponseOneOfResult::TextPayload(value) => {
                Format::<Bytes>::encode(value, 66u32, buf)?;
            }
            ConformanceResponseOneOfResult::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConformanceResponseOneOfResult {
    ParseError(String),
    SerializeError(String),
    TimeoutError(String),
    RuntimeError(String),
    ProtobufPayload(Vec<u8>),
    JsonPayload(String),
    Skipped(String),
    JspbPayload(String),
    TextPayload(String),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for ConformanceResponseOneOfResult {
    fn default() -> Self {
        ConformanceResponseOneOfResult::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct JspbEncodingConfig {
    pub use_jspb_array_any_format: bool,
    pub _unknown: (),
}
impl JspbEncodingConfig {
    #[inline(always)]
    pub fn r#with_use_jspb_array_any_format(mut self, it: bool) -> Self {
        self.r#set_use_jspb_array_any_format(it);
        self
    }
    #[inline(always)]
    pub fn r#set_use_jspb_array_any_format(&mut self, it: bool) -> &mut Self {
        self.use_jspb_array_any_format = it.into();
        self
    }
}
impl textformat::Decodable for JspbEncodingConfig {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("use_jspb_array_any_format") => {
                textformat::Field::merge(
                    &mut self.use_jspb_array_any_format,
                    ctx,
                    value,
                )?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for JspbEncodingConfig {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.use_jspb_array_any_format != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("use_jspb_array_any_format: ");
            textformat::Field::format(&self.use_jspb_array_any_format, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for JspbEncodingConfig {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.use_jspb_array_any_format, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.use_jspb_array_any_format, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for JspbEncodingConfig {
    fn qualified_name(&self) -> &'static str {
        "conformance.JspbEncodingConfig"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.use_jspb_array_any_format, 8u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum WireFormat {
    UNSPECIFIED,
    PROTOBUF,
    JSON,
    JSPB,
    TEXT_FORMAT,
    Unknown(u32),
}
impl Default for WireFormat {
    fn default() -> WireFormat {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for WireFormat {}
impl From<u32> for WireFormat {
    fn from(v: u32) -> WireFormat {
        match v {
            0u32 => WireFormat::UNSPECIFIED,
            1u32 => WireFormat::PROTOBUF,
            2u32 => WireFormat::JSON,
            3u32 => WireFormat::JSPB,
            4u32 => WireFormat::TEXT_FORMAT,
            other => WireFormat::Unknown(other),
        }
    }
}
impl From<WireFormat> for u32 {
    fn from(v: WireFormat) -> u32 {
        match v {
            WireFormat::UNSPECIFIED => 0u32,
            WireFormat::PROTOBUF => 1u32,
            WireFormat::JSON => 2u32,
            WireFormat::JSPB => 3u32,
            WireFormat::TEXT_FORMAT => 4u32,
            WireFormat::Unknown(other) => other,
        }
    }
}
impl textformat::Field for WireFormat {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            WireFormat::UNSPECIFIED => "UNSPECIFIED",
            WireFormat::PROTOBUF => "PROTOBUF",
            WireFormat::JSON => "JSON",
            WireFormat::JSPB => "JSPB",
            WireFormat::TEXT_FORMAT => "TEXT_FORMAT",
            WireFormat::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("UNSPECIFIED") => {
                *self = WireFormat::UNSPECIFIED;
            }
            textformat::ast::Literal::Identifier("PROTOBUF") => {
                *self = WireFormat::PROTOBUF;
            }
            textformat::ast::Literal::Identifier("JSON") => *self = WireFormat::JSON,
            textformat::ast::Literal::Identifier("JSPB") => *self = WireFormat::JSPB,
            textformat::ast::Literal::Identifier("TEXT_FORMAT") => {
                *self = WireFormat::TEXT_FORMAT;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TestCategory {
    UNSPECIFIED_TEST,
    BINARY_TEST,
    JSON_TEST,
    JSON_IGNORE_UNKNOWN_PARSING_TEST,
    JSPB_TEST,
    TEXT_FORMAT_TEST,
    Unknown(u32),
}
impl Default for TestCategory {
    fn default() -> TestCategory {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for TestCategory {}
impl From<u32> for TestCategory {
    fn from(v: u32) -> TestCategory {
        match v {
            0u32 => TestCategory::UNSPECIFIED_TEST,
            1u32 => TestCategory::BINARY_TEST,
            2u32 => TestCategory::JSON_TEST,
            3u32 => TestCategory::JSON_IGNORE_UNKNOWN_PARSING_TEST,
            4u32 => TestCategory::JSPB_TEST,
            5u32 => TestCategory::TEXT_FORMAT_TEST,
            other => TestCategory::Unknown(other),
        }
    }
}
impl From<TestCategory> for u32 {
    fn from(v: TestCategory) -> u32 {
        match v {
            TestCategory::UNSPECIFIED_TEST => 0u32,
            TestCategory::BINARY_TEST => 1u32,
            TestCategory::JSON_TEST => 2u32,
            TestCategory::JSON_IGNORE_UNKNOWN_PARSING_TEST => 3u32,
            TestCategory::JSPB_TEST => 4u32,
            TestCategory::TEXT_FORMAT_TEST => 5u32,
            TestCategory::Unknown(other) => other,
        }
    }
}
impl textformat::Field for TestCategory {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            TestCategory::UNSPECIFIED_TEST => "UNSPECIFIED_TEST",
            TestCategory::BINARY_TEST => "BINARY_TEST",
            TestCategory::JSON_TEST => "JSON_TEST",
            TestCategory::JSON_IGNORE_UNKNOWN_PARSING_TEST => {
                "JSON_IGNORE_UNKNOWN_PARSING_TEST"
            }
            TestCategory::JSPB_TEST => "JSPB_TEST",
            TestCategory::TEXT_FORMAT_TEST => "TEXT_FORMAT_TEST",
            TestCategory::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("UNSPECIFIED_TEST") => {
                *self = TestCategory::UNSPECIFIED_TEST;
            }
            textformat::ast::Literal::Identifier("BINARY_TEST") => {
                *self = TestCategory::BINARY_TEST;
            }
            textformat::ast::Literal::Identifier("JSON_TEST") => {
                *self = TestCategory::JSON_TEST;
            }
            textformat::ast::Literal::Identifier("JSON_IGNORE_UNKNOWN_PARSING_TEST") => {
                *self = TestCategory::JSON_IGNORE_UNKNOWN_PARSING_TEST;
            }
            textformat::ast::Literal::Identifier("JSPB_TEST") => {
                *self = TestCategory::JSPB_TEST;
            }
            textformat::ast::Literal::Identifier("TEXT_FORMAT_TEST") => {
                *self = TestCategory::TEXT_FORMAT_TEST;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
