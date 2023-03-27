use std::collections::{BTreeMap, HashSet};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{Context, Result};
use convert_case::Case::{Pascal, UpperSnake};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use protokit_proto::translate::TranslateCtx;
use quote::{format_ident, quote};

use crate::arcstr::ArcStr;
use crate::deps::*;

pub mod grpc;

#[derive(Debug)]
pub struct Options {
    pub generate_textformat: bool,
    pub import_root: TokenStream,

    pub string_type: TokenStream,
    pub bytes_type: TokenStream,
    pub map_type: TokenStream,

    pub track_unknowns: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            generate_textformat: true,
            import_root: quote! { ::protokit },
            string_type: quote! { String },
            bytes_type: quote! { Vec<u8> },
            map_type: quote! { ::std::collections::BTreeMap },
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

pub fn rustify_name(n: impl AsRef<str>) -> String {
    let n = n.as_ref();
    for s in STRICT.iter().chain(RESERVED) {
        if *s == n {
            return format!("r#{n}");
        }
    }
    for s in TYPES {
        if *s == n {
            return format!("Proto{n}");
        }
    }
    n.replace('.', "")
}

pub fn builtin_type_marker(typ: BuiltinType) -> &'static str {
    match typ {
        BuiltinType::Int32 => "varint",
        BuiltinType::Int64 => "varint",
        BuiltinType::Uint32 => "varint",
        BuiltinType::Uint64 => "varint",
        BuiltinType::Sint32 => "sigint",
        BuiltinType::Sint64 => "sigint",
        BuiltinType::Bool => "bool",
        BuiltinType::Fixed64 => "fixed64",
        BuiltinType::Sfixed64 => "fixed64",
        BuiltinType::Fixed32 => "fixed32",
        BuiltinType::Sfixed32 => "fixed32",
        BuiltinType::Double => "fixed64",
        BuiltinType::Float => "fixed32",
        BuiltinType::String_ => "string",
        BuiltinType::Bytes_ => "bytes",
    }
}

#[derive(Debug, Default)]
pub struct MessageParts {
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
    proto3: bool,

    output: Vec<TokenStream>,
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

    pub fn type_marker(&self, typ: &DataType) -> TokenStream {
        TokenStream::from_str(match typ {
            DataType::Unresolved(_) => panic!(),
            DataType::Builtin(b) => builtin_type_marker(*b),
            DataType::Message(m) => "nested",
            DataType::Enum(m) => "protoenum",
            DataType::Map(k) => {
                return TokenStream::from_str(&format!(
                    "map({}, {})",
                    builtin_type_marker(k.0),
                    self.type_marker(&k.1)
                ))
                .unwrap()
            }
        })
        .unwrap()
    }

    pub fn base_type(&self, typ: &DataType) -> Result<TokenStream> {
        Ok(match typ {
            DataType::Unresolved(path) => {
                panic!("Name {path} was not resolved to actual type")
            }
            DataType::Builtin(bt) => return Ok(self.builtin_rusttype(*bt)),
            DataType::Message(id) => TokenStream::from_str(&self.resolve_name(*id)?).unwrap(),
            DataType::Enum(id) => TokenStream::from_str(&self.resolve_name(*id)?).unwrap(),
            DataType::Map(m) => {
                let kt = self.base_type(&DataType::Builtin(m.deref().0))?;
                let vt = self.base_type(&m.deref().1)?;
                let mt = &self.options.map_type;
                return Ok(quote! { #mt<#kt,#vt> });
            }
        })
    }
    pub fn field_type(&self, typ: &FieldDef) -> Result<TokenStream> {
        let base = self.base_type(&typ.typ)?;
        let is_msg = match typ.typ {
            DataType::Message(_) => true,
            _ => false,
        };
        match (typ.frequency, is_msg) {
            (Frequency::Singular | Frequency::Required, false) => Ok(base),
            (Frequency::Singular | Frequency::Required, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Optional, false) => Ok(quote!(Option<#base>)),
            (Frequency::Optional, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Repeated, _) => Ok(quote!(Vec<#base>)),
        }
    }

    pub fn file(&mut self, file_id: usize, f: &FileDef) -> Result<()> {
        self.proto3 = f.syntax == Syntax::Proto3;
        for (index, (name, en)) in f.enums.iter().enumerate() {
            self.r#enum(en)?;
        }
        for (index, (name, msg)) in f.messages.iter().enumerate() {
            self.message(file_id, index, name, msg)?
        }

        Ok(())
    }

    pub fn message(&mut self, file_id: usize, msg_id: usize, msg_name: &ArcStr, msg: &MessageDef) -> Result<()> {
        let ident = format_ident!("{}", rustify_name(msg_name));

        let fields = msg
            .fields
            .by_number
            .iter()
            .map(|(num, f)| self.field(f))
            .collect::<Result<Vec<_>>>()?;

        let oneofs = msg
            .oneofs
            .iter()
            .map(|(name, def)| self.oneof(&msg_name, def))
            .collect::<Result<Vec<_>>>()?;

        self.output.push(quote! {
            #[derive(Debug, Default, Clone, Proto)]
            pub struct #ident {
                #(#fields,)*
                #(#oneofs,)*
            }
        });

        Ok(())
    }

    pub fn oneof(&mut self, msg_name: &str, def: &OneOfDef) -> Result<TokenStream> {
        let field_ident = format_ident!("{}", rustify_name(&def.name));
        let oneof_type = format_ident!("{msg_name}OneOf{}", def.name.as_str().to_case(Case::Pascal));

        let mut nums = vec![];
        let mut names = vec![];
        let mut vars = vec![];

        let mut default = None;

        for (i, (n, var)) in def.fields.by_number.iter().enumerate() {
            let var_name = format_ident!("{}", var.name.as_str().to_case(Case::Pascal));
            let typ = self.base_type(&var.typ)?;
            let num = var.num as u32;
            let name = var.name.as_str();
            let kind = self.type_marker(&var.typ);

            vars.push(quote! {
                #[field(#num, #name, #kind, singular)]
                #var_name(#typ),
            });

            if default.is_none() {
                default = Some(quote! {
                    impl Default for #oneof_type {
                        fn default() -> Self {
                            Self::#var_name(Default::default())
                        }
                    }
                })
            }

            nums.push(num);
            names.push(name);
        }

        self.output.push(quote! {
            #[derive(Debug, Clone, Proto)]
            pub enum #oneof_type {
                #(#vars)*
            }
            #default
        });

        Ok(quote! {
            #[oneof([#(#nums,)*], [#(#names,)*])]
            pub #field_ident: Option<#oneof_type>
        })
    }

    pub fn r#enum(&mut self, def: &EnumDef) -> Result<()> {
        let ident = format_ident!("{}", rustify_name(def.name.as_str()));
        let variants = def.variants.by_name.iter().map(|(var, def)| {
            let name = def.name.as_str();
            let var_ident = format_ident!("{}", def.name.as_str());
            let num = def.num as u32;
            quote! {
                #[var(#num, #name)]
                pub const #var_ident: #ident = #ident(#num);
            }
        });
        self.output.push(quote! {
            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct #ident(pub u32);
            #[protoenum]
            impl #ident {
                #(#variants)*
            }
        });

        Ok(())
    }

    pub fn field(&self, def: &FieldDef) -> Result<TokenStream> {
        let typ = self.field_type(def)?;
        let fname = format_ident!("{}", rustify_name(def.name.as_str()));
        let name = def.name.as_str();
        let num = def.num as u32;

        let kind = self.type_marker(&def.typ);
        let freq = TokenStream::from_str(match def.frequency {
            Frequency::Singular => "singular",
            Frequency::Optional => "optional",
            Frequency::Repeated if def.is_packed() => "packed",
            Frequency::Repeated if self.proto3 && def.typ.is_scalar() => "packed",
            Frequency::Repeated => "repeated",
            Frequency::Required => "required",
        })
        .unwrap();

        Ok(quote! {
            #[field(#num, #name, #kind, #freq)]
            pub #fname: #typ
        })
    }
}

struct FileOutput {
    messages: Vec<Ident>,
    imports: HashSet<usize>,
}

// pub fn message_field(
//     &self,
//     out: &mut MessageParts,
//     file: &FileDef,
//     unit_id: u64,
//     _unit: &MessageDef,
//     msg_name: &Ident,
//     field_idx: usize,
//     field: &FieldDef,
//     ext_pkg: Option<&ArcStr>,
// ) {
//     let field_raw_name = field.name.as_str();
//     // Find out if we need to add additional import
//     match field.typ {
//         DataType::Message(m) if (m >> 32) != unit_id >> 32 => {
//             out.imports.insert((m >> 32) as u32);
//         }
//         _ => {}
//     }
//     let field_name = format_ident!("{}", rustify_name(field.name.as_str()));
//
//     let field_item_type = self
//         .base_type(&field.typ)
//         .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
//         .expect("Resolving name");
//
//     let field_type = self.field_methods(out, &field_name, field, field_raw_name, &field_item_type);
//     out.fields.push(quote! { pub #field_name: #field_type });
//     if self.options.generate_textformat {
//         self.field_textformat(out, &field_name, field, field_raw_name, &field_type, ext_pkg);
//     }
//     self.field_binformat(out, file, msg_name, &field_name, field_idx, field);
// }
//
// pub fn oneof_variant(
//     &self,
//     out: &mut MessageParts,
//     file: &FileDef,
//     variant_encoders: &mut Vec<TokenStream>,
//     variant_text_encoders: &mut Vec<TokenStream>,
//     oneof: &OneOfDef,
//     oneof_name: &Ident,
//     oneof_type: &Ident,
//     variant: &FieldDef,
// ) {
//     let oneof_proto_name = oneof.name.as_str();
//     let variant_proto_name = variant.name.as_str();
//
//     let variant_name = format_ident!("{}", variant_proto_name.to_case(Case::Pascal));
//     let variant_textformat_key = match variant.typ {
//         DataType::Builtin(_) | DataType::Enum(_) => {
//             format!("{variant_proto_name}: ")
//         }
//         _ => {
//             format!("{variant_proto_name} ")
//         }
//     };
//
//     let var_typ = self
//         .base_type(&variant.typ)
//         // .with_context(|| format!("{}", name))
//         .expect("Resolving name");
//
//     out.text_decoders.push(quote! {
//         textformat::ast::FieldName::Normal(#variant_proto_name) => {
//             let mut target = Default::default();
//             textformat::Field::merge(&mut target, ctx, value)?;
//             self.#oneof_name = #oneof_type::#variant_name(target);
//         }
//     });
//
//     let (normal, packed) = variant.wire_types();
//     let encode_fn = self.type_to_encoder(variant, file.syntax == Proto3).unwrap();
//     let decode_fn = self.type_to_decoder(variant).unwrap();
//     let field_num = variant.num as u32;
//
//     let normal_tag = field_num << 3 | normal as u32;
//
//     variant_encoders.push(quote! {
//         #oneof_type::#variant_name(value) => {
//             #encode_fn(value, #field_num, buf)?;
//         }
//     });
//
//     out.bin_decoders.push(quote! {
//         #normal_tag => {
//             if let #oneof_type::#variant_name(tmp) = &mut self.#oneof_name {
//
//                 buf = #decode_fn(tmp, buf)?;
//             } else {
//                 let mut tmp = Default::default();
//                 buf = #decode_fn(&mut tmp, buf)?;
//                 self.#oneof_name = #oneof_type::#variant_name(tmp);
//             }
//         }
//     });
//
//     if let Some(packed) = packed {
//         let packed_tag = field_num << 3 | packed as u32;
//         let mut variant = variant.clone();
//         variant.set_packed(true);
//         let decode_fn = self.type_to_decoder(&variant).unwrap();
//
//         out.bin_decoders.push(quote! {
//             #packed_tag => {
//                 if let #oneof_type::#variant_name(tmp) = &mut self.#oneof_name {
//                     buf = #decode_fn(tmp, buf)?;
//                 } else {
//                     let mut tmp = Default::default();
//                     buf = #decode_fn(&mut tmp, buf)?;
//                     self.#oneof_name = #oneof_type::#variant_name(tmp);
//                 }
//             }
//         });
//     }
//
//     variant_text_encoders.push(quote! {
//         #oneof_type::#variant_name(value) => {
//             out.indent(pad);
//             out.push_str(#variant_textformat_key);
//             textformat::Field::format(value, ctx, pad, out)?;
//             out.push('\n');
//         }
//     });
//
//     let set_name = format_ident!("r#set_{oneof_proto_name}_{variant_proto_name}");
//     let with_name = format_ident!("r#with_{oneof_proto_name}_{variant_proto_name}");
//
//     out.builders.push(quote! {
//         #[inline(always)]
//         pub fn #with_name(mut self, it: #var_typ) -> Self {
//             self.#oneof_name = #oneof_type::#variant_name(it);
//             self
//         }
//         #[inline(always)]
//         pub fn #set_name(&mut self, it: #var_typ) -> &mut Self {
//             self.#oneof_name = #oneof_type::#variant_name(it);
//             self
//         }
//
//     });
// }

// pub fn message_oneof(
//     &self,
//     out: &mut MessageParts,
//     file: &FileDef,
//     normal_field_count: usize,
//     _msg_id: u64,
//     _msg: &MessageDef,
//     msg_name: &Ident,
//     oneof_idx: usize,
//     oneof: &OneOfDef,
// ) {
//     let _oneof_idx = normal_field_count + oneof_idx;
//     let _oneof_bin_decoders: Vec<TokenStream> = vec![];
//
//     let oneof: &OneOfDef = oneof;
//     let oneof_raw_name = oneof.name.as_str();
//     let oneof_name = format_ident!("{}", rustify_name(oneof_raw_name));
//     let oneof_type = format_ident!("{msg_name}OneOf{}", oneof.name.as_str().to_case(Case::Pascal));
//
//     let mut variant_encoders = vec![];
//     let mut variant_text_encoders = vec![];
//
//     out.oneof_defs.push(self.generate_oneof(msg_name, oneof, &oneof_type));
//     out.fields.push(quote! { pub #oneof_name: #oneof_type });
//     for (_vname, var_num) in oneof.fields.by_name.iter() {
//         let variant = &oneof.fields.by_number[var_num];
//         self.oneof_variant(
//             out,
//             file,
//             &mut variant_encoders,
//             &mut variant_text_encoders,
//             oneof,
//             &oneof_name,
//             &oneof_type,
//             variant,
//         );
//     }
//     out.bin_encoders.push(quote! {
//         match &self.#oneof_name {
//             #(#variant_encoders)*
//             #oneof_type::Unknown(..) => {},
//         }
//     });
//     out.text_encoders.push(quote! {
//         match &self.#oneof_name {
//             #(#variant_text_encoders)*
//             #oneof_type::Unknown(..) => {},
//         }
//     });
// }

// pub fn generate_msg(
//     &self,
//     set: &FileSetDef,
//     file: &FileDef,
//     unit_id: DefId,
//     unit: &MessageDef,
// ) -> (TokenStream, Vec<Ident>, HashSet<usize>) {
//     let proto_name = unit.name.as_str();
//     let msg_name = format_ident!("{}", rustify_name(proto_name));
//     let pkg = &file.package;
//     let qualified_name = format!("{pkg}.{proto_name}");
//
//     let msg_types = vec![msg_name.clone()];
//     let mut out = MessageParts::default();
//
//     struct TmpField<'a> {
//         field_idx: usize,
//         tag_num: FieldNum,
//         def: &'a FieldDef,
//         pkg: Option<ArcStr>,
//         file: &'a FileDef,
//     }
//
//     let field_iter = unit
//         .fields
//         .by_number
//         .iter()
//         .enumerate()
//         .map(|(idx, (num, def))| TmpField {
//             field_idx: idx,
//             tag_num: *num,
//             def,
//             pkg: None,
//             file,
//         });
//
//     let ext_iter = file.extenders.get(&(unit_id as LocalDefId));
//     let ext_iter = ext_iter.iter().flat_map(|ext_ids| {
//         ext_ids.iter().flat_map(|ext_defid| {
//             let file_id = (ext_defid >> 32) as usize;
//             let (_ext_file_name, ext_file): (_, &FileDef) = set.files.get_index(file_id).unwrap();
//             let (_, ext_def): (_, &ExtendDef) = ext_file
//                 .extensions
//                 .get_index((*ext_defid as u32 & LOCAL_ONLY_ID) as usize)
//                 .unwrap();
//
//             ext_def
//                 .fields
//                 .by_number
//                 .iter()
//                 .enumerate()
//                 .map(|(idx, (num, def))| TmpField {
//                     field_idx: idx,
//                     tag_num: *num,
//                     def,
//                     pkg: Some(ext_file.package.clone()),
//                     file: ext_file,
//                 })
//         })
//     });
//
//     let mut normal_field_count = 0;
//     for TmpField {
//         field_idx,
//         tag_num: _,
//         def,
//         pkg,
//         file: field_file,
//     } in field_iter.chain(ext_iter)
//     {
//         self.message_field(
//             &mut out,
//             field_file,
//             unit_id,
//             unit,
//             &msg_name,
//             field_idx,
//             def,
//             pkg.as_ref(),
//         );
//         normal_field_count = field_idx;
//     }
//
//     for (oneof_idx, (_oneof_name, oneof)) in unit.oneofs.iter().enumerate() {
//         self.message_oneof(
//             &mut out,
//             file,
//             normal_field_count,
//             unit_id,
//             unit,
//             &msg_name,
//             oneof_idx,
//             oneof,
//         )
//     }
//
//     let MessageParts {
//         text_decoders,
//         text_encoders,
//         bin_decoders,
//         bin_encoders,
//         mut fields,
//         builders,
//         tabular_fields: _,
//         imports,
//         oneof_defs,
//     } = out;
//
//     // bin_encoders.reverse();
//
//     let text_format = if self.options.generate_textformat {
//         quote! {
//             impl textformat::Decodable for #msg_name {
//                 fn merge_field(&mut self, ctx: &textformat::Context, name: &textformat::ast::FieldName, value: &textformat::ast::FieldValue) -> textformat::Result<()> {
//                     match name {
//                         #(#text_decoders),*
//                         other => textformat::bail!("{other:?} was not recognized"),
//                     }
//                     Ok(())
//                 }
//             }
//             impl textformat::Encodable for #msg_name {
//                 fn encode(&self, ctx: &textformat::Context, pad: usize, out: &mut std::string::String) -> textformat::Result<()> {
//                     use binformat::IsPresent;
//                     #(#text_encoders)*
//                     Ok(())
//                 }
//             }
//         }
//     } else {
//         quote! {}
//     };
//
//     if self.options.track_unknowns {
//         fields.push(quote! { pub _unknown: binformat::UnknownFields })
//     } else {
//         fields.push(quote! { pub _unknown: () })
//     }
//
//     let result = quote! {
//         #[repr(C)]
//         #[derive( Debug, Default, Clone, PartialEq, )]
//         pub struct #msg_name  {
//             #(#fields,)*
//         }
//
//         impl #msg_name {
//             #(#builders)*
//         }
//
//         #text_format
//
//         impl binformat::Decodable for #msg_name {
//             fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: binformat::ReadBuffer<'b>) -> binformat::Result< binformat::ReadBuffer<'b>> {
//                 use binformat::format::*;
//                 match tag {
//                     #(#bin_decoders)*
//                     other => buf = self._unknown.merge_field(tag, buf)?,
//                 }
//                 Ok(buf)
//             }
//         }
//
//         impl binformat::Encodable for #msg_name {
//             fn qualified_name(&self) -> &'static str {
//                 #qualified_name
//             }
//             fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
//                 use binformat::format::*;
//                 use binformat::IsPresent;
//                 #(#bin_encoders)*
//                 binformat::Encodable::encode(&self._unknown, buf)?;
//                 Ok(())
//             }
//         }
//         #(#oneof_defs)*
//     };
//
//     (result, msg_types, imports.into_iter().map(|v| v as _).collect())
// }

pub fn generate_file(ctx: &TranslateCtx, opts: &Options, name: PathBuf, file_id: usize, file: &FileDef) -> Result<()> {
    let mut generator = CodeGenerator {
        context: ctx,
        options: opts,
        proto3: false,
        output: vec![],
    };

    generator.file(file_id, file)?;
    //
    // let root = opts.import_root.clone();
    //
    // let mut push_imports = HashSet::new();
    // let mut msg_types = vec![];
    // let mut messages = vec![];
    // let mut enums = vec![];
    // let mut services: Vec<TokenStream> = vec![];
    //
    // let generator = CodeGenerator {
    //     context: ctx,
    //     options: opts,
    // };
    //
    // for (idx, (_sym, msg)) in unit.messages.iter().enumerate() {
    //     let defid = (unit_id as u64) << 32 | LOCAL_DEFID_MSG as u64 | idx as u64;
    //     let (msg_contents, types, imports) = generator.generate_msg(&ctx.def, unit, defid, msg);
    //     push_imports.extend(imports);
    //     msg_types.extend(types.into_iter());
    //     messages.push(msg_contents);
    // }
    //
    // for (idx, (_name, en)) in unit.enums.iter().enumerate() {
    //     let _defid = (unit_id as u64) << 32 | LOCAL_DEFID_ENUM as u64 | idx as u64;
    //     enums.push(generator.generate_enum(en));
    // }
    //
    // for (_name, svc) in unit.services.iter() {
    //     services.push(generator.generate_server(unit, svc));
    //     services.push(generator.generate_client(unit, svc));
    // }
    //

    //
    // let output = quote! {
    //     #![allow(nonstandard_style)]
    //     #![allow(unused)]
    //     #![deny(unused_must_use)]
    //     #![allow(clippy::derive_partial_eq_without_eq)]
    //
    //     use std::fmt::Write;
    //
    //     use #root::*;
    //     use #root as root;
    //
    //     #(#imports)*
    //
    //
    //     #(#messages)*
    //     #(#enums)*
    //     #(#services)*
    // };

    let root = opts.import_root.clone();
    let imports = file.imports.iter().map(|imp| imp.file_idx).map(|file_idx| {
        let (_, other): (_, &FileDef) = ctx.def.files.get_index(file_idx).unwrap();
        let _our_name = name.file_name().unwrap().to_str().unwrap();

        if let Some(rep) = ctx.replacement.get(other.name.as_str()) {
            let rep = TokenStream::from_str(rep).unwrap();
            return quote! { use #rep::*; };
        }

        let their_name = if other.name.contains('/') {
            &other.name.as_str()[other.name.rfind('/').unwrap() + 1 ..]
        } else {
            other.name.as_str()
        };
        let their_name = if their_name.contains('.') {
            &their_name[.. their_name.rfind('.').unwrap()]
        } else {
            their_name
        };
        let mut our_module = file.package.as_str();
        let mut that_module = other.package.as_str();

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
    let output = generator.output;
    let output = quote! {
        use #root::*;

        pub fn register_types(registry: &mut #root::textformat::reflect::Registry) {
            // #(#files::register_types(registry);)*
        }

        #(#imports)*
        #(#output)*
    };
    let output = syn::parse2(output.clone()).with_context(|| output.to_string()).unwrap();
    let output = prettyplease::unparse(&output);
    println!("Creating file: {name:?}");

    create_dir_all(name.parent().unwrap()).unwrap();

    let mut f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .unwrap();

    f.write_all(output.as_bytes())?;
    f.flush()?;
    Ok(())
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
    let root = opts.import_root.clone();
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
        pub fn register_types(registry: &mut #root::textformat::reflect::Registry) {
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

    let f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Creating mod.rs in, {path:?}"));

    f
}
