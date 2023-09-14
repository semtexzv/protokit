use core::ops::Deref;
use core::str::FromStr;
use std::collections::BTreeMap;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use protokit_desc::Syntax::{Proto2, Proto3};
use quote::{format_ident, quote};

use crate::deps::*;

pub mod grpc;

#[derive(Debug, Default)]
pub struct Generics {
    pub buf_arg: Option<TokenStream>,
    pub alloc_arg: Option<TokenStream>,
}

impl Generics {
    fn liftetime_arg(&self) -> Option<TokenStream> {
        self.buf_arg.clone()
    }
    fn struct_def_generics(&self) -> TokenStream {
        match (&self.buf_arg, &self.alloc_arg) {
            (Some(b), Some(a)) => quote! { < #b, #a : std::alloc::Allocator + Debug> },
            (Some(b), None) => quote! { <#b> },
            (None, Some(a)) => quote! { <#a: std::alloc::Allocator > },
            _ => quote! {},
        }
    }

    fn struct_use_generics(&self) -> TokenStream {
        match (&self.buf_arg, &self.alloc_arg) {
            (Some(b), Some(a)) => quote! { < #b, #a> },
            (Some(b), None) => quote! { <#b> },
            (None, Some(a)) => quote! { <#a> },
            _ => quote! {},
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub replacement: BTreeMap<String, String>,
    pub import_root: TokenStream,

    pub string_type: TokenStream,
    pub bytes_type: TokenStream,
    pub map_type: TokenStream,
    pub unknown_type: TokenStream,

    pub generics: Generics,
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
            import_root: quote! { ::protokit },
            string_type: quote! { String },
            bytes_type: quote! { Vec<u8> },
            map_type: quote! { ::protokit::IndexMap },
            unknown_type: quote! { ::protokit::binformat::UnknownFieldsOwned },
            generics: Generics::default(),
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
    // let pos = n.find('.');
    // let n = if let Some(pos) = pos {
    //     &n[pos + 1..]
    // } else {
    //     n
    // };
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

pub struct CodeGenerator<'a> {
    context: &'a FileSetDef,
    options: &'a Options,
    types: Vec<TokenStream>,
    output: Vec<TokenStream>,
}

impl CodeGenerator<'_> {
    pub fn resolve_name(&self, id: GlobalDefId) -> Result<String> {
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
        })
            .unwrap()
    }

    pub fn type_marker(typ: &DataType) -> TokenStream {
        TokenStream::from_str(match typ {
            DataType::Unresolved(_, _) => panic!(),
            DataType::Builtin(b) => builtin_type_marker(*b),
            DataType::Message(_) => "nested",
            DataType::Group(_) => "group",
            DataType::Enum(_) => "protoenum",

            DataType::Map(k) => {
                eprintln!("{:?} to {:?}", typ, k);
                return TokenStream::from_str(&format!(
                    "map({}, {})",
                    builtin_type_marker(k.0),
                    Self::type_marker(&k.1)
                ))
                    .unwrap();
            }
        })
            .unwrap()
    }

    pub fn base_type(&self, typ: &DataType) -> Result<TokenStream> {
        Ok(match typ {
            DataType::Unresolved(path, _) => {
                panic!("Name {path} was not resolved to actual type")
            }
            DataType::Builtin(bt) => return Ok(self.builtin_rusttype(*bt)),
            DataType::Message(id) | DataType::Group(id) => {
                // let borrow = self.borrow();
                let gen = self.options.generics.struct_use_generics();
                let ident = format_ident!("{}", self.resolve_name(*id)?);

                quote! {#ident #gen}
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

    pub fn field_type(&self, field: &FieldDef) -> Result<TokenStream> {
        let base = self.base_type(&field.typ)?;
        let is_msg = matches!(field.typ, DataType::Message(..) | DataType::Group(_));

        match (field.frequency, is_msg) {
            (Frequency::Singular | Frequency::Required, false) => Ok(base),
            (Frequency::Required, true) => Ok(quote!(Box<#base>)),
            (Frequency::Singular, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Optional, false) => Ok(quote!(Option<#base>)),
            (Frequency::Optional, true) => Ok(quote!(Option<Box<#base>>)),
            (Frequency::Repeated, _) => Ok(quote!(Vec<#base>)),
        }
    }

    // pub fn protoattrs(&self, msg: &MessageDef) -> TokenStream {
    //     if !self.options.protoattrs.is_empty() {
    //         let attrs = &self.options.protoattrs;
    //         quote! { #[proto(#(#attrs,)*)] }
    //     } else {
    //         quote! {}
    //     }
    // }

    pub fn file(&mut self, f: &FileDef) -> Result<()> {
        for (_, en) in f.enums.iter() {
            self.r#enum(f, en)?;
        }

        for (i, (name, msg)) in f.messages.iter().enumerate() {
            self.message(f, i as u32, name, msg)?
        }

        for (_name, svc) in f.services.iter() {
            self.output.push(self.generate_server(f, svc));
            self.output.push(self.generate_client(f, svc));
        }

        Ok(())
    }

    pub fn message(&mut self, file: &FileDef, msg_idx: u32, msg_name: &ArcStr, msg: &MessageDef) -> Result<()> {
        if msg.is_virtual_map {
            return Ok(());
        }

        let mut extfields = vec![];
        for (def, ext) in &file.extenders {
            if (def & LOCAL_ONLY_ID) == msg_idx {
                for ext in ext {
                    let (ext, extfile) = self.context.ext_by_id(*ext).unwrap();
                    extfields.extend(ext.fields.by_number.values().map(|f| (f, extfile.package.as_str())))
                }
            }
        }

        let ident = format_ident!("{}", rustify_name(msg_name));
        // let borrow = self.borrow();
        let generics = self.options.generics.struct_def_generics();
        let name = msg.name.as_str();
        let pkg = file.package.as_str();
        let attrs = quote!{
            #[proto(name = #name, package = #pkg)]
        };

        let extfields = extfields.into_iter()
            .map(|(f, pkg)| self.field(file, f, Some(pkg)));

        let fields = msg
            .fields
            .by_number
            .values()
            .map(|f| self.field(file, f, None));

        let fields = fields
            .chain(extfields)
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

        self.types.push(quote! {#ident});
        self.output.push(quote! {
            #[derive(Debug, Default, Clone, PartialEq, Proto)]
            #attrs
            pub struct #ident #generics {
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
        // let borrow = self.borrow();
        let generics = self.options.generics.struct_def_generics();
        let use_generics = self.options.generics.struct_use_generics();

        let borrow_or_static = self
            .options
            .generics
            .liftetime_arg()
            .unwrap_or_else(|| quote! { 'static });

        let mut nums = vec![];
        let mut names = vec![];
        let mut vars = vec![];

        let mut default = None;

        for (_, var) in def.fields.by_number.iter() {
            let var_name = format_ident!("{}", var.name.as_str().to_case(Case::Pascal));
            let typ = self.base_type(&var.typ)?;
            let num = var.num as u32;
            let name = var.name.as_str();
            let kind = Self::type_marker(&var.typ);

            vars.push(quote! {
                #[field(#num, #name, #kind, raw)]
                #var_name(#typ),
            });

            if default.is_none() {
                default = Some(quote! {
                    impl #generics Default for #oneof_type #use_generics {
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
            // #attrs
            pub enum #oneof_type #generics {
                #(#vars)*
                __Unused(::core::marker::PhantomData<& #borrow_or_static ()>),
            }
            #default
        });

        Ok(quote! {
            #[oneof([#(#nums,)*], [#(#names,)*])]
            pub #field_ident: Option<#oneof_type #use_generics>
        })
    }

    pub fn r#enum(&mut self, _: &FileDef, def: &EnumDef) -> Result<()> {
        let ident = format_ident!("{}", rustify_name(def.name.as_str()));
        // let open = if file.syntax == Proto3 {
        //     quote!{ open }
        // } else {
        //     quote!{ closed }
        // };
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

    pub fn field(&self, file: &FileDef, def: &FieldDef, extpkg: Option<&str>) -> Result<TokenStream> {
        let typ = self.field_type(def)?;
        let fname = format_ident!("{}", rustify_name(def.name.as_str()));
        let name = match extpkg {
            Some(pkg) => format!("[{}.{}]", pkg, def.name),
            None => def.name.to_string(),
        };

        if let DataType::Enum(id) = def.typ {
            if let Some((en, efile)) = self.context.enum_by_id(id) {
                if file.syntax == Proto3 && efile.syntax == Proto2 {
                    panic!("Can't use proto2 enum ({}) in proto3 file", en.name);
                }
            }
        }

        let num = def.num as u32;

        let kind = Self::type_marker(&def.typ);
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
        types: vec![],
        output: vec![],
    };

    generator.file(file)?;

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
    let types = generator.types;

    let output = quote! {
        #![allow(unused_imports)]
        #![allow(nonstandard_style)]
        #![allow(unreachable_patterns)]
        #![allow(clippy::module_inception)]
        use #root::*;

        pub fn register_types(registry: &mut #root::textformat::reflect::Registry) {
            #(registry.register(&#types::default());)*
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

    let f = File::options().write(true).create(true).truncate(true).open(path)?;

    Ok(f)
}
