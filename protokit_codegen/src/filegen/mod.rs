use std::collections::{BTreeMap, HashSet};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use protokit_binformat::Encodable;
use protokit_proto::translate::TranslateCtx;
use quote::__private::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::arcstr::ArcStr;
use crate::deps::*;

pub mod grpc;
// pub mod tabular;

#[derive(Debug)]
pub struct Options {
    pub generate_text: bool,
    pub root: TokenStream,

    pub string_type: TokenStream,
    pub bytes_type: TokenStream,
    pub map_type: TokenStream,

    pub track_unknowns: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            generate_text: true,
            root: quote! { ::protokit },
            string_type: quote! { String },
            bytes_type: quote! { Vec<u8> },
            map_type: quote! { ::std::collections::HashMap },
            track_unknowns: false,
        }
    }
}

const STRICT: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
];

const RESERVED: &[&str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
    "yield",
];

pub const TYPES: &[&str] = &["Option", "Result"];

pub fn rustify_name(n: &str) -> String {
    for s in STRICT.iter().chain(RESERVED) {
        if *s == n {
            return format!("r#{}", n);
        }
    }
    for s in TYPES {
        if *s == n {
            return format!("Proto{}", n);
        }
    }
    n.replace('.', "")
}

pub fn builtin_parser(typ: BuiltinType) -> &'static str {
    match typ {
        BuiltinType::Int32 => "VInt",
        BuiltinType::Int64 => "VInt",
        BuiltinType::Uint32 => "VInt",
        BuiltinType::Uint64 => "VInt",
        BuiltinType::Sint32 => "SInt",
        BuiltinType::Sint64 => "SInt",
        BuiltinType::Bool => "Fix",
        BuiltinType::Fixed64 => "Fix",
        BuiltinType::Sfixed64 => "Fix",
        BuiltinType::Fixed32 => "Fix",
        BuiltinType::Sfixed32 => "Fix",
        BuiltinType::Double => "Fix",
        BuiltinType::Float => "Fix",
        BuiltinType::String_ => "Bytes",
        BuiltinType::Bytes_ => "Bytes",
    }
}

#[derive(Debug, Default)]
pub struct MsgOutput {
    imports: HashSet<u32>,
    builders: Vec<TokenStream>,
    fields: Vec<TokenStream>,
    tabular_fields: BTreeMap<u32, TokenStream>,
    text_decoders: Vec<TokenStream>,
    text_encoders: Vec<TokenStream>,
    bin_encoders: Vec<TokenStream>,
    bin_decoders: Vec<TokenStream>,

    oneof_defs: Vec<TokenStream>,
}

pub struct CodeGenerator<'a> {
    context: &'a TranslateCtx,
    options: &'a Options,
}

impl CodeGenerator<'_> {
    pub fn resolve_name(&self, id: DefId) -> Result<String> {
        if let Some((msg, _)) = self.context.def.message_by_id(id) {
            return Ok(rustify_name(msg.name.as_str()));
        } else if let Some((en, _)) = self.context.def.enum_by_id(id) {
            return Ok(rustify_name(en.name.as_str()));
        } else {
            bail!(
                "Unresolved {} {} {:b} files:{}: {:#?}",
                id,
                id >> 32,
                id & 0xFFFFFFFF,
                self.context.def.files.len(),
                self.context.def.files.keys()
            );
        }
    }
    pub fn builtin_rusttype(&self, typ: BuiltinType) -> TokenStream {
        TokenStream::from_str(match typ {
            BuiltinType::Int32 => "i32",
            BuiltinType::Int64 => "i64",
            BuiltinType::Uint32 => "u32",
            BuiltinType::Uint64 => "u64",
            BuiltinType::Sint32 => "i32",
            BuiltinType::Sint64 => "i64",
            BuiltinType::Bool => "bool",
            BuiltinType::Fixed64 => "u64",
            BuiltinType::Sfixed64 => "i64",
            BuiltinType::Fixed32 => "u32",
            BuiltinType::Sfixed32 => "i32",
            BuiltinType::Double => "f64",
            BuiltinType::Float => "f32",
            BuiltinType::String_ => return self.options.string_type.clone(),
            BuiltinType::Bytes_ => return self.options.bytes_type.clone(),
        })
        .unwrap()
    }
    pub fn base_type(&self, typ: &DataType) -> Result<TokenStream> {
        Ok(match typ {
            DataType::Unresolved(path) => {
                panic!("Name {} was not resolved to actual type", path)
            }
            DataType::Builtin(bt) => return Ok(self.builtin_rusttype(*bt)),
            DataType::Message(id) => {
                let name = TokenStream::from_str(&self.resolve_name(*id)?).unwrap();
                quote! { #name  }
            }
            DataType::Enum(id) => TokenStream::from_str(&self.resolve_name(*id)?).unwrap(),
            DataType::Map(m) => {
                let kt = self.base_type(&DataType::Builtin(m.deref().0))?;
                let vt = self.base_type(&m.deref().1)?;
                return Ok(quote! { ::std::collections::HashMap<#kt,#vt> });
            }
        })
    }
    pub fn type_to_encoder(&self, f: &FieldDef) -> Result<TokenStream> {
        let format = self.type_to_binformat(f)?;
        Ok(quote! { Decode::<#format>::encode })
    }
    pub fn type_to_decoder(&self, f: &FieldDef) -> Result<TokenStream> {
        let format = self.type_to_binformat(f)?;
        Ok(quote! { Decode::<#format>::decode })
    }
    pub fn type_to_binformat(&self, f: &FieldDef) -> Result<TokenStream> {
        use BuiltinType::*;
        match &f.typ {
            DataType::Builtin(String_ | Bytes_) if f.is_repeated() => Ok(quote! { Repeat::<Bytes> }),
            DataType::Builtin(String_ | Bytes_) => Ok(quote! { Bytes }),
            DataType::Builtin(bt) if f.is_packed() => {
                let inner = format_ident!("{}", builtin_parser(*bt));
                Ok(quote! { Pack::<#inner> })
            }
            DataType::Builtin(bt) if f.is_repeated() => {
                let inner = format_ident!("{}", builtin_parser(*bt));
                Ok(quote! { Repeat::<#inner> })
            }
            DataType::Builtin(bt) => {
                let inner = format_ident!("{}", builtin_parser(*bt));
                Ok(quote! { #inner })
            }
            DataType::Message(m) => {
                let _inner = format_ident!("{}", self.resolve_name(*m).unwrap());
                Ok(if f.is_repeated() {
                    quote! { Repeat::<Nest> }
                } else {
                    quote! { Nest }
                })
            }
            DataType::Enum(e) => {
                let _en = self.resolve_name(*e).unwrap();
                Ok(if f.is_repeated() {
                    quote! { Repeat::<Enum> }
                } else {
                    quote! { Enum }
                })
            }
            DataType::Map(mt) => {
                let kp = builtin_parser(mt.deref().0);
                let kp = format_ident!("{kp}");

                let fd = FieldDef {
                    name: Default::default(),
                    frequency: Frequency::Singular,
                    typ: mt.deref().1.clone(),
                    num: 0,
                    #[cfg(feature = "descriptors")]
                    options: Default::default(),
                };

                let vp = self.type_to_binformat(&fd)?;
                Ok(quote! { Map::<#kp, #vp> })
            }
            DataType::Unresolved(name) => panic!("{name} was not resolved"),
        }
    }

    pub fn field_binformat(
        &self,
        out: &mut MsgOutput,
        _msg_name: &Ident,
        name: &Ident,
        _field_idx: usize,
        field: &FieldDef,
    ) {
        let mut field: FieldDef = field.clone();
        let field_num = field.num as u32;

        let encode_tag = field.default_wire_type();
        let encode_tag = (field_num << 3 | encode_tag as u32) as u32;
        let encode_fn = self.type_to_encoder(&field).unwrap();

        out.bin_encoders.push(quote! {
            #encode_fn(&self.#name, #encode_tag, buf)?;
        });

        let (normal, packed) = field.wire_types();

        field.set_packed(false);
        let normal_tag = (field_num << 3 | normal as u32) as u32;
        let decode_fn = self.type_to_decoder(&field).unwrap();
        out.bin_decoders.push(quote! {
            #normal_tag => {
                buf = #decode_fn(&mut self.#name, buf)?;
            }
        });

        // Generate for the case, when we have packed
        if let Some(wire_type) = packed {
            let packed_tag = field_num << 3 | wire_type as u32;
            let mut field = field.clone();
            field.set_packed(true);
            let parse_fn = self.type_to_decoder(&field).unwrap();

            out.bin_decoders.push(quote! {
                #packed_tag => {
                    buf = #parse_fn(&mut self.#name, buf)?;
                }
            });
        }
    }

    pub fn field_textformat(
        &self,
        out: &mut MsgOutput,
        field_name: &Ident,
        field: &FieldDef,
        field_proto_name: &str,
        field_type: &TokenStream,
        ext_pkg: Option<&ArcStr>,
    ) {
        let (field_textformat_key, sep) = match &field.typ {
            DataType::Builtin(_) | DataType::Enum(_) => (format!("{}: ", field_proto_name), ":"),
            _ => (format!("{} ", field_proto_name), ""),
        };

        if let Some(pkg) = ext_pkg {
            let ext_name = format!("{}.{}", pkg.as_str(), field_proto_name);
            let ext_key = format!("[{}.{}]{} ", pkg.as_str(), field_proto_name, sep);
            out.text_decoders.push(quote! {
                textformat::ast::FieldName::Extended(#ext_name) => {
                    textformat::Field::merge(&mut self.#field_name, ctx, value)?;
                }
            });
            out.text_encoders.push(quote! {
                if self.#field_name != <#field_type as Default>::default() {
                    out.indent(pad);
                    out.push_str(#ext_key);
                    textformat::Field::format(&self.#field_name, ctx, pad, out)?;
                    out.push('\n');
                }
            });
        } else {
            out.text_decoders.push(quote! {
                textformat::ast::FieldName::Normal(#field_proto_name) => {
                    textformat::Field::merge(&mut self.#field_name, ctx, value)?;
                }
            });
            out.text_encoders.push(quote! {
                if self.#field_name != <#field_type as Default>::default() {
                    out.indent(pad);
                    out.push_str(#field_textformat_key);
                    textformat::Field::format(&self.#field_name, ctx, pad, out)?;
                    out.push('\n');
                }
            });
        }
    }

    pub fn field_methods(
        &self,
        out: &mut MsgOutput,
        field_name: &Ident,
        field: &FieldDef,
        field_raw_name: &str,
        field_item_type: &TokenStream,
    ) -> TokenStream {
        let with_name = format_ident!("r#with_{field_raw_name}");

        let (field_type, setter) = if field.is_repeated() {
            (quote! {Vec<#field_item_type>}, quote! { self.#field_name = it.into(); })
        } else if field.is_message() {
            (
                quote! { Option<Box<#field_item_type>> },
                quote! { self.#field_name = Box::new(it).into(); },
            )
        } else if field.is_optional() {
            (
                quote! { Option<#field_item_type> },
                quote! { self.#field_name = it.into(); },
            )
        } else {
            (quote! { #field_item_type }, quote! { self.#field_name = it.into(); })
        };

        if let DataType::Map(m) = &field.typ {
            let add_name = format_ident!("r#add_{field_raw_name}");
            let k_typ = self
                .base_type(&DataType::Builtin(m.deref().0))
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            let v_typ = self
                .base_type(&m.deref().1)
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            out.builders.push(quote! {
                #[inline(always)]
                pub fn #with_name(mut self, k: #k_typ, v: #v_typ) -> Self {
                    self.#add_name(k, v);
                    self
                }
                #[inline(always)]
                pub fn #add_name(&mut self, k: #k_typ, v: #v_typ) -> &mut Self {
                    let _ = self.#field_name.insert(k, v);
                    self
                }
            });
        } else if field.is_repeated() {
            let add_name = format_ident!("r#add_{field_raw_name}");
            out.builders.push(quote! {
                #[inline(always)]
                pub fn #with_name(mut self, it: #field_item_type) -> Self {
                    self.#add_name(it);
                    self
                }
                #[inline(always)]
                pub fn #add_name(&mut self, it: #field_item_type) -> &mut Self {
                    self.#field_name.push(it);
                    self
                }
            });
        } else {
            let set_name = format_ident!("r#set_{field_raw_name}");
            out.builders.push(quote! {
                #[inline(always)]
                pub fn #with_name(mut self, it: #field_item_type) -> Self {
                    self.#set_name(it);
                    self
                }
                #[inline(always)]
                pub fn #set_name(&mut self, it: #field_item_type) -> &mut Self {
                    #setter
                    self
                }
            });
        }
        field_type
    }

    pub fn message_field(
        &self,
        out: &mut MsgOutput,
        file: &FileDef,
        unit_id: u64,
        _unit: &MessageDef,
        msg_name: &Ident,
        field_idx: usize,
        field: &FieldDef,
        ext_pkg: Option<&ArcStr>,
    ) {
        let field_raw_name = field.name.as_str();
        // Find out if we need to add additional import
        match field.typ {
            DataType::Message(m) if (m >> 32) != unit_id >> 32 => {
                out.imports.insert((m >> 32) as u32);
            }
            _ => {}
        }
        let field_name = format_ident!("{}", rustify_name(field.name.as_str()));

        let field_item_type = self
            .base_type(&field.typ)
            .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
            .expect("Resolving name");

        let field_type = self.field_methods(out, &field_name, field, field_raw_name, &field_item_type);
        out.fields.push(quote! { pub #field_name: #field_type });
        if self.options.generate_text {
            self.field_textformat(out, &field_name, field, field_raw_name, &field_type, ext_pkg);
        }
        self.field_binformat(out, msg_name, &field_name, field_idx, field);
    }

    pub fn oneof_variant(
        &self,
        out: &mut MsgOutput,
        variant_encoders: &mut Vec<TokenStream>,
        variant_text_encoders: &mut Vec<TokenStream>,
        oneof: &OneOfDef,
        oneof_name: &Ident,
        oneof_type: &Ident,
        variant: &FieldDef,
    ) {
        let oneof_proto_name = oneof.name.as_str();
        let variant_proto_name = variant.name.as_str();

        let variant_name = format_ident!("{}", variant_proto_name.to_case(Case::Pascal));
        let variant_textformat_key = match variant.typ {
            DataType::Builtin(_) | DataType::Enum(_) => {
                format!("{}: ", variant_proto_name)
            }
            _ => {
                format!("{} ", variant_proto_name)
            }
        };

        let var_typ = self
            .base_type(&variant.typ)
            // .with_context(|| format!("{}", name))
            .expect("Resolving name");

        out.text_decoders.push(quote! {
            textformat::ast::FieldName::Normal(#variant_proto_name) => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.#oneof_name = #oneof_type::#variant_name(target);
            }
        });

        let (normal, packed) = variant.wire_types();
        let encode_fn = self.type_to_encoder(variant).unwrap();
        let decode_fn = self.type_to_decoder(variant).unwrap();
        let id = variant.num as u32;

        let normal_tag = (id << 3 | normal as u32) as u32;

        variant_encoders.push(quote! {
            #oneof_type::#variant_name(value) => {
                #encode_fn(value, #normal_tag, buf)?;
            }
        });

        out.bin_decoders.push(quote! {
            #normal_tag => {
                let mut tmp = Default::default();
                buf = #decode_fn(&mut tmp, buf)?;
                self.#oneof_name = #oneof_type::#variant_name(tmp);
            }
        });

        if let Some(packed) = packed {
            let packed_tag = id << 3 | packed as u32;
            let mut variant = variant.clone();
            variant.set_packed(true);
            let parse_fn = self.type_to_decoder(&variant).unwrap();

            out.bin_decoders.push(quote! {
                #packed_tag => {
                    let mut tmp = Default::default();
                    buf = #parse_fn(&mut tmp, buf)?;
                    self.#oneof_name = #oneof_type::#variant_name(tmp);
                }
            });
        }

        variant_text_encoders.push(quote! {
            #oneof_type::#variant_name(value) => {
                out.indent(pad);
                out.push_str(#variant_textformat_key);
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
        });

        let set_name = format_ident!("r#set_{oneof_proto_name}_{variant_proto_name}");
        let with_name = format_ident!("r#with_{oneof_proto_name}_{variant_proto_name}");

        out.builders.push(quote! {
            #[inline(always)]
            pub fn #with_name(mut self, it: #var_typ) -> Self {
                self.#oneof_name = #oneof_type::#variant_name(it);
                self
            }
            #[inline(always)]
            pub fn #set_name(&mut self, it: #var_typ) -> &mut Self {
                self.#oneof_name = #oneof_type::#variant_name(it);
                self
            }

        });
    }

    pub fn message_oneof(
        &self,
        out: &mut MsgOutput,
        normal_field_count: usize,
        _msg_id: u64,
        _msg: &MessageDef,
        msg_name: &Ident,
        oneof_idx: usize,
        oneof: &OneOfDef,
    ) {
        let _oneof_idx = normal_field_count + oneof_idx;
        let _oneof_bin_decoders: Vec<TokenStream> = vec![];

        let oneof: &OneOfDef = oneof;
        let oneof_raw_name = oneof.name.as_str();
        let oneof_name = format_ident!("{}", rustify_name(oneof_raw_name));
        let oneof_type = format_ident!("{msg_name}OneOf{}", oneof.name.as_str().to_case(Case::Pascal));

        let mut variant_encoders = vec![];
        let mut variant_text_encoders = vec![];

        out.oneof_defs.push(self.generate_oneof(msg_name, oneof, &oneof_type));
        out.fields.push(quote! { pub #oneof_name: #oneof_type });
        for (_vname, var_num) in oneof.fields.by_name.iter() {
            let variant = &oneof.fields.by_number[var_num];
            self.oneof_variant(
                out,
                &mut variant_encoders,
                &mut variant_text_encoders,
                oneof,
                &oneof_name,
                &oneof_type,
                variant,
            );
        }
        out.bin_encoders.push(quote! {
            match &self.#oneof_name {
                #(#variant_encoders)*
                #oneof_type::Unknown(..) => {},
            }
        });
        out.text_encoders.push(quote! {
            match &self.#oneof_name {
                #(#variant_text_encoders)*
                #oneof_type::Unknown(..) => {},
            }
        });
    }

    pub fn generate_msg(
        &self,
        set: &FileSetDef,
        file: &FileDef,
        unit_id: DefId,
        unit: &MessageDef,
    ) -> (TokenStream, Vec<Ident>, HashSet<usize>) {
        let proto_name = unit.name.as_str();
        let msg_name = format_ident!("{}", rustify_name(proto_name));
        let pkg = &file.package;
        let qualified_name = format!("{pkg}.{proto_name}");

        let msg_types = vec![msg_name.clone()];

        let mut out = MsgOutput::default();

        struct TmpField<'a> {
            field_idx: usize,
            tag_num: FieldNum,
            def: &'a FieldDef,
            pkg: Option<ArcStr>,
        }

        let field_iter = unit
            .fields
            .by_number
            .iter()
            .enumerate()
            .map(|(idx, (num, def))| TmpField {
                field_idx: idx,
                tag_num: *num,
                def,
                pkg: None,
            });

        let ext_iter = file.extenders.get(&(unit_id as LocalDefId));
        let ext_iter = ext_iter.iter().flat_map(|ext_ids| {
            ext_ids.iter().flat_map(|ext_defid| {
                let file_id = (ext_defid >> 32) as usize;
                let (_ext_file_name, ext_file): (_, &FileDef) = set.files.get_index(file_id).unwrap();
                let (_, ext_def): (_, &ExtendDef) = ext_file
                    .extensions
                    .get_index((*ext_defid as u32 & LOCAL_ONLY_ID) as usize)
                    .unwrap();

                ext_def
                    .fields
                    .by_number
                    .iter()
                    .enumerate()
                    .map(|(idx, (num, def))| TmpField {
                        field_idx: idx,
                        tag_num: *num,
                        def: &def,
                        pkg: Some(ext_file.package.clone()),
                    })
            })
        });

        let mut normal_field_count = 0;
        for TmpField {
            field_idx,
            tag_num,
            def,
            pkg,
        } in field_iter.chain(ext_iter)
        {
            self.message_field(&mut out, file, unit_id, unit, &msg_name, field_idx, def, pkg.as_ref());
            normal_field_count = field_idx;
        }

        for (oneof_idx, (_oneof_name, oneof)) in unit.oneofs.iter().enumerate() {
            self.message_oneof(&mut out, normal_field_count, unit_id, unit, &msg_name, oneof_idx, oneof)
        }

        let MsgOutput {
            text_decoders,
            text_encoders,
            bin_decoders,
            bin_encoders,
            mut fields,
            builders,
            tabular_fields: _,
            imports,
            oneof_defs,
        } = out;

        // bin_encoders.reverse();

        let text_format = if self.options.generate_text {
            quote! {
                impl textformat::Decodable for #msg_name {
                    fn merge_field(&mut self, ctx: &textformat::Context, name: &textformat::ast::FieldName, value: &textformat::ast::FieldValue) -> textformat::Result<()> {
                        match name {
                            #(#text_decoders),*
                            other => textformat::bail!("{other:?} was not recognized"),
                        }
                        Ok(())
                    }
                }
                impl textformat::Encodable for #msg_name {
                    fn encode(&self, ctx: &textformat::Context, pad: usize, out: &mut std::string::String) -> textformat::Result<()> {
                        #(#text_encoders)*
                        Ok(())
                    }
                }
            }
        } else {
            quote! {}
        };

        if self.options.track_unknowns {
            fields.push(quote! { pub _unknown: util::UnknownFields })
        } else {
            fields.push(quote! { pub _unknown: () })
        }

        let result = quote! {
            #[repr(C)]
            #[derive( Debug, Default, Clone, PartialEq, )]
            pub struct #msg_name  {
                #(#fields,)*
            }

            impl #msg_name {
                #(#builders)*
            }

            #text_format

            impl binformat::Decodable for #msg_name {
                fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: binformat::ReadBuffer<'b>) -> binformat::Result< binformat::ReadBuffer<'b>> {
                    use binformat::format::*;
                    match tag {
                        #(#bin_decoders)*
                        other => buf = self._unknown.merge_field(tag, buf)?,
                    }
                    Ok(buf)
                }
            }

            impl binformat::Encodable for #msg_name {
                fn qualified_name(&self) -> &'static str {
                    #qualified_name
                }
                fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
                    use binformat::format::*;
                    #(#bin_encoders)*
                    binformat::Encodable::encode(&self._unknown, buf)?;
                    Ok(())
                }
            }
            #(#oneof_defs)*
        };

        (result, msg_types, imports.into_iter().map(|v| v as _).collect())
    }

    pub fn generate_oneof(&self, _msg_name: &Ident, unit: &OneOfDef, oneof_type: &Ident) -> TokenStream {
        let mut fields = vec![];

        for (num, field) in unit.fields.by_number.iter() {
            let _bit = (1 << *num) as u32;
            let field: &FieldDef = field;
            let name = format_ident!("{}", field.name.as_str().to_case(Case::Pascal));
            let typ = self
                .base_type(&field.typ)
                .with_context(|| format!("{}", name))
                .expect("Resolving name");
            if field.frequency != Frequency::Singular {
                panic!("Oneof fields can't have frequency");
            }
            fields.push(quote! { #name(#typ) })
        }

        let def = &unit.fields.by_number.first().unwrap().1.name;
        let def = format_ident!("{}", rustify_name(def.as_str()).to_case(Case::Pascal));
        let _def = quote! { #oneof_type::#def(Default::default()) };

        quote! {
            #[repr(C, u32)]
            #[derive(Debug, Clone, PartialEq)]
            pub enum #oneof_type  {
                #(#fields,)*
                Unknown(::core::marker::PhantomData<()>),
            }
            impl Default for #oneof_type {
                fn default() -> Self {
                    #oneof_type::Unknown(::core::marker::PhantomData)
                }
            }
        }
    }

    pub fn generate_enum(&self, unit: &EnumDef) -> TokenStream {
        let enum_name = format_ident!("{}", rustify_name(unit.name.as_str()));

        let mut variants = vec![];
        let mut arms_from = vec![];
        let mut arms_into = vec![];
        let mut text_from = vec![];
        let mut text_to = vec![];

        for (_name, field) in unit.variants.by_name.iter() {
            let name_str = field.name.as_str();
            let name = format_ident!("{}", field.name.as_str()); //ctx.syms.resolve(field.name).unwrap());
            let num = field.num as u32;

            variants.push(quote! { #name });
            arms_from.push(quote! { #num => #enum_name::#name });
            arms_into.push(quote! { #enum_name::#name => #num });
            text_from.push(quote! { textformat::ast::Literal::Identifier(#name_str) => *self = #enum_name::#name});
            text_to.push(quote! { #enum_name::#name => #name_str, });
        }

        let text_format = if self.options.generate_text {
            quote! {
                impl textformat::Field for #enum_name {
                    fn format(&self, ctx: &textformat::Context, pad: usize, out: &mut String) -> ::std::fmt::Result {
                        let str = match self {
                            #(#text_to)*
                            #enum_name::Unknown(n) => {
                                write!(out, "{n}")?;
                                return Ok(());
                            }
                        };
                        out.push_str(str);
                        Ok(())
                    }
                    fn merge_scalar(&mut self, _ctx: &textformat::Context, v: &textformat::ast::Literal) -> textformat::Result<()> {
                        match v {
                            #(#text_from,)*
                            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
                            other => textformat::bail!("Invalid enum value: {other:?}")
                        }
                        Ok(())
                    }
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum #enum_name {
                #(#variants,)*
                Unknown(u32)
            }
            impl Default for #enum_name {
                fn default() -> #enum_name {
                    Self::from(0)
                }
            }
            impl binformat::format::ProtoEnum for #enum_name {

            }
            impl From<u32> for #enum_name {
                fn from(v: u32) -> #enum_name {
                    match v {
                        #(#arms_from,)*
                        other => #enum_name::Unknown(other),
                    }
                }
            }
            impl From<#enum_name> for u32 {
                fn from(v: #enum_name) -> u32 {
                    match v {
                        #(#arms_into,)*
                        #enum_name::Unknown(other) => other,
                    }
                }
            }
            // impl Into<u32> for #enum_name {
            //     fn into(self) -> u32 {
            //         match self {
            //             #(#arms_into,)*
            //             #enum_name::Unknown(other) => other,
            //         }
            //     }
            // }
            #text_format
        }
    }
}

pub fn generate_file(ctx: &TranslateCtx, opts: &Options, name: PathBuf, unit_id: usize, unit: &FileDef) {
    create_dir_all(name.parent().unwrap()).unwrap();
    let root = opts.root.clone();

    let mut push_imports = HashSet::new();
    let mut msg_types = vec![];
    let mut messages = vec![];
    let mut enums = vec![];
    let mut services: Vec<TokenStream> = vec![];

    let generator = CodeGenerator {
        context: ctx,
        options: opts,
    };

    for (idx, (_sym, msg)) in unit.messages.iter().enumerate() {
        let defid = (unit_id as u64) << 32 | LOCAL_DEFID_MSG as u64 | idx as u64;
        let (msg_contents, types, imports) = generator.generate_msg(&ctx.def, unit, defid, msg);
        push_imports.extend(imports);
        msg_types.extend(types.into_iter());
        messages.push(msg_contents);
    }
    for (idx, (_name, en)) in unit.enums.iter().enumerate() {
        let _defid = (unit_id as u64) << 32 | LOCAL_DEFID_ENUM as u64 | idx as u64;
        enums.push(generator.generate_enum(en));
    }
    for (_name, svc) in unit.services.iter() {
        services.push(generator.generate_server(unit, svc));
        services.push(generator.generate_client(unit, svc));
    }

    let imports = unit
        .imports
        .iter()
        .map(|imp| imp.file_idx)
        .chain(push_imports.into_iter())
        .map(|file_idx| {
            let (_, file): (_, &FileDef) = ctx.def.files.get_index(file_idx).unwrap();
            let _our_name = name.file_name().unwrap().to_str().unwrap();

            if let Some(rep) = ctx.replacement.get(file.name.as_str()) {
                let rep = TokenStream::from_str(rep).unwrap();
                return quote! { use #rep::*; };
            }

            let their_name = if file.name.contains('/') {
                &file.name.as_str()[file.name.rfind('/').unwrap() + 1 ..]
            } else {
                file.name.as_str()
            };
            let their_name = if their_name.contains('.') {
                &their_name[.. their_name.rfind('.').unwrap()]
            } else {
                their_name
            };
            let mut our_module = unit.package.as_str();
            let mut that_module = file.package.as_str();

            while !our_module.is_empty() && !that_module.is_empty() && our_module[.. 1] == that_module[.. 1] {
                our_module = &our_module[1 ..];
                that_module = &that_module[1 ..];
            }
            let mut path = String::new();
            path.push_str("super::");

            if !our_module.is_empty() {
                for _s in our_module.strip_prefix('.').unwrap_or(our_module).split('.') {
                    path.push_str("super::");
                }
            }
            if !that_module.is_empty() {
                for s in that_module.split('.') {
                    path.push_str(&rustify_name(s));
                    path.push_str("::")
                }
            }
            path.push_str(&rustify_name(their_name));

            let import = TokenStream::from_str(&path).unwrap();
            quote! {
                use #import::*;
            }
        });
    let output = quote! {
        #![allow(nonstandard_style)]
        #![allow(unused)]
        #![deny(unused_must_use)]
        #![allow(clippy::derive_partial_eq_without_eq)]

        use std::fmt::Write;

        use #root::*;
        use #root as root;

        #(#imports)*

        pub fn register_types(registry: &mut reflect::Registry) {
            #(registry.register(&#msg_types::default());)*
        }

        #(#messages)*
        #(#enums)*
        #(#services)*
    };
    let output = syn::parse2(output).unwrap();
    let output = prettyplease::unparse(&output);
    println!("Creating file: {:?}", name);
    let mut f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .unwrap();
    f.write_all(output.as_bytes()).unwrap();
    f.flush().unwrap();
}

#[cfg(feature = "descriptors")]
pub fn generate_descriptor(ctx: &TranslateCtx, name: impl AsRef<Path>) {
    let mut output = vec![];
    ctx.def.to_descriptor().encode(&mut output).unwrap();

    let mut f = make_file(name);

    f.write_all(&output).unwrap();
    f.flush().unwrap();
}

pub fn generate_mod<'s>(path: impl AsRef<Path>, opts: &Options, files: impl Iterator<Item = &'s str>) {
    let root = opts.root.clone();
    let files: Vec<_> = files
        .map(|v| {
            let v = v.strip_suffix(".rs").unwrap_or(v);
            format_ident!("{}", rustify_name(v))
        })
        .collect();

    let output = quote! {
        #(
            pub mod #files;
        )*
        pub fn register_types(registry: &mut #root::reflect::Registry) {
            #(#files::register_types(registry);)*
        }
    };

    create_dir_all(path.as_ref().parent().unwrap()).unwrap();

    let mut f = make_file(path.as_ref().join("mod.rs"));

    let output = syn::parse2(output).unwrap();
    let output = prettyplease::unparse(&output);
    f.write_all(output.as_bytes()).unwrap();
    f.flush().unwrap();
}

pub fn make_file(path: impl AsRef<Path>) -> std::fs::File {
    let path = path.as_ref();

    // println!("cargo:rerun-if-changed={}", path.to_string_lossy());

    let f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Creating mod.rs in, {:?}", path));

    f
}
