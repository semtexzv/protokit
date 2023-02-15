#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::descriptor::*;
use root::types::descriptor::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Version::default());
    registry.register(&CodeGeneratorRequest::default());
    registry.register(&CodeGeneratorResponse::default());
    registry.register(&CodeGeneratorResponseFile::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Version {
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
    pub suffix: Option<String>,
    pub _unknown: (),
}
impl Version {
    #[inline(always)]
    pub fn r#with_major(mut self, it: i32) -> Self {
        self.r#set_major(it);
        self
    }
    #[inline(always)]
    pub fn r#set_major(&mut self, it: i32) -> &mut Self {
        self.major = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_minor(mut self, it: i32) -> Self {
        self.r#set_minor(it);
        self
    }
    #[inline(always)]
    pub fn r#set_minor(&mut self, it: i32) -> &mut Self {
        self.minor = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_patch(mut self, it: i32) -> Self {
        self.r#set_patch(it);
        self
    }
    #[inline(always)]
    pub fn r#set_patch(&mut self, it: i32) -> &mut Self {
        self.patch = it.into();
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
}
impl textformat::Decodable for Version {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("major") => {
                textformat::Field::merge(&mut self.major, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("minor") => {
                textformat::Field::merge(&mut self.minor, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("patch") => {
                textformat::Field::merge(&mut self.patch, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("suffix") => {
                textformat::Field::merge(&mut self.suffix, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Version {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.major != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("major: ");
            textformat::Field::format(&self.major, ctx, pad, out)?;
            out.push('\n');
        }
        if self.minor != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("minor: ");
            textformat::Field::format(&self.minor, ctx, pad, out)?;
            out.push('\n');
        }
        if self.patch != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("patch: ");
            textformat::Field::format(&self.patch, ctx, pad, out)?;
            out.push('\n');
        }
        if self.suffix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("suffix: ");
            textformat::Field::format(&self.suffix, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Version {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.major, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.major, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.minor, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.minor, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.patch, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.patch, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.suffix, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Version {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.compiler.Version"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.major.should_encode(false) {
            Format::<VInt>::encode(&self.major, 1u32, buf)?;
        }
        if self.minor.should_encode(false) {
            Format::<VInt>::encode(&self.minor, 2u32, buf)?;
        }
        if self.patch.should_encode(false) {
            Format::<VInt>::encode(&self.patch, 3u32, buf)?;
        }
        if self.suffix.should_encode(false) {
            Format::<Bytes>::encode(&self.suffix, 4u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: Vec<String>,
    pub parameter: Option<String>,
    pub proto_file: Vec<FileDescriptorProto>,
    pub compiler_version: Option<Box<Version>>,
    pub _unknown: (),
}
impl CodeGeneratorRequest {
    #[inline(always)]
    pub fn r#with_file_to_generate(mut self, it: String) -> Self {
        self.r#add_file_to_generate(it);
        self
    }
    #[inline(always)]
    pub fn r#add_file_to_generate(&mut self, it: String) -> &mut Self {
        self.file_to_generate.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_parameter(mut self, it: String) -> Self {
        self.r#set_parameter(it);
        self
    }
    #[inline(always)]
    pub fn r#set_parameter(&mut self, it: String) -> &mut Self {
        self.parameter = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_proto_file(mut self, it: FileDescriptorProto) -> Self {
        self.r#add_proto_file(it);
        self
    }
    #[inline(always)]
    pub fn r#add_proto_file(&mut self, it: FileDescriptorProto) -> &mut Self {
        self.proto_file.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_compiler_version(mut self, it: Version) -> Self {
        self.r#set_compiler_version(it);
        self
    }
    #[inline(always)]
    pub fn r#set_compiler_version(&mut self, it: Version) -> &mut Self {
        self.compiler_version = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for CodeGeneratorRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("file_to_generate") => {
                textformat::Field::merge(&mut self.file_to_generate, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("parameter") => {
                textformat::Field::merge(&mut self.parameter, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("proto_file") => {
                textformat::Field::merge(&mut self.proto_file, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("compiler_version") => {
                textformat::Field::merge(&mut self.compiler_version, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for CodeGeneratorRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.file_to_generate != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("file_to_generate: ");
            textformat::Field::format(&self.file_to_generate, ctx, pad, out)?;
            out.push('\n');
        }
        if self.parameter != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("parameter: ");
            textformat::Field::format(&self.parameter, ctx, pad, out)?;
            out.push('\n');
        }
        if self.proto_file != <Vec<FileDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("proto_file ");
            textformat::Field::format(&self.proto_file, ctx, pad, out)?;
            out.push('\n');
        }
        if self.compiler_version != <Option<Box<Version>> as Default>::default() {
            out.indent(pad);
            out.push_str("compiler_version ");
            textformat::Field::format(&self.compiler_version, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for CodeGeneratorRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<
                    Repeat::<Bytes>,
                >::decode(&mut self.file_to_generate, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.parameter, buf)?;
            }
            122u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.proto_file, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.compiler_version, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for CodeGeneratorRequest {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.compiler.CodeGeneratorRequest"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.file_to_generate.should_encode(false) {
            Format::<Repeat::<Bytes>>::encode(&self.file_to_generate, 1u32, buf)?;
        }
        if self.parameter.should_encode(false) {
            Format::<Bytes>::encode(&self.parameter, 2u32, buf)?;
        }
        if self.proto_file.should_encode(false) {
            Format::<Repeat::<Nest>>::encode(&self.proto_file, 15u32, buf)?;
        }
        if self.compiler_version.should_encode(false) {
            Format::<Nest>::encode(&self.compiler_version, 3u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CodeGeneratorResponse {
    pub error: Option<String>,
    pub supported_features: Option<u64>,
    pub file: Vec<CodeGeneratorResponseFile>,
    pub _unknown: (),
}
impl CodeGeneratorResponse {
    #[inline(always)]
    pub fn r#with_error(mut self, it: String) -> Self {
        self.r#set_error(it);
        self
    }
    #[inline(always)]
    pub fn r#set_error(&mut self, it: String) -> &mut Self {
        self.error = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_supported_features(mut self, it: u64) -> Self {
        self.r#set_supported_features(it);
        self
    }
    #[inline(always)]
    pub fn r#set_supported_features(&mut self, it: u64) -> &mut Self {
        self.supported_features = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_file(mut self, it: CodeGeneratorResponseFile) -> Self {
        self.r#add_file(it);
        self
    }
    #[inline(always)]
    pub fn r#add_file(&mut self, it: CodeGeneratorResponseFile) -> &mut Self {
        self.file.push(it);
        self
    }
}
impl textformat::Decodable for CodeGeneratorResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("error") => {
                textformat::Field::merge(&mut self.error, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("supported_features") => {
                textformat::Field::merge(&mut self.supported_features, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("file") => {
                textformat::Field::merge(&mut self.file, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for CodeGeneratorResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.error != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("error: ");
            textformat::Field::format(&self.error, ctx, pad, out)?;
            out.push('\n');
        }
        if self.supported_features != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("supported_features: ");
            textformat::Field::format(&self.supported_features, ctx, pad, out)?;
            out.push('\n');
        }
        if self.file != <Vec<CodeGeneratorResponseFile> as Default>::default() {
            out.indent(pad);
            out.push_str("file ");
            textformat::Field::format(&self.file, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for CodeGeneratorResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.error, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.supported_features, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.supported_features, buf)?;
            }
            122u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.file, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for CodeGeneratorResponse {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.compiler.CodeGeneratorResponse"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.error.should_encode(false) {
            Format::<Bytes>::encode(&self.error, 1u32, buf)?;
        }
        if self.supported_features.should_encode(false) {
            Format::<VInt>::encode(&self.supported_features, 2u32, buf)?;
        }
        if self.file.should_encode(false) {
            Format::<Repeat::<Nest>>::encode(&self.file, 15u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CodeGeneratorResponseFile {
    pub name: Option<String>,
    pub insertion_point: Option<String>,
    pub content: Option<String>,
    pub generated_code_info: Option<Box<GeneratedCodeInfo>>,
    pub _unknown: (),
}
impl CodeGeneratorResponseFile {
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
    pub fn r#with_insertion_point(mut self, it: String) -> Self {
        self.r#set_insertion_point(it);
        self
    }
    #[inline(always)]
    pub fn r#set_insertion_point(&mut self, it: String) -> &mut Self {
        self.insertion_point = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_content(mut self, it: String) -> Self {
        self.r#set_content(it);
        self
    }
    #[inline(always)]
    pub fn r#set_content(&mut self, it: String) -> &mut Self {
        self.content = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_generated_code_info(mut self, it: GeneratedCodeInfo) -> Self {
        self.r#set_generated_code_info(it);
        self
    }
    #[inline(always)]
    pub fn r#set_generated_code_info(&mut self, it: GeneratedCodeInfo) -> &mut Self {
        self.generated_code_info = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for CodeGeneratorResponseFile {
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
            textformat::ast::FieldName::Normal("insertion_point") => {
                textformat::Field::merge(&mut self.insertion_point, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("content") => {
                textformat::Field::merge(&mut self.content, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("generated_code_info") => {
                textformat::Field::merge(&mut self.generated_code_info, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for CodeGeneratorResponseFile {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.insertion_point != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("insertion_point: ");
            textformat::Field::format(&self.insertion_point, ctx, pad, out)?;
            out.push('\n');
        }
        if self.content != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("content: ");
            textformat::Field::format(&self.content, ctx, pad, out)?;
            out.push('\n');
        }
        if self.generated_code_info
            != <Option<Box<GeneratedCodeInfo>> as Default>::default()
        {
            out.indent(pad);
            out.push_str("generated_code_info ");
            textformat::Field::format(&self.generated_code_info, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for CodeGeneratorResponseFile {
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
                buf = Format::<Bytes>::decode(&mut self.insertion_point, buf)?;
            }
            122u32 => {
                buf = Format::<Bytes>::decode(&mut self.content, buf)?;
            }
            130u32 => {
                buf = Format::<Nest>::decode(&mut self.generated_code_info, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for CodeGeneratorResponseFile {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.compiler.CodeGeneratorResponse.File"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        use binformat::ShouldEncode;
        if self.name.should_encode(false) {
            Format::<Bytes>::encode(&self.name, 1u32, buf)?;
        }
        if self.insertion_point.should_encode(false) {
            Format::<Bytes>::encode(&self.insertion_point, 2u32, buf)?;
        }
        if self.content.should_encode(false) {
            Format::<Bytes>::encode(&self.content, 15u32, buf)?;
        }
        if self.generated_code_info.should_encode(false) {
            Format::<Nest>::encode(&self.generated_code_info, 16u32, buf)?;
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum CodeGeneratorResponseFeature {
    FEATURE_NONE,
    FEATURE_PROTO3_OPTIONAL,
    Unknown(u32),
}
impl Default for CodeGeneratorResponseFeature {
    fn default() -> CodeGeneratorResponseFeature {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for CodeGeneratorResponseFeature {}
impl binformat::ShouldEncode for CodeGeneratorResponseFeature {
    fn should_encode(&self, proto3: bool) -> bool {
        match self {
            Self::Unknown(_) => false,
            _ => true,
        }
    }
}
impl From<u32> for CodeGeneratorResponseFeature {
    fn from(v: u32) -> CodeGeneratorResponseFeature {
        match v {
            0u32 => CodeGeneratorResponseFeature::FEATURE_NONE,
            1u32 => CodeGeneratorResponseFeature::FEATURE_PROTO3_OPTIONAL,
            other => CodeGeneratorResponseFeature::Unknown(other),
        }
    }
}
impl From<CodeGeneratorResponseFeature> for u32 {
    fn from(v: CodeGeneratorResponseFeature) -> u32 {
        match v {
            CodeGeneratorResponseFeature::FEATURE_NONE => 0u32,
            CodeGeneratorResponseFeature::FEATURE_PROTO3_OPTIONAL => 1u32,
            CodeGeneratorResponseFeature::Unknown(other) => other,
        }
    }
}
impl textformat::Field for CodeGeneratorResponseFeature {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            CodeGeneratorResponseFeature::FEATURE_NONE => "FEATURE_NONE",
            CodeGeneratorResponseFeature::FEATURE_PROTO3_OPTIONAL => {
                "FEATURE_PROTO3_OPTIONAL"
            }
            CodeGeneratorResponseFeature::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("FEATURE_NONE") => {
                *self = CodeGeneratorResponseFeature::FEATURE_NONE;
            }
            textformat::ast::Literal::Identifier("FEATURE_PROTO3_OPTIONAL") => {
                *self = CodeGeneratorResponseFeature::FEATURE_PROTO3_OPTIONAL;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
