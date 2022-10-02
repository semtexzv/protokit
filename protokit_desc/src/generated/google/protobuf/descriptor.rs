#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use crate::*;
use crate as root;
use super::super::super::validate::validate::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&FileDescriptorSet::default());
    registry.register(&FileDescriptorProto::default());
    registry.register(&DescriptorProto::default());
    registry.register(&DescriptorProtoExtensionRange::default());
    registry.register(&DescriptorProtoReservedRange::default());
    registry.register(&ExtensionRangeOptions::default());
    registry.register(&FieldDescriptorProto::default());
    registry.register(&OneofDescriptorProto::default());
    registry.register(&EnumDescriptorProto::default());
    registry.register(&EnumDescriptorProtoEnumReservedRange::default());
    registry.register(&EnumValueDescriptorProto::default());
    registry.register(&ServiceDescriptorProto::default());
    registry.register(&MethodDescriptorProto::default());
    registry.register(&FileOptions::default());
    registry.register(&MessageOptions::default());
    registry.register(&FieldOptions::default());
    registry.register(&OneofOptions::default());
    registry.register(&EnumOptions::default());
    registry.register(&EnumValueOptions::default());
    registry.register(&ServiceOptions::default());
    registry.register(&MethodOptions::default());
    registry.register(&UninterpretedOption::default());
    registry.register(&UninterpretedOptionNamePart::default());
    registry.register(&SourceCodeInfo::default());
    registry.register(&SourceCodeInfoLocation::default());
    registry.register(&GeneratedCodeInfo::default());
    registry.register(&GeneratedCodeInfoAnnotation::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileDescriptorSet {
    pub file: Vec<FileDescriptorProto>,
    pub _unknown: (),
}
impl FileDescriptorSet {
    #[inline(always)]
    pub fn r#with_file(mut self, it: FileDescriptorProto) -> Self {
        self.r#add_file(it);
        self
    }
    #[inline(always)]
    pub fn r#add_file(&mut self, it: FileDescriptorProto) -> &mut Self {
        self.file.push(it);
        self
    }
}
impl textformat::Decodable for FileDescriptorSet {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("file") => {
                textformat::Field::merge(&mut self.file, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FileDescriptorSet {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.file != <Vec<FileDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("file ");
            textformat::Field::format(&self.file, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FileDescriptorSet {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.file, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FileDescriptorSet {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FileDescriptorSet"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.file, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileDescriptorProto {
    pub name: Option<String>,
    pub package: Option<String>,
    pub dependency: Vec<String>,
    pub public_dependency: Vec<i32>,
    pub weak_dependency: Vec<i32>,
    pub message_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub service: Vec<ServiceDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub options: Option<Box<FileOptions>>,
    pub source_code_info: Option<Box<SourceCodeInfo>>,
    pub syntax: Option<String>,
    pub _unknown: (),
}
impl FileDescriptorProto {
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
    pub fn r#with_package(mut self, it: String) -> Self {
        self.r#set_package(it);
        self
    }
    #[inline(always)]
    pub fn r#set_package(&mut self, it: String) -> &mut Self {
        self.package = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_dependency(mut self, it: String) -> Self {
        self.r#add_dependency(it);
        self
    }
    #[inline(always)]
    pub fn r#add_dependency(&mut self, it: String) -> &mut Self {
        self.dependency.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_public_dependency(mut self, it: i32) -> Self {
        self.r#add_public_dependency(it);
        self
    }
    #[inline(always)]
    pub fn r#add_public_dependency(&mut self, it: i32) -> &mut Self {
        self.public_dependency.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_weak_dependency(mut self, it: i32) -> Self {
        self.r#add_weak_dependency(it);
        self
    }
    #[inline(always)]
    pub fn r#add_weak_dependency(&mut self, it: i32) -> &mut Self {
        self.weak_dependency.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_message_type(mut self, it: DescriptorProto) -> Self {
        self.r#add_message_type(it);
        self
    }
    #[inline(always)]
    pub fn r#add_message_type(&mut self, it: DescriptorProto) -> &mut Self {
        self.message_type.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_enum_type(mut self, it: EnumDescriptorProto) -> Self {
        self.r#add_enum_type(it);
        self
    }
    #[inline(always)]
    pub fn r#add_enum_type(&mut self, it: EnumDescriptorProto) -> &mut Self {
        self.enum_type.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_service(mut self, it: ServiceDescriptorProto) -> Self {
        self.r#add_service(it);
        self
    }
    #[inline(always)]
    pub fn r#add_service(&mut self, it: ServiceDescriptorProto) -> &mut Self {
        self.service.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_extension(mut self, it: FieldDescriptorProto) -> Self {
        self.r#add_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#add_extension(&mut self, it: FieldDescriptorProto) -> &mut Self {
        self.extension.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: FileOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: FileOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_source_code_info(mut self, it: SourceCodeInfo) -> Self {
        self.r#set_source_code_info(it);
        self
    }
    #[inline(always)]
    pub fn r#set_source_code_info(&mut self, it: SourceCodeInfo) -> &mut Self {
        self.source_code_info = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_syntax(mut self, it: String) -> Self {
        self.r#set_syntax(it);
        self
    }
    #[inline(always)]
    pub fn r#set_syntax(&mut self, it: String) -> &mut Self {
        self.syntax = it.into();
        self
    }
}
impl textformat::Decodable for FileDescriptorProto {
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
            textformat::ast::FieldName::Normal("package") => {
                textformat::Field::merge(&mut self.package, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("dependency") => {
                textformat::Field::merge(&mut self.dependency, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("public_dependency") => {
                textformat::Field::merge(&mut self.public_dependency, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("weak_dependency") => {
                textformat::Field::merge(&mut self.weak_dependency, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("message_type") => {
                textformat::Field::merge(&mut self.message_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("enum_type") => {
                textformat::Field::merge(&mut self.enum_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("service") => {
                textformat::Field::merge(&mut self.service, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("extension") => {
                textformat::Field::merge(&mut self.extension, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("source_code_info") => {
                textformat::Field::merge(&mut self.source_code_info, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("syntax") => {
                textformat::Field::merge(&mut self.syntax, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FileDescriptorProto {
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
        if self.package != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("package: ");
            textformat::Field::format(&self.package, ctx, pad, out)?;
            out.push('\n');
        }
        if self.dependency != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("dependency: ");
            textformat::Field::format(&self.dependency, ctx, pad, out)?;
            out.push('\n');
        }
        if self.public_dependency != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("public_dependency: ");
            textformat::Field::format(&self.public_dependency, ctx, pad, out)?;
            out.push('\n');
        }
        if self.weak_dependency != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("weak_dependency: ");
            textformat::Field::format(&self.weak_dependency, ctx, pad, out)?;
            out.push('\n');
        }
        if self.message_type != <Vec<DescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("message_type ");
            textformat::Field::format(&self.message_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.enum_type != <Vec<EnumDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("enum_type ");
            textformat::Field::format(&self.enum_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.service != <Vec<ServiceDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("service ");
            textformat::Field::format(&self.service, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extension != <Vec<FieldDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("extension ");
            textformat::Field::format(&self.extension, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<FileOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.source_code_info != <Option<Box<SourceCodeInfo>> as Default>::default() {
            out.indent(pad);
            out.push_str("source_code_info ");
            textformat::Field::format(&self.source_code_info, ctx, pad, out)?;
            out.push('\n');
        }
        if self.syntax != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("syntax: ");
            textformat::Field::format(&self.syntax, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FileDescriptorProto {
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
                buf = Format::<Bytes>::decode(&mut self.package, buf)?;
            }
            26u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.dependency, buf)?;
            }
            80u32 => {
                buf = Format::<
                    Repeat::<VInt>,
                >::decode(&mut self.public_dependency, buf)?;
            }
            82u32 => {
                buf = Format::<
                    Repeat::<VInt>,
                >::decode(&mut self.public_dependency, buf)?;
            }
            88u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.weak_dependency, buf)?;
            }
            90u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.weak_dependency, buf)?;
            }
            34u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.message_type, buf)?;
            }
            42u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.enum_type, buf)?;
            }
            50u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.service, buf)?;
            }
            58u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.extension, buf)?;
            }
            66u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            74u32 => {
                buf = Format::<Nest>::decode(&mut self.source_code_info, buf)?;
            }
            98u32 => {
                buf = Format::<Bytes>::decode(&mut self.syntax, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FileDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FileDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Bytes>::encode(&self.package, 18u32, buf)?;
        Format::<Repeat::<Bytes>>::encode(&self.dependency, 26u32, buf)?;
        Format::<Repeat::<VInt>>::encode(&self.public_dependency, 80u32, buf)?;
        Format::<Repeat::<VInt>>::encode(&self.weak_dependency, 88u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.message_type, 34u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.enum_type, 42u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.service, 50u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.extension, 58u32, buf)?;
        Format::<Nest>::encode(&self.options, 66u32, buf)?;
        Format::<Nest>::encode(&self.source_code_info, 74u32, buf)?;
        Format::<Bytes>::encode(&self.syntax, 98u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DescriptorProto {
    pub name: Option<String>,
    pub field: Vec<FieldDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub nested_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub extension_range: Vec<DescriptorProtoExtensionRange>,
    pub oneof_decl: Vec<OneofDescriptorProto>,
    pub options: Option<Box<MessageOptions>>,
    pub reserved_range: Vec<DescriptorProtoReservedRange>,
    pub reserved_name: Vec<String>,
    pub _unknown: (),
}
impl DescriptorProto {
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
    pub fn r#with_field(mut self, it: FieldDescriptorProto) -> Self {
        self.r#add_field(it);
        self
    }
    #[inline(always)]
    pub fn r#add_field(&mut self, it: FieldDescriptorProto) -> &mut Self {
        self.field.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_extension(mut self, it: FieldDescriptorProto) -> Self {
        self.r#add_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#add_extension(&mut self, it: FieldDescriptorProto) -> &mut Self {
        self.extension.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_nested_type(mut self, it: DescriptorProto) -> Self {
        self.r#add_nested_type(it);
        self
    }
    #[inline(always)]
    pub fn r#add_nested_type(&mut self, it: DescriptorProto) -> &mut Self {
        self.nested_type.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_enum_type(mut self, it: EnumDescriptorProto) -> Self {
        self.r#add_enum_type(it);
        self
    }
    #[inline(always)]
    pub fn r#add_enum_type(&mut self, it: EnumDescriptorProto) -> &mut Self {
        self.enum_type.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_extension_range(mut self, it: DescriptorProtoExtensionRange) -> Self {
        self.r#add_extension_range(it);
        self
    }
    #[inline(always)]
    pub fn r#add_extension_range(
        &mut self,
        it: DescriptorProtoExtensionRange,
    ) -> &mut Self {
        self.extension_range.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oneof_decl(mut self, it: OneofDescriptorProto) -> Self {
        self.r#add_oneof_decl(it);
        self
    }
    #[inline(always)]
    pub fn r#add_oneof_decl(&mut self, it: OneofDescriptorProto) -> &mut Self {
        self.oneof_decl.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: MessageOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: MessageOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_reserved_range(mut self, it: DescriptorProtoReservedRange) -> Self {
        self.r#add_reserved_range(it);
        self
    }
    #[inline(always)]
    pub fn r#add_reserved_range(
        &mut self,
        it: DescriptorProtoReservedRange,
    ) -> &mut Self {
        self.reserved_range.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_reserved_name(mut self, it: String) -> Self {
        self.r#add_reserved_name(it);
        self
    }
    #[inline(always)]
    pub fn r#add_reserved_name(&mut self, it: String) -> &mut Self {
        self.reserved_name.push(it);
        self
    }
}
impl textformat::Decodable for DescriptorProto {
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
            textformat::ast::FieldName::Normal("field") => {
                textformat::Field::merge(&mut self.field, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("extension") => {
                textformat::Field::merge(&mut self.extension, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("nested_type") => {
                textformat::Field::merge(&mut self.nested_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("enum_type") => {
                textformat::Field::merge(&mut self.enum_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("extension_range") => {
                textformat::Field::merge(&mut self.extension_range, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oneof_decl") => {
                textformat::Field::merge(&mut self.oneof_decl, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("reserved_range") => {
                textformat::Field::merge(&mut self.reserved_range, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("reserved_name") => {
                textformat::Field::merge(&mut self.reserved_name, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DescriptorProto {
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
        if self.field != <Vec<FieldDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("field ");
            textformat::Field::format(&self.field, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extension != <Vec<FieldDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("extension ");
            textformat::Field::format(&self.extension, ctx, pad, out)?;
            out.push('\n');
        }
        if self.nested_type != <Vec<DescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("nested_type ");
            textformat::Field::format(&self.nested_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.enum_type != <Vec<EnumDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("enum_type ");
            textformat::Field::format(&self.enum_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extension_range
            != <Vec<DescriptorProtoExtensionRange> as Default>::default()
        {
            out.indent(pad);
            out.push_str("extension_range ");
            textformat::Field::format(&self.extension_range, ctx, pad, out)?;
            out.push('\n');
        }
        if self.oneof_decl != <Vec<OneofDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("oneof_decl ");
            textformat::Field::format(&self.oneof_decl, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<MessageOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.reserved_range
            != <Vec<DescriptorProtoReservedRange> as Default>::default()
        {
            out.indent(pad);
            out.push_str("reserved_range ");
            textformat::Field::format(&self.reserved_range, ctx, pad, out)?;
            out.push('\n');
        }
        if self.reserved_name != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("reserved_name: ");
            textformat::Field::format(&self.reserved_name, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DescriptorProto {
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
                buf = Format::<Repeat::<Nest>>::decode(&mut self.field, buf)?;
            }
            50u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.extension, buf)?;
            }
            26u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.nested_type, buf)?;
            }
            34u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.enum_type, buf)?;
            }
            42u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.extension_range, buf)?;
            }
            66u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.oneof_decl, buf)?;
            }
            58u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            74u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.reserved_range, buf)?;
            }
            82u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.reserved_name, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.DescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.field, 18u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.extension, 50u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.nested_type, 26u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.enum_type, 34u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.extension_range, 42u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.oneof_decl, 66u32, buf)?;
        Format::<Nest>::encode(&self.options, 58u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.reserved_range, 74u32, buf)?;
        Format::<Repeat::<Bytes>>::encode(&self.reserved_name, 82u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DescriptorProtoExtensionRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub options: Option<Box<ExtensionRangeOptions>>,
    pub _unknown: (),
}
impl DescriptorProtoExtensionRange {
    #[inline(always)]
    pub fn r#with_start(mut self, it: i32) -> Self {
        self.r#set_start(it);
        self
    }
    #[inline(always)]
    pub fn r#set_start(&mut self, it: i32) -> &mut Self {
        self.start = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_end(mut self, it: i32) -> Self {
        self.r#set_end(it);
        self
    }
    #[inline(always)]
    pub fn r#set_end(&mut self, it: i32) -> &mut Self {
        self.end = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: ExtensionRangeOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: ExtensionRangeOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for DescriptorProtoExtensionRange {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("start") => {
                textformat::Field::merge(&mut self.start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("end") => {
                textformat::Field::merge(&mut self.end, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DescriptorProtoExtensionRange {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.start != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("start: ");
            textformat::Field::format(&self.start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.end != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("end: ");
            textformat::Field::format(&self.end, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<ExtensionRangeOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DescriptorProtoExtensionRange {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DescriptorProtoExtensionRange {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.DescriptorProto.ExtensionRange"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<VInt>::encode(&self.start, 8u32, buf)?;
        Format::<VInt>::encode(&self.end, 16u32, buf)?;
        Format::<Nest>::encode(&self.options, 26u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DescriptorProtoReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub _unknown: (),
}
impl DescriptorProtoReservedRange {
    #[inline(always)]
    pub fn r#with_start(mut self, it: i32) -> Self {
        self.r#set_start(it);
        self
    }
    #[inline(always)]
    pub fn r#set_start(&mut self, it: i32) -> &mut Self {
        self.start = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_end(mut self, it: i32) -> Self {
        self.r#set_end(it);
        self
    }
    #[inline(always)]
    pub fn r#set_end(&mut self, it: i32) -> &mut Self {
        self.end = it.into();
        self
    }
}
impl textformat::Decodable for DescriptorProtoReservedRange {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("start") => {
                textformat::Field::merge(&mut self.start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("end") => {
                textformat::Field::merge(&mut self.end, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DescriptorProtoReservedRange {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.start != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("start: ");
            textformat::Field::format(&self.start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.end != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("end: ");
            textformat::Field::format(&self.end, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DescriptorProtoReservedRange {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DescriptorProtoReservedRange {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.DescriptorProto.ReservedRange"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<VInt>::encode(&self.start, 8u32, buf)?;
        Format::<VInt>::encode(&self.end, 16u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl ExtensionRangeOptions {
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for ExtensionRangeOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ExtensionRangeOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ExtensionRangeOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ExtensionRangeOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.ExtensionRangeOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub label: Option<FieldDescriptorProtoLabel>,
    pub r#type: Option<FieldDescriptorProtoType>,
    pub type_name: Option<String>,
    pub extendee: Option<String>,
    pub default_value: Option<String>,
    pub oneof_index: Option<i32>,
    pub json_name: Option<String>,
    pub options: Option<Box<FieldOptions>>,
    pub proto3_optional: Option<bool>,
    pub _unknown: (),
}
impl FieldDescriptorProto {
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
    pub fn r#with_label(mut self, it: FieldDescriptorProtoLabel) -> Self {
        self.r#set_label(it);
        self
    }
    #[inline(always)]
    pub fn r#set_label(&mut self, it: FieldDescriptorProtoLabel) -> &mut Self {
        self.label = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: FieldDescriptorProtoType) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: FieldDescriptorProtoType) -> &mut Self {
        self.r#type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type_name(mut self, it: String) -> Self {
        self.r#set_type_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type_name(&mut self, it: String) -> &mut Self {
        self.type_name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_extendee(mut self, it: String) -> Self {
        self.r#set_extendee(it);
        self
    }
    #[inline(always)]
    pub fn r#set_extendee(&mut self, it: String) -> &mut Self {
        self.extendee = it.into();
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
    pub fn r#with_options(mut self, it: FieldOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: FieldOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_proto3_optional(mut self, it: bool) -> Self {
        self.r#set_proto3_optional(it);
        self
    }
    #[inline(always)]
    pub fn r#set_proto3_optional(&mut self, it: bool) -> &mut Self {
        self.proto3_optional = it.into();
        self
    }
}
impl textformat::Decodable for FieldDescriptorProto {
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
            textformat::ast::FieldName::Normal("label") => {
                textformat::Field::merge(&mut self.label, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type_name") => {
                textformat::Field::merge(&mut self.type_name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("extendee") => {
                textformat::Field::merge(&mut self.extendee, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("default_value") => {
                textformat::Field::merge(&mut self.default_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oneof_index") => {
                textformat::Field::merge(&mut self.oneof_index, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("json_name") => {
                textformat::Field::merge(&mut self.json_name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("proto3_optional") => {
                textformat::Field::merge(&mut self.proto3_optional, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FieldDescriptorProto {
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
        if self.number != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("number: ");
            textformat::Field::format(&self.number, ctx, pad, out)?;
            out.push('\n');
        }
        if self.label != <Option<FieldDescriptorProtoLabel> as Default>::default() {
            out.indent(pad);
            out.push_str("label: ");
            textformat::Field::format(&self.label, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <Option<FieldDescriptorProtoType> as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.type_name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("type_name: ");
            textformat::Field::format(&self.type_name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extendee != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("extendee: ");
            textformat::Field::format(&self.extendee, ctx, pad, out)?;
            out.push('\n');
        }
        if self.default_value != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("default_value: ");
            textformat::Field::format(&self.default_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.oneof_index != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("oneof_index: ");
            textformat::Field::format(&self.oneof_index, ctx, pad, out)?;
            out.push('\n');
        }
        if self.json_name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("json_name: ");
            textformat::Field::format(&self.json_name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<FieldOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.proto3_optional != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("proto3_optional: ");
            textformat::Field::format(&self.proto3_optional, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldDescriptorProto {
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
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.number, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.number, buf)?;
            }
            32u32 => {
                buf = Format::<Enum>::decode(&mut self.label, buf)?;
            }
            34u32 => {
                buf = Format::<Enum>::decode(&mut self.label, buf)?;
            }
            40u32 => {
                buf = Format::<Enum>::decode(&mut self.r#type, buf)?;
            }
            42u32 => {
                buf = Format::<Enum>::decode(&mut self.r#type, buf)?;
            }
            50u32 => {
                buf = Format::<Bytes>::decode(&mut self.type_name, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.extendee, buf)?;
            }
            58u32 => {
                buf = Format::<Bytes>::decode(&mut self.default_value, buf)?;
            }
            72u32 => {
                buf = Format::<VInt>::decode(&mut self.oneof_index, buf)?;
            }
            74u32 => {
                buf = Format::<VInt>::decode(&mut self.oneof_index, buf)?;
            }
            82u32 => {
                buf = Format::<Bytes>::decode(&mut self.json_name, buf)?;
            }
            66u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            136u32 => {
                buf = Format::<Fix>::decode(&mut self.proto3_optional, buf)?;
            }
            138u32 => {
                buf = Format::<Fix>::decode(&mut self.proto3_optional, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FieldDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<VInt>::encode(&self.number, 24u32, buf)?;
        Format::<Enum>::encode(&self.label, 32u32, buf)?;
        Format::<Enum>::encode(&self.r#type, 40u32, buf)?;
        Format::<Bytes>::encode(&self.type_name, 50u32, buf)?;
        Format::<Bytes>::encode(&self.extendee, 18u32, buf)?;
        Format::<Bytes>::encode(&self.default_value, 58u32, buf)?;
        Format::<VInt>::encode(&self.oneof_index, 72u32, buf)?;
        Format::<Bytes>::encode(&self.json_name, 82u32, buf)?;
        Format::<Nest>::encode(&self.options, 66u32, buf)?;
        Format::<Fix>::encode(&self.proto3_optional, 136u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OneofDescriptorProto {
    pub name: Option<String>,
    pub options: Option<Box<OneofOptions>>,
    pub _unknown: (),
}
impl OneofDescriptorProto {
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
    pub fn r#with_options(mut self, it: OneofOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: OneofOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for OneofDescriptorProto {
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
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for OneofDescriptorProto {
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
        if self.options != <Option<Box<OneofOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for OneofDescriptorProto {
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
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for OneofDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.OneofDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Nest>::encode(&self.options, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumDescriptorProto {
    pub name: Option<String>,
    pub value: Vec<EnumValueDescriptorProto>,
    pub options: Option<Box<EnumOptions>>,
    pub reserved_range: Vec<EnumDescriptorProtoEnumReservedRange>,
    pub reserved_name: Vec<String>,
    pub _unknown: (),
}
impl EnumDescriptorProto {
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
    pub fn r#with_value(mut self, it: EnumValueDescriptorProto) -> Self {
        self.r#add_value(it);
        self
    }
    #[inline(always)]
    pub fn r#add_value(&mut self, it: EnumValueDescriptorProto) -> &mut Self {
        self.value.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: EnumOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: EnumOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_reserved_range(
        mut self,
        it: EnumDescriptorProtoEnumReservedRange,
    ) -> Self {
        self.r#add_reserved_range(it);
        self
    }
    #[inline(always)]
    pub fn r#add_reserved_range(
        &mut self,
        it: EnumDescriptorProtoEnumReservedRange,
    ) -> &mut Self {
        self.reserved_range.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_reserved_name(mut self, it: String) -> Self {
        self.r#add_reserved_name(it);
        self
    }
    #[inline(always)]
    pub fn r#add_reserved_name(&mut self, it: String) -> &mut Self {
        self.reserved_name.push(it);
        self
    }
}
impl textformat::Decodable for EnumDescriptorProto {
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
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("reserved_range") => {
                textformat::Field::merge(&mut self.reserved_range, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("reserved_name") => {
                textformat::Field::merge(&mut self.reserved_name, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumDescriptorProto {
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
        if self.value != <Vec<EnumValueDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("value ");
            textformat::Field::format(&self.value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<EnumOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.reserved_range
            != <Vec<EnumDescriptorProtoEnumReservedRange> as Default>::default()
        {
            out.indent(pad);
            out.push_str("reserved_range ");
            textformat::Field::format(&self.reserved_range, ctx, pad, out)?;
            out.push('\n');
        }
        if self.reserved_name != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("reserved_name: ");
            textformat::Field::format(&self.reserved_name, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumDescriptorProto {
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
                buf = Format::<Repeat::<Nest>>::decode(&mut self.value, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            34u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.reserved_range, buf)?;
            }
            42u32 => {
                buf = Format::<Repeat::<Bytes>>::decode(&mut self.reserved_name, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.value, 18u32, buf)?;
        Format::<Nest>::encode(&self.options, 26u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.reserved_range, 34u32, buf)?;
        Format::<Repeat::<Bytes>>::encode(&self.reserved_name, 42u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumDescriptorProtoEnumReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub _unknown: (),
}
impl EnumDescriptorProtoEnumReservedRange {
    #[inline(always)]
    pub fn r#with_start(mut self, it: i32) -> Self {
        self.r#set_start(it);
        self
    }
    #[inline(always)]
    pub fn r#set_start(&mut self, it: i32) -> &mut Self {
        self.start = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_end(mut self, it: i32) -> Self {
        self.r#set_end(it);
        self
    }
    #[inline(always)]
    pub fn r#set_end(&mut self, it: i32) -> &mut Self {
        self.end = it.into();
        self
    }
}
impl textformat::Decodable for EnumDescriptorProtoEnumReservedRange {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("start") => {
                textformat::Field::merge(&mut self.start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("end") => {
                textformat::Field::merge(&mut self.end, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumDescriptorProtoEnumReservedRange {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.start != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("start: ");
            textformat::Field::format(&self.start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.end != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("end: ");
            textformat::Field::format(&self.end, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumDescriptorProtoEnumReservedRange {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            10u32 => {
                buf = Format::<VInt>::decode(&mut self.start, buf)?;
            }
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumDescriptorProtoEnumReservedRange {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumDescriptorProto.EnumReservedRange"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<VInt>::encode(&self.start, 8u32, buf)?;
        Format::<VInt>::encode(&self.end, 16u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumValueDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub options: Option<Box<EnumValueOptions>>,
    pub _unknown: (),
}
impl EnumValueDescriptorProto {
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
    pub fn r#with_options(mut self, it: EnumValueOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: EnumValueOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for EnumValueDescriptorProto {
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
impl textformat::Encodable for EnumValueDescriptorProto {
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
        if self.number != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("number: ");
            textformat::Field::format(&self.number, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<EnumValueOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumValueDescriptorProto {
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
            16u32 => {
                buf = Format::<VInt>::decode(&mut self.number, buf)?;
            }
            18u32 => {
                buf = Format::<VInt>::decode(&mut self.number, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumValueDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumValueDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<VInt>::encode(&self.number, 16u32, buf)?;
        Format::<Nest>::encode(&self.options, 26u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceDescriptorProto {
    pub name: Option<String>,
    pub method: Vec<MethodDescriptorProto>,
    pub options: Option<Box<ServiceOptions>>,
    pub _unknown: (),
}
impl ServiceDescriptorProto {
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
    pub fn r#with_method(mut self, it: MethodDescriptorProto) -> Self {
        self.r#add_method(it);
        self
    }
    #[inline(always)]
    pub fn r#add_method(&mut self, it: MethodDescriptorProto) -> &mut Self {
        self.method.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: ServiceOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: ServiceOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for ServiceDescriptorProto {
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
            textformat::ast::FieldName::Normal("method") => {
                textformat::Field::merge(&mut self.method, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ServiceDescriptorProto {
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
        if self.method != <Vec<MethodDescriptorProto> as Default>::default() {
            out.indent(pad);
            out.push_str("method ");
            textformat::Field::format(&self.method, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<ServiceOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ServiceDescriptorProto {
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
                buf = Format::<Repeat::<Nest>>::decode(&mut self.method, buf)?;
            }
            26u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ServiceDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.ServiceDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.method, 18u32, buf)?;
        Format::<Nest>::encode(&self.options, 26u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodDescriptorProto {
    pub name: Option<String>,
    pub input_type: Option<String>,
    pub output_type: Option<String>,
    pub options: Option<Box<MethodOptions>>,
    pub client_streaming: Option<bool>,
    pub server_streaming: Option<bool>,
    pub _unknown: (),
}
impl MethodDescriptorProto {
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
    pub fn r#with_input_type(mut self, it: String) -> Self {
        self.r#set_input_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_input_type(&mut self, it: String) -> &mut Self {
        self.input_type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_output_type(mut self, it: String) -> Self {
        self.r#set_output_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_output_type(&mut self, it: String) -> &mut Self {
        self.output_type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_options(mut self, it: MethodOptions) -> Self {
        self.r#set_options(it);
        self
    }
    #[inline(always)]
    pub fn r#set_options(&mut self, it: MethodOptions) -> &mut Self {
        self.options = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_client_streaming(mut self, it: bool) -> Self {
        self.r#set_client_streaming(it);
        self
    }
    #[inline(always)]
    pub fn r#set_client_streaming(&mut self, it: bool) -> &mut Self {
        self.client_streaming = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_server_streaming(mut self, it: bool) -> Self {
        self.r#set_server_streaming(it);
        self
    }
    #[inline(always)]
    pub fn r#set_server_streaming(&mut self, it: bool) -> &mut Self {
        self.server_streaming = it.into();
        self
    }
}
impl textformat::Decodable for MethodDescriptorProto {
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
            textformat::ast::FieldName::Normal("input_type") => {
                textformat::Field::merge(&mut self.input_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("output_type") => {
                textformat::Field::merge(&mut self.output_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("options") => {
                textformat::Field::merge(&mut self.options, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("client_streaming") => {
                textformat::Field::merge(&mut self.client_streaming, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("server_streaming") => {
                textformat::Field::merge(&mut self.server_streaming, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for MethodDescriptorProto {
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
        if self.input_type != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("input_type: ");
            textformat::Field::format(&self.input_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.output_type != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("output_type: ");
            textformat::Field::format(&self.output_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.options != <Option<Box<MethodOptions>> as Default>::default() {
            out.indent(pad);
            out.push_str("options ");
            textformat::Field::format(&self.options, ctx, pad, out)?;
            out.push('\n');
        }
        if self.client_streaming != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("client_streaming: ");
            textformat::Field::format(&self.client_streaming, ctx, pad, out)?;
            out.push('\n');
        }
        if self.server_streaming != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("server_streaming: ");
            textformat::Field::format(&self.server_streaming, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for MethodDescriptorProto {
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
                buf = Format::<Bytes>::decode(&mut self.input_type, buf)?;
            }
            26u32 => {
                buf = Format::<Bytes>::decode(&mut self.output_type, buf)?;
            }
            34u32 => {
                buf = Format::<Nest>::decode(&mut self.options, buf)?;
            }
            40u32 => {
                buf = Format::<Fix>::decode(&mut self.client_streaming, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.client_streaming, buf)?;
            }
            48u32 => {
                buf = Format::<Fix>::decode(&mut self.server_streaming, buf)?;
            }
            50u32 => {
                buf = Format::<Fix>::decode(&mut self.server_streaming, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for MethodDescriptorProto {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.MethodDescriptorProto"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name, 10u32, buf)?;
        Format::<Bytes>::encode(&self.input_type, 18u32, buf)?;
        Format::<Bytes>::encode(&self.output_type, 26u32, buf)?;
        Format::<Nest>::encode(&self.options, 34u32, buf)?;
        Format::<Fix>::encode(&self.client_streaming, 40u32, buf)?;
        Format::<Fix>::encode(&self.server_streaming, 48u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileOptions {
    pub java_package: Option<String>,
    pub java_outer_classname: Option<String>,
    pub java_multiple_files: Option<bool>,
    pub java_generate_equals_and_hash: Option<bool>,
    pub java_string_check_utf8: Option<bool>,
    pub optimize_for: Option<FileOptionsOptimizeMode>,
    pub go_package: Option<String>,
    pub cc_generic_services: Option<bool>,
    pub java_generic_services: Option<bool>,
    pub py_generic_services: Option<bool>,
    pub php_generic_services: Option<bool>,
    pub deprecated: Option<bool>,
    pub cc_enable_arenas: Option<bool>,
    pub objc_class_prefix: Option<String>,
    pub csharp_namespace: Option<String>,
    pub swift_prefix: Option<String>,
    pub php_class_prefix: Option<String>,
    pub php_namespace: Option<String>,
    pub php_metadata_namespace: Option<String>,
    pub ruby_package: Option<String>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl FileOptions {
    #[inline(always)]
    pub fn r#with_java_package(mut self, it: String) -> Self {
        self.r#set_java_package(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_package(&mut self, it: String) -> &mut Self {
        self.java_package = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_java_outer_classname(mut self, it: String) -> Self {
        self.r#set_java_outer_classname(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_outer_classname(&mut self, it: String) -> &mut Self {
        self.java_outer_classname = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_java_multiple_files(mut self, it: bool) -> Self {
        self.r#set_java_multiple_files(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_multiple_files(&mut self, it: bool) -> &mut Self {
        self.java_multiple_files = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_java_generate_equals_and_hash(mut self, it: bool) -> Self {
        self.r#set_java_generate_equals_and_hash(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_generate_equals_and_hash(&mut self, it: bool) -> &mut Self {
        self.java_generate_equals_and_hash = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_java_string_check_utf8(mut self, it: bool) -> Self {
        self.r#set_java_string_check_utf8(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_string_check_utf8(&mut self, it: bool) -> &mut Self {
        self.java_string_check_utf8 = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_optimize_for(mut self, it: FileOptionsOptimizeMode) -> Self {
        self.r#set_optimize_for(it);
        self
    }
    #[inline(always)]
    pub fn r#set_optimize_for(&mut self, it: FileOptionsOptimizeMode) -> &mut Self {
        self.optimize_for = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_go_package(mut self, it: String) -> Self {
        self.r#set_go_package(it);
        self
    }
    #[inline(always)]
    pub fn r#set_go_package(&mut self, it: String) -> &mut Self {
        self.go_package = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_cc_generic_services(mut self, it: bool) -> Self {
        self.r#set_cc_generic_services(it);
        self
    }
    #[inline(always)]
    pub fn r#set_cc_generic_services(&mut self, it: bool) -> &mut Self {
        self.cc_generic_services = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_java_generic_services(mut self, it: bool) -> Self {
        self.r#set_java_generic_services(it);
        self
    }
    #[inline(always)]
    pub fn r#set_java_generic_services(&mut self, it: bool) -> &mut Self {
        self.java_generic_services = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_py_generic_services(mut self, it: bool) -> Self {
        self.r#set_py_generic_services(it);
        self
    }
    #[inline(always)]
    pub fn r#set_py_generic_services(&mut self, it: bool) -> &mut Self {
        self.py_generic_services = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_php_generic_services(mut self, it: bool) -> Self {
        self.r#set_php_generic_services(it);
        self
    }
    #[inline(always)]
    pub fn r#set_php_generic_services(&mut self, it: bool) -> &mut Self {
        self.php_generic_services = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_cc_enable_arenas(mut self, it: bool) -> Self {
        self.r#set_cc_enable_arenas(it);
        self
    }
    #[inline(always)]
    pub fn r#set_cc_enable_arenas(&mut self, it: bool) -> &mut Self {
        self.cc_enable_arenas = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_objc_class_prefix(mut self, it: String) -> Self {
        self.r#set_objc_class_prefix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_objc_class_prefix(&mut self, it: String) -> &mut Self {
        self.objc_class_prefix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_csharp_namespace(mut self, it: String) -> Self {
        self.r#set_csharp_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_csharp_namespace(&mut self, it: String) -> &mut Self {
        self.csharp_namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_swift_prefix(mut self, it: String) -> Self {
        self.r#set_swift_prefix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_swift_prefix(&mut self, it: String) -> &mut Self {
        self.swift_prefix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_php_class_prefix(mut self, it: String) -> Self {
        self.r#set_php_class_prefix(it);
        self
    }
    #[inline(always)]
    pub fn r#set_php_class_prefix(&mut self, it: String) -> &mut Self {
        self.php_class_prefix = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_php_namespace(mut self, it: String) -> Self {
        self.r#set_php_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_php_namespace(&mut self, it: String) -> &mut Self {
        self.php_namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_php_metadata_namespace(mut self, it: String) -> Self {
        self.r#set_php_metadata_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_php_metadata_namespace(&mut self, it: String) -> &mut Self {
        self.php_metadata_namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_ruby_package(mut self, it: String) -> Self {
        self.r#set_ruby_package(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ruby_package(&mut self, it: String) -> &mut Self {
        self.ruby_package = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for FileOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("java_package") => {
                textformat::Field::merge(&mut self.java_package, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("java_outer_classname") => {
                textformat::Field::merge(&mut self.java_outer_classname, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("java_multiple_files") => {
                textformat::Field::merge(&mut self.java_multiple_files, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("java_generate_equals_and_hash") => {
                textformat::Field::merge(
                    &mut self.java_generate_equals_and_hash,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("java_string_check_utf8") => {
                textformat::Field::merge(&mut self.java_string_check_utf8, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("optimize_for") => {
                textformat::Field::merge(&mut self.optimize_for, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("go_package") => {
                textformat::Field::merge(&mut self.go_package, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("cc_generic_services") => {
                textformat::Field::merge(&mut self.cc_generic_services, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("java_generic_services") => {
                textformat::Field::merge(&mut self.java_generic_services, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("py_generic_services") => {
                textformat::Field::merge(&mut self.py_generic_services, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("php_generic_services") => {
                textformat::Field::merge(&mut self.php_generic_services, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("cc_enable_arenas") => {
                textformat::Field::merge(&mut self.cc_enable_arenas, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("objc_class_prefix") => {
                textformat::Field::merge(&mut self.objc_class_prefix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("csharp_namespace") => {
                textformat::Field::merge(&mut self.csharp_namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("swift_prefix") => {
                textformat::Field::merge(&mut self.swift_prefix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("php_class_prefix") => {
                textformat::Field::merge(&mut self.php_class_prefix, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("php_namespace") => {
                textformat::Field::merge(&mut self.php_namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("php_metadata_namespace") => {
                textformat::Field::merge(&mut self.php_metadata_namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("ruby_package") => {
                textformat::Field::merge(&mut self.ruby_package, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FileOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.java_package != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("java_package: ");
            textformat::Field::format(&self.java_package, ctx, pad, out)?;
            out.push('\n');
        }
        if self.java_outer_classname != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("java_outer_classname: ");
            textformat::Field::format(&self.java_outer_classname, ctx, pad, out)?;
            out.push('\n');
        }
        if self.java_multiple_files != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("java_multiple_files: ");
            textformat::Field::format(&self.java_multiple_files, ctx, pad, out)?;
            out.push('\n');
        }
        if self.java_generate_equals_and_hash != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("java_generate_equals_and_hash: ");
            textformat::Field::format(
                &self.java_generate_equals_and_hash,
                ctx,
                pad,
                out,
            )?;
            out.push('\n');
        }
        if self.java_string_check_utf8 != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("java_string_check_utf8: ");
            textformat::Field::format(&self.java_string_check_utf8, ctx, pad, out)?;
            out.push('\n');
        }
        if self.optimize_for != <Option<FileOptionsOptimizeMode> as Default>::default() {
            out.indent(pad);
            out.push_str("optimize_for: ");
            textformat::Field::format(&self.optimize_for, ctx, pad, out)?;
            out.push('\n');
        }
        if self.go_package != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("go_package: ");
            textformat::Field::format(&self.go_package, ctx, pad, out)?;
            out.push('\n');
        }
        if self.cc_generic_services != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("cc_generic_services: ");
            textformat::Field::format(&self.cc_generic_services, ctx, pad, out)?;
            out.push('\n');
        }
        if self.java_generic_services != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("java_generic_services: ");
            textformat::Field::format(&self.java_generic_services, ctx, pad, out)?;
            out.push('\n');
        }
        if self.py_generic_services != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("py_generic_services: ");
            textformat::Field::format(&self.py_generic_services, ctx, pad, out)?;
            out.push('\n');
        }
        if self.php_generic_services != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("php_generic_services: ");
            textformat::Field::format(&self.php_generic_services, ctx, pad, out)?;
            out.push('\n');
        }
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.cc_enable_arenas != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("cc_enable_arenas: ");
            textformat::Field::format(&self.cc_enable_arenas, ctx, pad, out)?;
            out.push('\n');
        }
        if self.objc_class_prefix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("objc_class_prefix: ");
            textformat::Field::format(&self.objc_class_prefix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.csharp_namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("csharp_namespace: ");
            textformat::Field::format(&self.csharp_namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.swift_prefix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("swift_prefix: ");
            textformat::Field::format(&self.swift_prefix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.php_class_prefix != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("php_class_prefix: ");
            textformat::Field::format(&self.php_class_prefix, ctx, pad, out)?;
            out.push('\n');
        }
        if self.php_namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("php_namespace: ");
            textformat::Field::format(&self.php_namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.php_metadata_namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("php_metadata_namespace: ");
            textformat::Field::format(&self.php_metadata_namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ruby_package != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("ruby_package: ");
            textformat::Field::format(&self.ruby_package, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FileOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.java_package, buf)?;
            }
            66u32 => {
                buf = Format::<Bytes>::decode(&mut self.java_outer_classname, buf)?;
            }
            80u32 => {
                buf = Format::<Fix>::decode(&mut self.java_multiple_files, buf)?;
            }
            82u32 => {
                buf = Format::<Fix>::decode(&mut self.java_multiple_files, buf)?;
            }
            160u32 => {
                buf = Format::<
                    Fix,
                >::decode(&mut self.java_generate_equals_and_hash, buf)?;
            }
            162u32 => {
                buf = Format::<
                    Fix,
                >::decode(&mut self.java_generate_equals_and_hash, buf)?;
            }
            216u32 => {
                buf = Format::<Fix>::decode(&mut self.java_string_check_utf8, buf)?;
            }
            218u32 => {
                buf = Format::<Fix>::decode(&mut self.java_string_check_utf8, buf)?;
            }
            72u32 => {
                buf = Format::<Enum>::decode(&mut self.optimize_for, buf)?;
            }
            74u32 => {
                buf = Format::<Enum>::decode(&mut self.optimize_for, buf)?;
            }
            90u32 => {
                buf = Format::<Bytes>::decode(&mut self.go_package, buf)?;
            }
            128u32 => {
                buf = Format::<Fix>::decode(&mut self.cc_generic_services, buf)?;
            }
            130u32 => {
                buf = Format::<Fix>::decode(&mut self.cc_generic_services, buf)?;
            }
            136u32 => {
                buf = Format::<Fix>::decode(&mut self.java_generic_services, buf)?;
            }
            138u32 => {
                buf = Format::<Fix>::decode(&mut self.java_generic_services, buf)?;
            }
            144u32 => {
                buf = Format::<Fix>::decode(&mut self.py_generic_services, buf)?;
            }
            146u32 => {
                buf = Format::<Fix>::decode(&mut self.py_generic_services, buf)?;
            }
            336u32 => {
                buf = Format::<Fix>::decode(&mut self.php_generic_services, buf)?;
            }
            338u32 => {
                buf = Format::<Fix>::decode(&mut self.php_generic_services, buf)?;
            }
            184u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            186u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            248u32 => {
                buf = Format::<Fix>::decode(&mut self.cc_enable_arenas, buf)?;
            }
            250u32 => {
                buf = Format::<Fix>::decode(&mut self.cc_enable_arenas, buf)?;
            }
            290u32 => {
                buf = Format::<Bytes>::decode(&mut self.objc_class_prefix, buf)?;
            }
            298u32 => {
                buf = Format::<Bytes>::decode(&mut self.csharp_namespace, buf)?;
            }
            314u32 => {
                buf = Format::<Bytes>::decode(&mut self.swift_prefix, buf)?;
            }
            322u32 => {
                buf = Format::<Bytes>::decode(&mut self.php_class_prefix, buf)?;
            }
            330u32 => {
                buf = Format::<Bytes>::decode(&mut self.php_namespace, buf)?;
            }
            354u32 => {
                buf = Format::<Bytes>::decode(&mut self.php_metadata_namespace, buf)?;
            }
            362u32 => {
                buf = Format::<Bytes>::decode(&mut self.ruby_package, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FileOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FileOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.java_package, 10u32, buf)?;
        Format::<Bytes>::encode(&self.java_outer_classname, 66u32, buf)?;
        Format::<Fix>::encode(&self.java_multiple_files, 80u32, buf)?;
        Format::<Fix>::encode(&self.java_generate_equals_and_hash, 160u32, buf)?;
        Format::<Fix>::encode(&self.java_string_check_utf8, 216u32, buf)?;
        Format::<Enum>::encode(&self.optimize_for, 72u32, buf)?;
        Format::<Bytes>::encode(&self.go_package, 90u32, buf)?;
        Format::<Fix>::encode(&self.cc_generic_services, 128u32, buf)?;
        Format::<Fix>::encode(&self.java_generic_services, 136u32, buf)?;
        Format::<Fix>::encode(&self.py_generic_services, 144u32, buf)?;
        Format::<Fix>::encode(&self.php_generic_services, 336u32, buf)?;
        Format::<Fix>::encode(&self.deprecated, 184u32, buf)?;
        Format::<Fix>::encode(&self.cc_enable_arenas, 248u32, buf)?;
        Format::<Bytes>::encode(&self.objc_class_prefix, 290u32, buf)?;
        Format::<Bytes>::encode(&self.csharp_namespace, 298u32, buf)?;
        Format::<Bytes>::encode(&self.swift_prefix, 314u32, buf)?;
        Format::<Bytes>::encode(&self.php_class_prefix, 322u32, buf)?;
        Format::<Bytes>::encode(&self.php_namespace, 330u32, buf)?;
        Format::<Bytes>::encode(&self.php_metadata_namespace, 354u32, buf)?;
        Format::<Bytes>::encode(&self.ruby_package, 362u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageOptions {
    pub message_set_wire_format: Option<bool>,
    pub no_standard_descriptor_accessor: Option<bool>,
    pub deprecated: Option<bool>,
    pub map_entry: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub disabled: Option<bool>,
    pub ignored: Option<bool>,
    pub _unknown: (),
}
impl MessageOptions {
    #[inline(always)]
    pub fn r#with_message_set_wire_format(mut self, it: bool) -> Self {
        self.r#set_message_set_wire_format(it);
        self
    }
    #[inline(always)]
    pub fn r#set_message_set_wire_format(&mut self, it: bool) -> &mut Self {
        self.message_set_wire_format = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_no_standard_descriptor_accessor(mut self, it: bool) -> Self {
        self.r#set_no_standard_descriptor_accessor(it);
        self
    }
    #[inline(always)]
    pub fn r#set_no_standard_descriptor_accessor(&mut self, it: bool) -> &mut Self {
        self.no_standard_descriptor_accessor = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_map_entry(mut self, it: bool) -> Self {
        self.r#set_map_entry(it);
        self
    }
    #[inline(always)]
    pub fn r#set_map_entry(&mut self, it: bool) -> &mut Self {
        self.map_entry = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_disabled(mut self, it: bool) -> Self {
        self.r#set_disabled(it);
        self
    }
    #[inline(always)]
    pub fn r#set_disabled(&mut self, it: bool) -> &mut Self {
        self.disabled = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_ignored(mut self, it: bool) -> Self {
        self.r#set_ignored(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ignored(&mut self, it: bool) -> &mut Self {
        self.ignored = it.into();
        self
    }
}
impl textformat::Decodable for MessageOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("message_set_wire_format") => {
                textformat::Field::merge(&mut self.message_set_wire_format, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("no_standard_descriptor_accessor") => {
                textformat::Field::merge(
                    &mut self.no_standard_descriptor_accessor,
                    ctx,
                    value,
                )?;
            }
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("map_entry") => {
                textformat::Field::merge(&mut self.map_entry, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            textformat::ast::FieldName::Extended("validate.disabled") => {
                textformat::Field::merge(&mut self.disabled, ctx, value)?;
            }
            textformat::ast::FieldName::Extended("validate.ignored") => {
                textformat::Field::merge(&mut self.ignored, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for MessageOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.message_set_wire_format != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("message_set_wire_format: ");
            textformat::Field::format(&self.message_set_wire_format, ctx, pad, out)?;
            out.push('\n');
        }
        if self.no_standard_descriptor_accessor != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("no_standard_descriptor_accessor: ");
            textformat::Field::format(
                &self.no_standard_descriptor_accessor,
                ctx,
                pad,
                out,
            )?;
            out.push('\n');
        }
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.map_entry != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("map_entry: ");
            textformat::Field::format(&self.map_entry, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        if self.disabled != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("[validate.disabled]: ");
            textformat::Field::format(&self.disabled, ctx, pad, out)?;
            out.push('\n');
        }
        if self.ignored != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("[validate.ignored]: ");
            textformat::Field::format(&self.ignored, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for MessageOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.message_set_wire_format, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.message_set_wire_format, buf)?;
            }
            16u32 => {
                buf = Format::<
                    Fix,
                >::decode(&mut self.no_standard_descriptor_accessor, buf)?;
            }
            18u32 => {
                buf = Format::<
                    Fix,
                >::decode(&mut self.no_standard_descriptor_accessor, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            56u32 => {
                buf = Format::<Fix>::decode(&mut self.map_entry, buf)?;
            }
            58u32 => {
                buf = Format::<Fix>::decode(&mut self.map_entry, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            8568u32 => {
                buf = Format::<Fix>::decode(&mut self.disabled, buf)?;
            }
            8570u32 => {
                buf = Format::<Fix>::decode(&mut self.disabled, buf)?;
            }
            8576u32 => {
                buf = Format::<Fix>::decode(&mut self.ignored, buf)?;
            }
            8578u32 => {
                buf = Format::<Fix>::decode(&mut self.ignored, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for MessageOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.MessageOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.message_set_wire_format, 8u32, buf)?;
        Format::<Fix>::encode(&self.no_standard_descriptor_accessor, 16u32, buf)?;
        Format::<Fix>::encode(&self.deprecated, 24u32, buf)?;
        Format::<Fix>::encode(&self.map_entry, 56u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        Format::<Fix>::encode(&self.disabled, 8568u32, buf)?;
        Format::<Fix>::encode(&self.ignored, 8576u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldOptions {
    pub ctype: Option<FieldOptionsCType>,
    pub packed: Option<bool>,
    pub jstype: Option<FieldOptionsJSType>,
    pub lazy: Option<bool>,
    pub deprecated: Option<bool>,
    pub weak: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub rules: Option<Box<FieldRules>>,
    pub _unknown: (),
}
impl FieldOptions {
    #[inline(always)]
    pub fn r#with_ctype(mut self, it: FieldOptionsCType) -> Self {
        self.r#set_ctype(it);
        self
    }
    #[inline(always)]
    pub fn r#set_ctype(&mut self, it: FieldOptionsCType) -> &mut Self {
        self.ctype = it.into();
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
    pub fn r#with_jstype(mut self, it: FieldOptionsJSType) -> Self {
        self.r#set_jstype(it);
        self
    }
    #[inline(always)]
    pub fn r#set_jstype(&mut self, it: FieldOptionsJSType) -> &mut Self {
        self.jstype = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_lazy(mut self, it: bool) -> Self {
        self.r#set_lazy(it);
        self
    }
    #[inline(always)]
    pub fn r#set_lazy(&mut self, it: bool) -> &mut Self {
        self.lazy = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_weak(mut self, it: bool) -> Self {
        self.r#set_weak(it);
        self
    }
    #[inline(always)]
    pub fn r#set_weak(&mut self, it: bool) -> &mut Self {
        self.weak = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_rules(mut self, it: FieldRules) -> Self {
        self.r#set_rules(it);
        self
    }
    #[inline(always)]
    pub fn r#set_rules(&mut self, it: FieldRules) -> &mut Self {
        self.rules = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for FieldOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("ctype") => {
                textformat::Field::merge(&mut self.ctype, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("packed") => {
                textformat::Field::merge(&mut self.packed, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("jstype") => {
                textformat::Field::merge(&mut self.jstype, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("lazy") => {
                textformat::Field::merge(&mut self.lazy, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("weak") => {
                textformat::Field::merge(&mut self.weak, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            textformat::ast::FieldName::Extended("validate.rules") => {
                textformat::Field::merge(&mut self.rules, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FieldOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.ctype != <Option<FieldOptionsCType> as Default>::default() {
            out.indent(pad);
            out.push_str("ctype: ");
            textformat::Field::format(&self.ctype, ctx, pad, out)?;
            out.push('\n');
        }
        if self.packed != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("packed: ");
            textformat::Field::format(&self.packed, ctx, pad, out)?;
            out.push('\n');
        }
        if self.jstype != <Option<FieldOptionsJSType> as Default>::default() {
            out.indent(pad);
            out.push_str("jstype: ");
            textformat::Field::format(&self.jstype, ctx, pad, out)?;
            out.push('\n');
        }
        if self.lazy != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("lazy: ");
            textformat::Field::format(&self.lazy, ctx, pad, out)?;
            out.push('\n');
        }
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.weak != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("weak: ");
            textformat::Field::format(&self.weak, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        if self.rules != <Option<Box<FieldRules>> as Default>::default() {
            out.indent(pad);
            out.push_str("[validate.rules] ");
            textformat::Field::format(&self.rules, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Enum>::decode(&mut self.ctype, buf)?;
            }
            10u32 => {
                buf = Format::<Enum>::decode(&mut self.ctype, buf)?;
            }
            16u32 => {
                buf = Format::<Fix>::decode(&mut self.packed, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.packed, buf)?;
            }
            48u32 => {
                buf = Format::<Enum>::decode(&mut self.jstype, buf)?;
            }
            50u32 => {
                buf = Format::<Enum>::decode(&mut self.jstype, buf)?;
            }
            40u32 => {
                buf = Format::<Fix>::decode(&mut self.lazy, buf)?;
            }
            42u32 => {
                buf = Format::<Fix>::decode(&mut self.lazy, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            80u32 => {
                buf = Format::<Fix>::decode(&mut self.weak, buf)?;
            }
            82u32 => {
                buf = Format::<Fix>::decode(&mut self.weak, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            8570u32 => {
                buf = Format::<Nest>::decode(&mut self.rules, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.FieldOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Enum>::encode(&self.ctype, 8u32, buf)?;
        Format::<Fix>::encode(&self.packed, 16u32, buf)?;
        Format::<Enum>::encode(&self.jstype, 48u32, buf)?;
        Format::<Fix>::encode(&self.lazy, 40u32, buf)?;
        Format::<Fix>::encode(&self.deprecated, 24u32, buf)?;
        Format::<Fix>::encode(&self.weak, 80u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        Format::<Nest>::encode(&self.rules, 8570u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OneofOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub required: Option<bool>,
    pub _unknown: (),
}
impl OneofOptions {
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
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
impl textformat::Decodable for OneofOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            textformat::ast::FieldName::Extended("validate.required") => {
                textformat::Field::merge(&mut self.required, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for OneofOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        if self.required != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("[validate.required]: ");
            textformat::Field::format(&self.required, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for OneofOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            8568u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            8570u32 => {
                buf = Format::<Fix>::decode(&mut self.required, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for OneofOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.OneofOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        Format::<Fix>::encode(&self.required, 8568u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumOptions {
    pub allow_alias: Option<bool>,
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl EnumOptions {
    #[inline(always)]
    pub fn r#with_allow_alias(mut self, it: bool) -> Self {
        self.r#set_allow_alias(it);
        self
    }
    #[inline(always)]
    pub fn r#set_allow_alias(&mut self, it: bool) -> &mut Self {
        self.allow_alias = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for EnumOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("allow_alias") => {
                textformat::Field::merge(&mut self.allow_alias, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.allow_alias != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("allow_alias: ");
            textformat::Field::format(&self.allow_alias, ctx, pad, out)?;
            out.push('\n');
        }
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            16u32 => {
                buf = Format::<Fix>::decode(&mut self.allow_alias, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.allow_alias, buf)?;
            }
            24u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            26u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.allow_alias, 16u32, buf)?;
        Format::<Fix>::encode(&self.deprecated, 24u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumValueOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl EnumValueOptions {
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for EnumValueOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for EnumValueOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for EnumValueOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            10u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for EnumValueOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.EnumValueOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.deprecated, 8u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl ServiceOptions {
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for ServiceOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ServiceOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ServiceOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            264u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            266u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ServiceOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.ServiceOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.deprecated, 264u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodOptions {
    pub deprecated: Option<bool>,
    pub idempotency_level: Option<MethodOptionsIdempotencyLevel>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
    pub _unknown: (),
}
impl MethodOptions {
    #[inline(always)]
    pub fn r#with_deprecated(mut self, it: bool) -> Self {
        self.r#set_deprecated(it);
        self
    }
    #[inline(always)]
    pub fn r#set_deprecated(&mut self, it: bool) -> &mut Self {
        self.deprecated = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_idempotency_level(
        mut self,
        it: MethodOptionsIdempotencyLevel,
    ) -> Self {
        self.r#set_idempotency_level(it);
        self
    }
    #[inline(always)]
    pub fn r#set_idempotency_level(
        &mut self,
        it: MethodOptionsIdempotencyLevel,
    ) -> &mut Self {
        self.idempotency_level = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_uninterpreted_option(mut self, it: UninterpretedOption) -> Self {
        self.r#add_uninterpreted_option(it);
        self
    }
    #[inline(always)]
    pub fn r#add_uninterpreted_option(&mut self, it: UninterpretedOption) -> &mut Self {
        self.uninterpreted_option.push(it);
        self
    }
}
impl textformat::Decodable for MethodOptions {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("deprecated") => {
                textformat::Field::merge(&mut self.deprecated, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("idempotency_level") => {
                textformat::Field::merge(&mut self.idempotency_level, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("uninterpreted_option") => {
                textformat::Field::merge(&mut self.uninterpreted_option, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for MethodOptions {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.deprecated != <Option<bool> as Default>::default() {
            out.indent(pad);
            out.push_str("deprecated: ");
            textformat::Field::format(&self.deprecated, ctx, pad, out)?;
            out.push('\n');
        }
        if self.idempotency_level
            != <Option<MethodOptionsIdempotencyLevel> as Default>::default()
        {
            out.indent(pad);
            out.push_str("idempotency_level: ");
            textformat::Field::format(&self.idempotency_level, ctx, pad, out)?;
            out.push('\n');
        }
        if self.uninterpreted_option != <Vec<UninterpretedOption> as Default>::default()
        {
            out.indent(pad);
            out.push_str("uninterpreted_option ");
            textformat::Field::format(&self.uninterpreted_option, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for MethodOptions {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            264u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            266u32 => {
                buf = Format::<Fix>::decode(&mut self.deprecated, buf)?;
            }
            272u32 => {
                buf = Format::<Enum>::decode(&mut self.idempotency_level, buf)?;
            }
            274u32 => {
                buf = Format::<Enum>::decode(&mut self.idempotency_level, buf)?;
            }
            7994u32 => {
                buf = Format::<
                    Repeat::<Nest>,
                >::decode(&mut self.uninterpreted_option, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for MethodOptions {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.MethodOptions"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Fix>::encode(&self.deprecated, 264u32, buf)?;
        Format::<Enum>::encode(&self.idempotency_level, 272u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.uninterpreted_option, 7994u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UninterpretedOption {
    pub name: Vec<UninterpretedOptionNamePart>,
    pub identifier_value: Option<String>,
    pub positive_int_value: Option<u64>,
    pub negative_int_value: Option<i64>,
    pub double_value: Option<f64>,
    pub string_value: Option<Vec<u8>>,
    pub aggregate_value: Option<String>,
    pub _unknown: (),
}
impl UninterpretedOption {
    #[inline(always)]
    pub fn r#with_name(mut self, it: UninterpretedOptionNamePart) -> Self {
        self.r#add_name(it);
        self
    }
    #[inline(always)]
    pub fn r#add_name(&mut self, it: UninterpretedOptionNamePart) -> &mut Self {
        self.name.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_identifier_value(mut self, it: String) -> Self {
        self.r#set_identifier_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_identifier_value(&mut self, it: String) -> &mut Self {
        self.identifier_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_positive_int_value(mut self, it: u64) -> Self {
        self.r#set_positive_int_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_positive_int_value(&mut self, it: u64) -> &mut Self {
        self.positive_int_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_negative_int_value(mut self, it: i64) -> Self {
        self.r#set_negative_int_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_negative_int_value(&mut self, it: i64) -> &mut Self {
        self.negative_int_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_double_value(mut self, it: f64) -> Self {
        self.r#set_double_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_double_value(&mut self, it: f64) -> &mut Self {
        self.double_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_string_value(mut self, it: Vec<u8>) -> Self {
        self.r#set_string_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_string_value(&mut self, it: Vec<u8>) -> &mut Self {
        self.string_value = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_aggregate_value(mut self, it: String) -> Self {
        self.r#set_aggregate_value(it);
        self
    }
    #[inline(always)]
    pub fn r#set_aggregate_value(&mut self, it: String) -> &mut Self {
        self.aggregate_value = it.into();
        self
    }
}
impl textformat::Decodable for UninterpretedOption {
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
            textformat::ast::FieldName::Normal("identifier_value") => {
                textformat::Field::merge(&mut self.identifier_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("positive_int_value") => {
                textformat::Field::merge(&mut self.positive_int_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("negative_int_value") => {
                textformat::Field::merge(&mut self.negative_int_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("double_value") => {
                textformat::Field::merge(&mut self.double_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("string_value") => {
                textformat::Field::merge(&mut self.string_value, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("aggregate_value") => {
                textformat::Field::merge(&mut self.aggregate_value, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UninterpretedOption {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name != <Vec<UninterpretedOptionNamePart> as Default>::default() {
            out.indent(pad);
            out.push_str("name ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.identifier_value != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("identifier_value: ");
            textformat::Field::format(&self.identifier_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.positive_int_value != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("positive_int_value: ");
            textformat::Field::format(&self.positive_int_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.negative_int_value != <Option<i64> as Default>::default() {
            out.indent(pad);
            out.push_str("negative_int_value: ");
            textformat::Field::format(&self.negative_int_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.double_value != <Option<f64> as Default>::default() {
            out.indent(pad);
            out.push_str("double_value: ");
            textformat::Field::format(&self.double_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.string_value != <Option<Vec<u8>> as Default>::default() {
            out.indent(pad);
            out.push_str("string_value: ");
            textformat::Field::format(&self.string_value, ctx, pad, out)?;
            out.push('\n');
        }
        if self.aggregate_value != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("aggregate_value: ");
            textformat::Field::format(&self.aggregate_value, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UninterpretedOption {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            18u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.name, buf)?;
            }
            26u32 => {
                buf = Format::<Bytes>::decode(&mut self.identifier_value, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.positive_int_value, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.positive_int_value, buf)?;
            }
            40u32 => {
                buf = Format::<VInt>::decode(&mut self.negative_int_value, buf)?;
            }
            42u32 => {
                buf = Format::<VInt>::decode(&mut self.negative_int_value, buf)?;
            }
            49u32 => {
                buf = Format::<Fix>::decode(&mut self.double_value, buf)?;
            }
            50u32 => {
                buf = Format::<Fix>::decode(&mut self.double_value, buf)?;
            }
            58u32 => {
                buf = Format::<Bytes>::decode(&mut self.string_value, buf)?;
            }
            66u32 => {
                buf = Format::<Bytes>::decode(&mut self.aggregate_value, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UninterpretedOption {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.UninterpretedOption"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.name, 18u32, buf)?;
        Format::<Bytes>::encode(&self.identifier_value, 26u32, buf)?;
        Format::<VInt>::encode(&self.positive_int_value, 32u32, buf)?;
        Format::<VInt>::encode(&self.negative_int_value, 40u32, buf)?;
        Format::<Fix>::encode(&self.double_value, 49u32, buf)?;
        Format::<Bytes>::encode(&self.string_value, 58u32, buf)?;
        Format::<Bytes>::encode(&self.aggregate_value, 66u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UninterpretedOptionNamePart {
    pub name_part: String,
    pub is_extension: bool,
    pub _unknown: (),
}
impl UninterpretedOptionNamePart {
    #[inline(always)]
    pub fn r#with_name_part(mut self, it: String) -> Self {
        self.r#set_name_part(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name_part(&mut self, it: String) -> &mut Self {
        self.name_part = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_is_extension(mut self, it: bool) -> Self {
        self.r#set_is_extension(it);
        self
    }
    #[inline(always)]
    pub fn r#set_is_extension(&mut self, it: bool) -> &mut Self {
        self.is_extension = it.into();
        self
    }
}
impl textformat::Decodable for UninterpretedOptionNamePart {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("name_part") => {
                textformat::Field::merge(&mut self.name_part, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("is_extension") => {
                textformat::Field::merge(&mut self.is_extension, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for UninterpretedOptionNamePart {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.name_part != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name_part: ");
            textformat::Field::format(&self.name_part, ctx, pad, out)?;
            out.push('\n');
        }
        if self.is_extension != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("is_extension: ");
            textformat::Field::format(&self.is_extension, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for UninterpretedOptionNamePart {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.name_part, buf)?;
            }
            16u32 => {
                buf = Format::<Fix>::decode(&mut self.is_extension, buf)?;
            }
            18u32 => {
                buf = Format::<Fix>::decode(&mut self.is_extension, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for UninterpretedOptionNamePart {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.UninterpretedOption.NamePart"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.name_part, 10u32, buf)?;
        Format::<Fix>::encode(&self.is_extension, 16u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SourceCodeInfo {
    pub location: Vec<SourceCodeInfoLocation>,
    pub _unknown: (),
}
impl SourceCodeInfo {
    #[inline(always)]
    pub fn r#with_location(mut self, it: SourceCodeInfoLocation) -> Self {
        self.r#add_location(it);
        self
    }
    #[inline(always)]
    pub fn r#add_location(&mut self, it: SourceCodeInfoLocation) -> &mut Self {
        self.location.push(it);
        self
    }
}
impl textformat::Decodable for SourceCodeInfo {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("location") => {
                textformat::Field::merge(&mut self.location, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SourceCodeInfo {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.location != <Vec<SourceCodeInfoLocation> as Default>::default() {
            out.indent(pad);
            out.push_str("location ");
            textformat::Field::format(&self.location, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SourceCodeInfo {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.location, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SourceCodeInfo {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.SourceCodeInfo"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.location, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SourceCodeInfoLocation {
    pub path: Vec<i32>,
    pub span: Vec<i32>,
    pub leading_comments: Option<String>,
    pub trailing_comments: Option<String>,
    pub leading_detached_comments: Vec<String>,
    pub _unknown: (),
}
impl SourceCodeInfoLocation {
    #[inline(always)]
    pub fn r#with_path(mut self, it: i32) -> Self {
        self.r#add_path(it);
        self
    }
    #[inline(always)]
    pub fn r#add_path(&mut self, it: i32) -> &mut Self {
        self.path.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_span(mut self, it: i32) -> Self {
        self.r#add_span(it);
        self
    }
    #[inline(always)]
    pub fn r#add_span(&mut self, it: i32) -> &mut Self {
        self.span.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_leading_comments(mut self, it: String) -> Self {
        self.r#set_leading_comments(it);
        self
    }
    #[inline(always)]
    pub fn r#set_leading_comments(&mut self, it: String) -> &mut Self {
        self.leading_comments = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_trailing_comments(mut self, it: String) -> Self {
        self.r#set_trailing_comments(it);
        self
    }
    #[inline(always)]
    pub fn r#set_trailing_comments(&mut self, it: String) -> &mut Self {
        self.trailing_comments = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_leading_detached_comments(mut self, it: String) -> Self {
        self.r#add_leading_detached_comments(it);
        self
    }
    #[inline(always)]
    pub fn r#add_leading_detached_comments(&mut self, it: String) -> &mut Self {
        self.leading_detached_comments.push(it);
        self
    }
}
impl textformat::Decodable for SourceCodeInfoLocation {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("path") => {
                textformat::Field::merge(&mut self.path, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("span") => {
                textformat::Field::merge(&mut self.span, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("leading_comments") => {
                textformat::Field::merge(&mut self.leading_comments, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("trailing_comments") => {
                textformat::Field::merge(&mut self.trailing_comments, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("leading_detached_comments") => {
                textformat::Field::merge(
                    &mut self.leading_detached_comments,
                    ctx,
                    value,
                )?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for SourceCodeInfoLocation {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.path != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("path: ");
            textformat::Field::format(&self.path, ctx, pad, out)?;
            out.push('\n');
        }
        if self.span != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("span: ");
            textformat::Field::format(&self.span, ctx, pad, out)?;
            out.push('\n');
        }
        if self.leading_comments != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("leading_comments: ");
            textformat::Field::format(&self.leading_comments, ctx, pad, out)?;
            out.push('\n');
        }
        if self.trailing_comments != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("trailing_comments: ");
            textformat::Field::format(&self.trailing_comments, ctx, pad, out)?;
            out.push('\n');
        }
        if self.leading_detached_comments != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("leading_detached_comments: ");
            textformat::Field::format(&self.leading_detached_comments, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for SourceCodeInfoLocation {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.path, buf)?;
            }
            10u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.path, buf)?;
            }
            16u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.span, buf)?;
            }
            18u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.span, buf)?;
            }
            26u32 => {
                buf = Format::<Bytes>::decode(&mut self.leading_comments, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.trailing_comments, buf)?;
            }
            50u32 => {
                buf = Format::<
                    Repeat::<Bytes>,
                >::decode(&mut self.leading_detached_comments, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for SourceCodeInfoLocation {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.SourceCodeInfo.Location"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<VInt>>::encode(&self.path, 8u32, buf)?;
        Format::<Repeat::<VInt>>::encode(&self.span, 16u32, buf)?;
        Format::<Bytes>::encode(&self.leading_comments, 26u32, buf)?;
        Format::<Bytes>::encode(&self.trailing_comments, 34u32, buf)?;
        Format::<Repeat::<Bytes>>::encode(&self.leading_detached_comments, 50u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GeneratedCodeInfo {
    pub annotation: Vec<GeneratedCodeInfoAnnotation>,
    pub _unknown: (),
}
impl GeneratedCodeInfo {
    #[inline(always)]
    pub fn r#with_annotation(mut self, it: GeneratedCodeInfoAnnotation) -> Self {
        self.r#add_annotation(it);
        self
    }
    #[inline(always)]
    pub fn r#add_annotation(&mut self, it: GeneratedCodeInfoAnnotation) -> &mut Self {
        self.annotation.push(it);
        self
    }
}
impl textformat::Decodable for GeneratedCodeInfo {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("annotation") => {
                textformat::Field::merge(&mut self.annotation, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for GeneratedCodeInfo {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.annotation != <Vec<GeneratedCodeInfoAnnotation> as Default>::default() {
            out.indent(pad);
            out.push_str("annotation ");
            textformat::Field::format(&self.annotation, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for GeneratedCodeInfo {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.annotation, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for GeneratedCodeInfo {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.GeneratedCodeInfo"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<Nest>>::encode(&self.annotation, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GeneratedCodeInfoAnnotation {
    pub path: Vec<i32>,
    pub source_file: Option<String>,
    pub begin: Option<i32>,
    pub end: Option<i32>,
    pub _unknown: (),
}
impl GeneratedCodeInfoAnnotation {
    #[inline(always)]
    pub fn r#with_path(mut self, it: i32) -> Self {
        self.r#add_path(it);
        self
    }
    #[inline(always)]
    pub fn r#add_path(&mut self, it: i32) -> &mut Self {
        self.path.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_source_file(mut self, it: String) -> Self {
        self.r#set_source_file(it);
        self
    }
    #[inline(always)]
    pub fn r#set_source_file(&mut self, it: String) -> &mut Self {
        self.source_file = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_begin(mut self, it: i32) -> Self {
        self.r#set_begin(it);
        self
    }
    #[inline(always)]
    pub fn r#set_begin(&mut self, it: i32) -> &mut Self {
        self.begin = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_end(mut self, it: i32) -> Self {
        self.r#set_end(it);
        self
    }
    #[inline(always)]
    pub fn r#set_end(&mut self, it: i32) -> &mut Self {
        self.end = it.into();
        self
    }
}
impl textformat::Decodable for GeneratedCodeInfoAnnotation {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("path") => {
                textformat::Field::merge(&mut self.path, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("source_file") => {
                textformat::Field::merge(&mut self.source_file, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("begin") => {
                textformat::Field::merge(&mut self.begin, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("end") => {
                textformat::Field::merge(&mut self.end, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for GeneratedCodeInfoAnnotation {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.path != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("path: ");
            textformat::Field::format(&self.path, ctx, pad, out)?;
            out.push('\n');
        }
        if self.source_file != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("source_file: ");
            textformat::Field::format(&self.source_file, ctx, pad, out)?;
            out.push('\n');
        }
        if self.begin != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("begin: ");
            textformat::Field::format(&self.begin, ctx, pad, out)?;
            out.push('\n');
        }
        if self.end != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("end: ");
            textformat::Field::format(&self.end, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for GeneratedCodeInfoAnnotation {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.path, buf)?;
            }
            10u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.path, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.source_file, buf)?;
            }
            24u32 => {
                buf = Format::<VInt>::decode(&mut self.begin, buf)?;
            }
            26u32 => {
                buf = Format::<VInt>::decode(&mut self.begin, buf)?;
            }
            32u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            34u32 => {
                buf = Format::<VInt>::decode(&mut self.end, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for GeneratedCodeInfoAnnotation {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.GeneratedCodeInfo.Annotation"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Repeat::<VInt>>::encode(&self.path, 8u32, buf)?;
        Format::<Bytes>::encode(&self.source_file, 18u32, buf)?;
        Format::<VInt>::encode(&self.begin, 24u32, buf)?;
        Format::<VInt>::encode(&self.end, 32u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldDescriptorProtoType {
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
impl Default for FieldDescriptorProtoType {
    fn default() -> FieldDescriptorProtoType {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldDescriptorProtoType {}
impl From<u32> for FieldDescriptorProtoType {
    fn from(v: u32) -> FieldDescriptorProtoType {
        match v {
            1u32 => FieldDescriptorProtoType::TYPE_DOUBLE,
            2u32 => FieldDescriptorProtoType::TYPE_FLOAT,
            3u32 => FieldDescriptorProtoType::TYPE_INT64,
            4u32 => FieldDescriptorProtoType::TYPE_UINT64,
            5u32 => FieldDescriptorProtoType::TYPE_INT32,
            6u32 => FieldDescriptorProtoType::TYPE_FIXED64,
            7u32 => FieldDescriptorProtoType::TYPE_FIXED32,
            8u32 => FieldDescriptorProtoType::TYPE_BOOL,
            9u32 => FieldDescriptorProtoType::TYPE_STRING,
            10u32 => FieldDescriptorProtoType::TYPE_GROUP,
            11u32 => FieldDescriptorProtoType::TYPE_MESSAGE,
            12u32 => FieldDescriptorProtoType::TYPE_BYTES,
            13u32 => FieldDescriptorProtoType::TYPE_UINT32,
            14u32 => FieldDescriptorProtoType::TYPE_ENUM,
            15u32 => FieldDescriptorProtoType::TYPE_SFIXED32,
            16u32 => FieldDescriptorProtoType::TYPE_SFIXED64,
            17u32 => FieldDescriptorProtoType::TYPE_SINT32,
            18u32 => FieldDescriptorProtoType::TYPE_SINT64,
            other => FieldDescriptorProtoType::Unknown(other),
        }
    }
}
impl From<FieldDescriptorProtoType> for u32 {
    fn from(v: FieldDescriptorProtoType) -> u32 {
        match v {
            FieldDescriptorProtoType::TYPE_DOUBLE => 1u32,
            FieldDescriptorProtoType::TYPE_FLOAT => 2u32,
            FieldDescriptorProtoType::TYPE_INT64 => 3u32,
            FieldDescriptorProtoType::TYPE_UINT64 => 4u32,
            FieldDescriptorProtoType::TYPE_INT32 => 5u32,
            FieldDescriptorProtoType::TYPE_FIXED64 => 6u32,
            FieldDescriptorProtoType::TYPE_FIXED32 => 7u32,
            FieldDescriptorProtoType::TYPE_BOOL => 8u32,
            FieldDescriptorProtoType::TYPE_STRING => 9u32,
            FieldDescriptorProtoType::TYPE_GROUP => 10u32,
            FieldDescriptorProtoType::TYPE_MESSAGE => 11u32,
            FieldDescriptorProtoType::TYPE_BYTES => 12u32,
            FieldDescriptorProtoType::TYPE_UINT32 => 13u32,
            FieldDescriptorProtoType::TYPE_ENUM => 14u32,
            FieldDescriptorProtoType::TYPE_SFIXED32 => 15u32,
            FieldDescriptorProtoType::TYPE_SFIXED64 => 16u32,
            FieldDescriptorProtoType::TYPE_SINT32 => 17u32,
            FieldDescriptorProtoType::TYPE_SINT64 => 18u32,
            FieldDescriptorProtoType::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldDescriptorProtoType {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldDescriptorProtoType::TYPE_DOUBLE => "TYPE_DOUBLE",
            FieldDescriptorProtoType::TYPE_FLOAT => "TYPE_FLOAT",
            FieldDescriptorProtoType::TYPE_INT64 => "TYPE_INT64",
            FieldDescriptorProtoType::TYPE_UINT64 => "TYPE_UINT64",
            FieldDescriptorProtoType::TYPE_INT32 => "TYPE_INT32",
            FieldDescriptorProtoType::TYPE_FIXED64 => "TYPE_FIXED64",
            FieldDescriptorProtoType::TYPE_FIXED32 => "TYPE_FIXED32",
            FieldDescriptorProtoType::TYPE_BOOL => "TYPE_BOOL",
            FieldDescriptorProtoType::TYPE_STRING => "TYPE_STRING",
            FieldDescriptorProtoType::TYPE_GROUP => "TYPE_GROUP",
            FieldDescriptorProtoType::TYPE_MESSAGE => "TYPE_MESSAGE",
            FieldDescriptorProtoType::TYPE_BYTES => "TYPE_BYTES",
            FieldDescriptorProtoType::TYPE_UINT32 => "TYPE_UINT32",
            FieldDescriptorProtoType::TYPE_ENUM => "TYPE_ENUM",
            FieldDescriptorProtoType::TYPE_SFIXED32 => "TYPE_SFIXED32",
            FieldDescriptorProtoType::TYPE_SFIXED64 => "TYPE_SFIXED64",
            FieldDescriptorProtoType::TYPE_SINT32 => "TYPE_SINT32",
            FieldDescriptorProtoType::TYPE_SINT64 => "TYPE_SINT64",
            FieldDescriptorProtoType::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("TYPE_DOUBLE") => {
                *self = FieldDescriptorProtoType::TYPE_DOUBLE;
            }
            textformat::ast::Literal::Identifier("TYPE_FLOAT") => {
                *self = FieldDescriptorProtoType::TYPE_FLOAT;
            }
            textformat::ast::Literal::Identifier("TYPE_INT64") => {
                *self = FieldDescriptorProtoType::TYPE_INT64;
            }
            textformat::ast::Literal::Identifier("TYPE_UINT64") => {
                *self = FieldDescriptorProtoType::TYPE_UINT64;
            }
            textformat::ast::Literal::Identifier("TYPE_INT32") => {
                *self = FieldDescriptorProtoType::TYPE_INT32;
            }
            textformat::ast::Literal::Identifier("TYPE_FIXED64") => {
                *self = FieldDescriptorProtoType::TYPE_FIXED64;
            }
            textformat::ast::Literal::Identifier("TYPE_FIXED32") => {
                *self = FieldDescriptorProtoType::TYPE_FIXED32;
            }
            textformat::ast::Literal::Identifier("TYPE_BOOL") => {
                *self = FieldDescriptorProtoType::TYPE_BOOL;
            }
            textformat::ast::Literal::Identifier("TYPE_STRING") => {
                *self = FieldDescriptorProtoType::TYPE_STRING;
            }
            textformat::ast::Literal::Identifier("TYPE_GROUP") => {
                *self = FieldDescriptorProtoType::TYPE_GROUP;
            }
            textformat::ast::Literal::Identifier("TYPE_MESSAGE") => {
                *self = FieldDescriptorProtoType::TYPE_MESSAGE;
            }
            textformat::ast::Literal::Identifier("TYPE_BYTES") => {
                *self = FieldDescriptorProtoType::TYPE_BYTES;
            }
            textformat::ast::Literal::Identifier("TYPE_UINT32") => {
                *self = FieldDescriptorProtoType::TYPE_UINT32;
            }
            textformat::ast::Literal::Identifier("TYPE_ENUM") => {
                *self = FieldDescriptorProtoType::TYPE_ENUM;
            }
            textformat::ast::Literal::Identifier("TYPE_SFIXED32") => {
                *self = FieldDescriptorProtoType::TYPE_SFIXED32;
            }
            textformat::ast::Literal::Identifier("TYPE_SFIXED64") => {
                *self = FieldDescriptorProtoType::TYPE_SFIXED64;
            }
            textformat::ast::Literal::Identifier("TYPE_SINT32") => {
                *self = FieldDescriptorProtoType::TYPE_SINT32;
            }
            textformat::ast::Literal::Identifier("TYPE_SINT64") => {
                *self = FieldDescriptorProtoType::TYPE_SINT64;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldDescriptorProtoLabel {
    LABEL_OPTIONAL,
    LABEL_REQUIRED,
    LABEL_REPEATED,
    Unknown(u32),
}
impl Default for FieldDescriptorProtoLabel {
    fn default() -> FieldDescriptorProtoLabel {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldDescriptorProtoLabel {}
impl From<u32> for FieldDescriptorProtoLabel {
    fn from(v: u32) -> FieldDescriptorProtoLabel {
        match v {
            1u32 => FieldDescriptorProtoLabel::LABEL_OPTIONAL,
            2u32 => FieldDescriptorProtoLabel::LABEL_REQUIRED,
            3u32 => FieldDescriptorProtoLabel::LABEL_REPEATED,
            other => FieldDescriptorProtoLabel::Unknown(other),
        }
    }
}
impl From<FieldDescriptorProtoLabel> for u32 {
    fn from(v: FieldDescriptorProtoLabel) -> u32 {
        match v {
            FieldDescriptorProtoLabel::LABEL_OPTIONAL => 1u32,
            FieldDescriptorProtoLabel::LABEL_REQUIRED => 2u32,
            FieldDescriptorProtoLabel::LABEL_REPEATED => 3u32,
            FieldDescriptorProtoLabel::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldDescriptorProtoLabel {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldDescriptorProtoLabel::LABEL_OPTIONAL => "LABEL_OPTIONAL",
            FieldDescriptorProtoLabel::LABEL_REQUIRED => "LABEL_REQUIRED",
            FieldDescriptorProtoLabel::LABEL_REPEATED => "LABEL_REPEATED",
            FieldDescriptorProtoLabel::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("LABEL_OPTIONAL") => {
                *self = FieldDescriptorProtoLabel::LABEL_OPTIONAL;
            }
            textformat::ast::Literal::Identifier("LABEL_REQUIRED") => {
                *self = FieldDescriptorProtoLabel::LABEL_REQUIRED;
            }
            textformat::ast::Literal::Identifier("LABEL_REPEATED") => {
                *self = FieldDescriptorProtoLabel::LABEL_REPEATED;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FileOptionsOptimizeMode {
    SPEED,
    CODE_SIZE,
    LITE_RUNTIME,
    Unknown(u32),
}
impl Default for FileOptionsOptimizeMode {
    fn default() -> FileOptionsOptimizeMode {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FileOptionsOptimizeMode {}
impl From<u32> for FileOptionsOptimizeMode {
    fn from(v: u32) -> FileOptionsOptimizeMode {
        match v {
            1u32 => FileOptionsOptimizeMode::SPEED,
            2u32 => FileOptionsOptimizeMode::CODE_SIZE,
            3u32 => FileOptionsOptimizeMode::LITE_RUNTIME,
            other => FileOptionsOptimizeMode::Unknown(other),
        }
    }
}
impl From<FileOptionsOptimizeMode> for u32 {
    fn from(v: FileOptionsOptimizeMode) -> u32 {
        match v {
            FileOptionsOptimizeMode::SPEED => 1u32,
            FileOptionsOptimizeMode::CODE_SIZE => 2u32,
            FileOptionsOptimizeMode::LITE_RUNTIME => 3u32,
            FileOptionsOptimizeMode::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FileOptionsOptimizeMode {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FileOptionsOptimizeMode::SPEED => "SPEED",
            FileOptionsOptimizeMode::CODE_SIZE => "CODE_SIZE",
            FileOptionsOptimizeMode::LITE_RUNTIME => "LITE_RUNTIME",
            FileOptionsOptimizeMode::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("SPEED") => {
                *self = FileOptionsOptimizeMode::SPEED;
            }
            textformat::ast::Literal::Identifier("CODE_SIZE") => {
                *self = FileOptionsOptimizeMode::CODE_SIZE;
            }
            textformat::ast::Literal::Identifier("LITE_RUNTIME") => {
                *self = FileOptionsOptimizeMode::LITE_RUNTIME;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldOptionsCType {
    STRING,
    CORD,
    STRING_PIECE,
    Unknown(u32),
}
impl Default for FieldOptionsCType {
    fn default() -> FieldOptionsCType {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldOptionsCType {}
impl From<u32> for FieldOptionsCType {
    fn from(v: u32) -> FieldOptionsCType {
        match v {
            0u32 => FieldOptionsCType::STRING,
            1u32 => FieldOptionsCType::CORD,
            2u32 => FieldOptionsCType::STRING_PIECE,
            other => FieldOptionsCType::Unknown(other),
        }
    }
}
impl From<FieldOptionsCType> for u32 {
    fn from(v: FieldOptionsCType) -> u32 {
        match v {
            FieldOptionsCType::STRING => 0u32,
            FieldOptionsCType::CORD => 1u32,
            FieldOptionsCType::STRING_PIECE => 2u32,
            FieldOptionsCType::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldOptionsCType {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldOptionsCType::STRING => "STRING",
            FieldOptionsCType::CORD => "CORD",
            FieldOptionsCType::STRING_PIECE => "STRING_PIECE",
            FieldOptionsCType::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("STRING") => {
                *self = FieldOptionsCType::STRING;
            }
            textformat::ast::Literal::Identifier("CORD") => {
                *self = FieldOptionsCType::CORD;
            }
            textformat::ast::Literal::Identifier("STRING_PIECE") => {
                *self = FieldOptionsCType::STRING_PIECE;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldOptionsJSType {
    JS_NORMAL,
    JS_STRING,
    JS_NUMBER,
    Unknown(u32),
}
impl Default for FieldOptionsJSType {
    fn default() -> FieldOptionsJSType {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for FieldOptionsJSType {}
impl From<u32> for FieldOptionsJSType {
    fn from(v: u32) -> FieldOptionsJSType {
        match v {
            0u32 => FieldOptionsJSType::JS_NORMAL,
            1u32 => FieldOptionsJSType::JS_STRING,
            2u32 => FieldOptionsJSType::JS_NUMBER,
            other => FieldOptionsJSType::Unknown(other),
        }
    }
}
impl From<FieldOptionsJSType> for u32 {
    fn from(v: FieldOptionsJSType) -> u32 {
        match v {
            FieldOptionsJSType::JS_NORMAL => 0u32,
            FieldOptionsJSType::JS_STRING => 1u32,
            FieldOptionsJSType::JS_NUMBER => 2u32,
            FieldOptionsJSType::Unknown(other) => other,
        }
    }
}
impl textformat::Field for FieldOptionsJSType {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            FieldOptionsJSType::JS_NORMAL => "JS_NORMAL",
            FieldOptionsJSType::JS_STRING => "JS_STRING",
            FieldOptionsJSType::JS_NUMBER => "JS_NUMBER",
            FieldOptionsJSType::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("JS_NORMAL") => {
                *self = FieldOptionsJSType::JS_NORMAL;
            }
            textformat::ast::Literal::Identifier("JS_STRING") => {
                *self = FieldOptionsJSType::JS_STRING;
            }
            textformat::ast::Literal::Identifier("JS_NUMBER") => {
                *self = FieldOptionsJSType::JS_NUMBER;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MethodOptionsIdempotencyLevel {
    IDEMPOTENCY_UNKNOWN,
    NO_SIDE_EFFECTS,
    IDEMPOTENT,
    Unknown(u32),
}
impl Default for MethodOptionsIdempotencyLevel {
    fn default() -> MethodOptionsIdempotencyLevel {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for MethodOptionsIdempotencyLevel {}
impl From<u32> for MethodOptionsIdempotencyLevel {
    fn from(v: u32) -> MethodOptionsIdempotencyLevel {
        match v {
            0u32 => MethodOptionsIdempotencyLevel::IDEMPOTENCY_UNKNOWN,
            1u32 => MethodOptionsIdempotencyLevel::NO_SIDE_EFFECTS,
            2u32 => MethodOptionsIdempotencyLevel::IDEMPOTENT,
            other => MethodOptionsIdempotencyLevel::Unknown(other),
        }
    }
}
impl From<MethodOptionsIdempotencyLevel> for u32 {
    fn from(v: MethodOptionsIdempotencyLevel) -> u32 {
        match v {
            MethodOptionsIdempotencyLevel::IDEMPOTENCY_UNKNOWN => 0u32,
            MethodOptionsIdempotencyLevel::NO_SIDE_EFFECTS => 1u32,
            MethodOptionsIdempotencyLevel::IDEMPOTENT => 2u32,
            MethodOptionsIdempotencyLevel::Unknown(other) => other,
        }
    }
}
impl textformat::Field for MethodOptionsIdempotencyLevel {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            MethodOptionsIdempotencyLevel::IDEMPOTENCY_UNKNOWN => "IDEMPOTENCY_UNKNOWN",
            MethodOptionsIdempotencyLevel::NO_SIDE_EFFECTS => "NO_SIDE_EFFECTS",
            MethodOptionsIdempotencyLevel::IDEMPOTENT => "IDEMPOTENT",
            MethodOptionsIdempotencyLevel::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("IDEMPOTENCY_UNKNOWN") => {
                *self = MethodOptionsIdempotencyLevel::IDEMPOTENCY_UNKNOWN;
            }
            textformat::ast::Literal::Identifier("NO_SIDE_EFFECTS") => {
                *self = MethodOptionsIdempotencyLevel::NO_SIDE_EFFECTS;
            }
            textformat::ast::Literal::Identifier("IDEMPOTENT") => {
                *self = MethodOptionsIdempotencyLevel::IDEMPOTENT;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
