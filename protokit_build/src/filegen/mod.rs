use core::ops::Deref;
use core::str::FromStr;
use std::collections::HashMap;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use protokit_desc::Syntax::Proto3;

use crate::deps::*;

pub mod grpc;

#[derive(Debug)]
pub struct Options {
    pub replacement: HashMap<String, String>,

    pub lifetime_arg: Option<TokenStream>,
    pub import_root: TokenStream,

    pub string_type: TokenStream,
    pub bytes_type: TokenStream,
    pub map_type: TokenStream,
    pub unknown_type: TokenStream,

    pub protoattrs: Vec<TokenStream>,

    pub track_unknowns: bool,
}

impl Options {
    pub fn replace_import(&mut self, from: &str, to: &str) -> &mut Self {
        self.replacement.insert(from.to_string(), to.to_string());
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            replacement: Default::default(),
            lifetime_arg: None,
            import_root: quote! { ::protokit },
            string_type: quote! { String },
            bytes_type: quote! { Vec<u8> },
            map_type: quote! { ::protokit::IndexMap },
            unknown_type: quote! { binformat::UnknownFieldsOwned },
            protoattrs: vec![],
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
    let pos = n.find('.');
    let n = if let Some(pos) = pos {
        &n[pos + 1..]
    } else {
        n
    };
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

    n.to_string()
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

pub struct CodeGenerator<'a> {
    context: &'a FileSetDef,
    options: &'a Options,
    output: Vec<TokenStream>,
}

impl CodeGenerator<'_> {
    pub fn resolve_name(&self, id: DefId) -> Result<String> {
        if let Some((msg, _)) = self.context.message_by_id(id) {
            return Ok(rustify_name(msg.name.as_str()));
        } else if let Some((en, _)) = self.context.enum_by_id(id) {
            return Ok(rustify_name(en.name.as_str()));
        } else {
            bail!(
                "Could not resolve {} {} {:b} files:{}: {:#?}",
                id,
                id >> 32,
                id & 0xFFFFFFFF,
                self.context.files.len(),
                self.context.files.keys()
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
        }).unwrap()
    }

    pub fn type_marker(&self, typ: &DataType) -> TokenStream {
        TokenStream::from_str(match typ {
            DataType::Unresolved(_) => panic!(),
            DataType::Builtin(b) => builtin_type_marker(*b),
            DataType::Message(m) => {
                let (m, _) = self.context.message_by_id(*m).unwrap();
                if m.is_group {
                    "group"
                } else {
                    "nested"
                }
            }
            DataType::Enum(_) => "protoenum",
            DataType::Map(k) => {
                return TokenStream::from_str(&format!(
                    "map({}, {})",
                    builtin_type_marker(k.0),
                    self.type_marker(&k.1)
                ))
                    .unwrap();
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
            DataType::Message(id) => {
                let borrow = self.borrow();
                let ident = format_ident!("{}", self.resolve_name(*id)?);

                quote! {#ident #borrow}
            }
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
        let is_msg = matches!(typ.typ, DataType::Message(..));

        match (typ.frequency, is_msg) {
            (Frequency::Singular | Frequency::Required, false) => Ok(base),
            (Frequency::Singular | Frequency::Required, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Optional, false) => Ok(quote!(Option<#base>)),
            (Frequency::Optional, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Repeated, _) => Ok(quote!(Vec<#base>)),
        }
    }

    pub fn borrow(&self) -> Option<TokenStream> {
        self.options.lifetime_arg.clone().map(|a| quote! { <#a> })
    }

    pub fn protoattrs(&self) -> TokenStream {
        if !self.options.protoattrs.is_empty() {
            let attrs = &self.options.protoattrs;
            quote! { #[proto(#(#attrs,)*)] }
        } else {
            quote! {}
        }
    }

    pub fn file(&mut self, f: &FileDef) -> Result<()> {
        for (_, en) in f.enums.iter() {
            self.r#enum(en)?;
        }
        for (name, msg) in f.messages.iter() {
            self.message(f, name, msg)?
        }


        for (_name, svc) in f.services.iter() {
            self.output.push(self.generate_server(f, svc));
            self.output.push(self.generate_client(f, svc));
        }

        Ok(())
    }

    pub fn message(&mut self, file: &FileDef, msg_name: &ArcStr, msg: &MessageDef) -> Result<()> {
        if msg.is_virtual_map {
            return Ok(())
        }
        let ident = format_ident!("{}", rustify_name(msg_name));
        let borrow = self.borrow();
        let attrs = self.protoattrs();

        let fields = msg
            .fields
            .by_number
            .iter()
            .map(|(_, f)| self.field(file, f))
            .collect::<Result<Vec<_>>>()?;

        let oneofs = msg
            .oneofs
            .iter()
            .map(|(_, def)| self.oneof(msg_name, def))
            .collect::<Result<Vec<_>>>()?;

        let last = if self.options.track_unknowns {
            let unk = &self.options.unknown_type;
            Some(quote! {
                #[unknown]
                pub unknown: #unk,
            })
        } else {
            None
        };

        self.output.push(quote! {
            #[derive(Debug, Default, Clone, PartialEq, Proto)]
            #attrs
            pub struct #ident #borrow {
                #(#fields,)*
                #(#oneofs,)*
                #last
            }
        });

        Ok(())
    }

    pub fn oneof(&mut self, msg_name: &str, def: &OneOfDef) -> Result<TokenStream> {
        let field_ident = format_ident!("{}", rustify_name(&def.name));
        let oneof_type = format_ident!("{msg_name}OneOf{}", def.name.as_str().to_case(Case::Pascal));
        let borrow = self.borrow();
        let borrow_or_static = self.options.lifetime_arg.clone().unwrap_or_else(|| quote! { 'static });
        let attrs = self.protoattrs();

        let mut nums = vec![];
        let mut names = vec![];
        let mut vars = vec![];

        let mut default = None;

        for (_, var) in def.fields.by_number.iter() {
            let var_name = format_ident!("{}", var.name.as_str().to_case(Case::Pascal));
            let typ = self.base_type(&var.typ)?;
            let num = var.num as u32;
            let name = var.name.as_str();
            let kind = self.type_marker(&var.typ);

            vars.push(quote! {
                #[field(#num, #name, #kind, raw)]
                #var_name(#typ),
            });

            if default.is_none() {
                default = Some(quote! {
                    impl #borrow Default for #oneof_type #borrow {
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
            #[derive(Debug, Clone, PartialEq, Proto)]
            #attrs
            pub enum #oneof_type #borrow {
                #(#vars)*
                __Unused(::core::marker::PhantomData< & #borrow_or_static ()>),
            }
            #default
        });

        Ok(quote! {
            #[oneof([#(#nums,)*], [#(#names,)*])]
            pub #field_ident: Option<#oneof_type #borrow>
        })
    }

    pub fn r#enum(&mut self, def: &EnumDef) -> Result<()> {
        let ident = format_ident!("{}", rustify_name(def.name.as_str()));
        let variants = def.variants.by_name.iter().map(|(_, def)| {
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

    pub fn field(&self, file: &FileDef, def: &FieldDef) -> Result<TokenStream> {
        let typ = self.field_type(def)?;
        let fname = format_ident!("{}", rustify_name(def.name.as_str()));
        let name = def.name.as_str();
        let num = def.num as u32;

        let kind = self.type_marker(&def.typ);
        let freq = TokenStream::from_str(match def.frequency {
            Frequency::Singular if def.typ.is_message() => "optional",
            Frequency::Singular => "singular",
            Frequency::Optional => "optional",
            Frequency::Repeated if def.is_packed() => "packed",
            Frequency::Repeated if file.syntax == Proto3 && def.typ.is_scalar() => "packed",
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

pub fn generate_file(ctx: &FileSetDef, opts: &Options, name: PathBuf, file: &FileDef) -> Result<()> {
    let mut generator = CodeGenerator {
        context: ctx,
        options: opts,
        output: vec![],
    };

    generator.file(file)?;
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
    //     use core::fmt::Write;
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
        let (_, other): (_, &FileDef) = ctx.files.get_index(file_idx).unwrap();
        let _our_name = name.file_name().unwrap().to_str().unwrap();

        // if let Some(rep) = ctx.replacement.get(other.name.as_str()) {
        //     let rep = TokenStream::from_str(rep).unwrap();
        //     return quote! { use #rep::*; };
        // }

        let their_name = if other.name.contains('/') {
            &other.name.as_str()[other.name.rfind('/').unwrap() + 1..]
        } else {
            other.name.as_str()
        };
        let their_name = if their_name.contains('.') {
            &their_name[..their_name.rfind('.').unwrap()]
        } else {
            their_name
        };
        let mut our_module = file.package.as_str();
        let mut that_module = other.package.as_str();

        while !our_module.is_empty() && !that_module.is_empty() && our_module[..1] == that_module[..1] {
            our_module = &our_module[1..];
            that_module = &that_module[1..];
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
        #![allow(unused_imports)]
        #![allow(nonstandard_style)]
        #![allow(unreachable_patterns)]
        use #root::*;

        pub fn register_types(_registry: &mut #root::textformat::reflect::Registry) {
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

// #[cfg(feature = "descriptors")]
// pub fn generate_descriptor(ctx: &TranslateCtx, name: impl AsRef<Path>) {
//     let mut output = vec![];
//     ctx.def.to_descriptor().encode(&mut output).unwrap();
//
//     let mut f = make_file(name);
//
//     f.write_all(&output).unwrap();
//     f.flush().unwrap();
// }

pub fn generate_mod<'s>(path: impl AsRef<Path>, opts: &Options, files: impl Iterator<Item=&'s str>) -> Result<()> {
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

    create_dir_all(path.as_ref().parent().unwrap())?;

    let mut f = make_file(path.as_ref().join("mod.rs"))?;

    let output = syn::parse2(output)?;
    let output = prettyplease::unparse(&output);
    f.write_all(output.as_bytes())?;
    f.flush()?;

    Ok(())
}

pub fn make_file(path: impl AsRef<Path>) -> Result<std::fs::File> {
    let path = path.as_ref();

    let f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    Ok(f)
}
