use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    DescriptorPool, DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
    FieldDescriptorProtoLabel, FieldDescriptorProtoType, FileDescriptorProto,
    OneofDescriptorProto, ServiceDescriptorProto,
};

pub mod grpc;

#[derive(Debug, Default)]
pub struct Generics {
    pub buf_arg: Option<TokenStream>,
    pub alloc_arg: Option<TokenStream>,
}

impl Generics {
    fn lifetime_arg(&self) -> Option<TokenStream> {
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
    pub import_root: Option<TokenStream>,

    pub string_type: TokenStream,
    pub bytes_type: TokenStream,
    pub map_type: TokenStream,
    pub unknown_type: TokenStream,

    pub generics: Generics,
    pub protoattrs: Vec<TokenStream>,

    pub track_unknowns: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            import_root: None,
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
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

const RESERVED: &[&str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
    "unsized", "virtual", "yield",
];

pub const TYPES: &[&str] = &["Option", "Result"];

pub fn rustify_name(n: impl AsRef<str>) -> String {
    let n = n.as_ref().replace('.', "_");
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

/// Extract the simple name from a fully qualified type name
/// e.g. ".google.protobuf.Any" -> "Any"
fn simple_name(fqn: &str) -> &str {
    fqn.rsplit('.').next().unwrap_or(fqn)
}

/// Get the Rust type name from a fully qualified protobuf type name
fn rust_type_name(pool: &DescriptorPool, type_name: &str) -> String {
    if let Some(loc) = pool.lookup(type_name) {
        // Use the path to construct nested type names
        rustify_name(loc.path.join("_"))
    } else {
        // Fallback to simple name
        rustify_name(simple_name(type_name))
    }
}

fn builtin_type_marker(typ: FieldDescriptorProtoType) -> &'static str {
    match typ {
        FieldDescriptorProtoType::TYPE_INT32 => "varint",
        FieldDescriptorProtoType::TYPE_INT64 => "varint",
        FieldDescriptorProtoType::TYPE_UINT32 => "varint",
        FieldDescriptorProtoType::TYPE_UINT64 => "varint",
        FieldDescriptorProtoType::TYPE_SINT32 => "sigint",
        FieldDescriptorProtoType::TYPE_SINT64 => "sigint",
        FieldDescriptorProtoType::TYPE_BOOL => "bool",
        FieldDescriptorProtoType::TYPE_FIXED64 => "fixed64",
        FieldDescriptorProtoType::TYPE_SFIXED64 => "fixed64",
        FieldDescriptorProtoType::TYPE_FIXED32 => "fixed32",
        FieldDescriptorProtoType::TYPE_SFIXED32 => "fixed32",
        FieldDescriptorProtoType::TYPE_DOUBLE => "fixed64",
        FieldDescriptorProtoType::TYPE_FLOAT => "fixed32",
        FieldDescriptorProtoType::TYPE_STRING => "string",
        FieldDescriptorProtoType::TYPE_BYTES => "bytes",
        FieldDescriptorProtoType::TYPE_MESSAGE => "nested",
        FieldDescriptorProtoType::TYPE_GROUP => "group",
        FieldDescriptorProtoType::TYPE_ENUM => "protoenum",
        _ => panic!("Unknown field type"),
    }
}

fn builtin_rust_type(opts: &Options, typ: FieldDescriptorProtoType) -> TokenStream {
    match typ {
        FieldDescriptorProtoType::TYPE_INT32 => quote! { i32 },
        FieldDescriptorProtoType::TYPE_INT64 => quote! { i64 },
        FieldDescriptorProtoType::TYPE_UINT32 => quote! { u32 },
        FieldDescriptorProtoType::TYPE_UINT64 => quote! { u64 },
        FieldDescriptorProtoType::TYPE_SINT32 => quote! { i32 },
        FieldDescriptorProtoType::TYPE_SINT64 => quote! { i64 },
        FieldDescriptorProtoType::TYPE_BOOL => quote! { bool },
        FieldDescriptorProtoType::TYPE_FIXED64 => quote! { u64 },
        FieldDescriptorProtoType::TYPE_SFIXED64 => quote! { i64 },
        FieldDescriptorProtoType::TYPE_FIXED32 => quote! { u32 },
        FieldDescriptorProtoType::TYPE_SFIXED32 => quote! { i32 },
        FieldDescriptorProtoType::TYPE_DOUBLE => quote! { f64 },
        FieldDescriptorProtoType::TYPE_FLOAT => quote! { f32 },
        FieldDescriptorProtoType::TYPE_STRING => opts.string_type.clone(),
        FieldDescriptorProtoType::TYPE_BYTES => opts.bytes_type.clone(),
        _ => panic!("Not a builtin type"),
    }
}

fn is_scalar_type(typ: FieldDescriptorProtoType) -> bool {
    !matches!(
        typ,
        FieldDescriptorProtoType::TYPE_STRING
            | FieldDescriptorProtoType::TYPE_BYTES
            | FieldDescriptorProtoType::TYPE_MESSAGE
            | FieldDescriptorProtoType::TYPE_GROUP
    )
}

pub struct CodeGenerator<'a> {
    pool: &'a DescriptorPool,
    file: &'a FileDescriptorProto,
    options: &'a Options,
    types: Vec<TokenStream>,
    output: Vec<TokenStream>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(pool: &'a DescriptorPool, file_idx: usize, options: &'a Options) -> Self {
        Self {
            pool,
            file: &pool.descriptor_set.file[file_idx],
            options,
            types: vec![],
            output: vec![],
        }
    }

    fn is_proto3(&self) -> bool {
        self.file.syntax.as_deref() == Some("proto3")
    }

    fn package(&self) -> &str {
        self.file.package.as_deref().unwrap_or("")
    }

    fn base_type(&self, field: &FieldDescriptorProto) -> Result<TokenStream> {
        let typ = field.r#type.as_ref().unwrap();

        if let Some(type_name) = &field.type_name {
            // Message, Enum, or Group type
            let rust_name = rust_type_name(self.pool, type_name);
            let ident = format_ident!("{}", rust_name);
            let gen = self.options.generics.struct_use_generics();
            Ok(quote! { #ident #gen })
        } else {
            // Builtin type
            Ok(builtin_rust_type(self.options, *typ))
        }
    }

    fn is_map_field(&self, field: &FieldDescriptorProto) -> Option<(&DescriptorProto, &FieldDescriptorProto, &FieldDescriptorProto)> {
        if field.label != Some(FieldDescriptorProtoLabel::LABEL_REPEATED) {
            return None;
        }

        let type_name = field.type_name.as_ref()?;
        let msg = self.pool.get_message(type_name)?;

        // Check if it's a map entry
        if msg.options.as_ref().and_then(|o| o.map_entry) != Some(true) {
            return None;
        }

        // Get key and value fields
        let key_field = msg.field.iter().find(|f| f.number == Some(1))?;
        let value_field = msg.field.iter().find(|f| f.number == Some(2))?;

        Some((msg, key_field, value_field))
    }

    fn field_type(&self, field: &FieldDescriptorProto) -> Result<TokenStream> {
        let typ = field.r#type.as_ref().unwrap();
        let is_msg = matches!(
            typ,
            &FieldDescriptorProtoType::TYPE_MESSAGE | &FieldDescriptorProtoType::TYPE_GROUP
        );
        let is_repeated = field.label == Some(FieldDescriptorProtoLabel::LABEL_REPEATED);

        // Check if this is a map field
        if let Some((_, key_field, value_field)) = self.is_map_field(field) {
            let kt = self.base_type(key_field)?;
            let vt = self.base_type(value_field)?;
            let mt = &self.options.map_type;
            return Ok(quote! { #mt<#kt, #vt> });
        }

        let base = self.base_type(field)?;

        let force_box = field
            .type_name
            .as_ref()
            .map(|t| self.pool.is_boxed(t))
            .unwrap_or(false);

        let is_optional = field.label == Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL);
        let is_required = field.label == Some(FieldDescriptorProtoLabel::LABEL_REQUIRED);
        let is_proto3_optional = field.proto3_optional == Some(true);

        match (is_repeated, is_optional || is_proto3_optional, is_required, is_msg, force_box) {
            // Repeated field -> Vec
            (true, _, _, _, _) => Ok(quote!(Vec<#base>)),

            // Required message with boxing
            (false, _, true, true, true) => Ok(quote!(Option<Box<#base>>)),
            // Required message without boxing
            (false, _, true, true, false) => Ok(quote!(#base)),
            // Required non-message
            (false, _, true, false, _) => Ok(quote!(#base)),

            // Optional message with boxing
            (false, true, _, true, true) => Ok(quote!(Option<Box<#base>>)),
            // Optional message without boxing
            (false, true, _, true, false) => Ok(quote!(Option<#base>)),
            // Optional non-message
            (false, true, _, false, _) => Ok(quote!(Option<#base>)),

            // Singular (proto3 implicit presence) message with boxing
            (false, false, false, true, true) => Ok(quote!(Option<Box<#base>>)),
            // Singular message without boxing
            (false, false, false, true, false) => Ok(quote!(Option<#base>)),
            // Singular non-message (proto3)
            (false, false, false, false, _) => Ok(base),
        }
    }

    fn type_marker(&self, field: &FieldDescriptorProto) -> TokenStream {
        let typ = field.r#type.as_ref().unwrap();

        // Check for map fields
        if let Some((_, key_field, value_field)) = self.is_map_field(field) {
            let key_typ = key_field.r#type.as_ref().unwrap();
            let key_marker = builtin_type_marker(*key_typ);

            let val_typ = value_field.r#type.as_ref().unwrap();
            let val_marker = builtin_type_marker(*val_typ);

            let marker_str = format!("map({}, {})", key_marker, val_marker);
            return core::str::FromStr::from_str(&marker_str).unwrap();
        }

        let marker = builtin_type_marker(*typ);
        core::str::FromStr::from_str(marker).unwrap()
    }

    fn field_frequency(&self, field: &FieldDescriptorProto) -> TokenStream {
        let typ = field.r#type.as_ref().unwrap();
        let is_msg = matches!(
            typ,
            &FieldDescriptorProtoType::TYPE_MESSAGE | &FieldDescriptorProtoType::TYPE_GROUP
        );
        let is_scalar = is_scalar_type(*typ);
        let is_enum = *typ == FieldDescriptorProtoType::TYPE_ENUM;
        let force_box = field
            .type_name
            .as_ref()
            .map(|t| self.pool.is_boxed(t))
            .unwrap_or(false);

        // Check if packed
        let is_packed = field
            .options
            .as_ref()
            .and_then(|o| o.packed)
            .unwrap_or(false);

        let is_repeated = field.label == Some(FieldDescriptorProtoLabel::LABEL_REPEATED);
        let is_optional = field.label == Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL);
        let is_required = field.label == Some(FieldDescriptorProtoLabel::LABEL_REQUIRED);
        let is_proto3_optional = field.proto3_optional == Some(true);

        // Handle map fields specially - they're singular even though they're repeated messages
        if self.is_map_field(field).is_some() {
            return quote! { singular };
        }

        let freq = match (is_repeated, is_optional, is_required, is_msg, force_box) {
            (true, _, _, _, _) if is_packed => "packed",
            (true, _, _, _, _) if self.is_proto3() && (is_scalar || is_enum) => "packed",
            (true, _, _, _, _) => "repeated",
            (false, true, _, _, _) => "optional",
            (false, _, _, true, _) if is_proto3_optional => "optional",
            (false, false, false, true, _) => "optional",
            (false, false, false, false, _) if is_proto3_optional => "optional",
            (false, false, false, false, _) => "singular",
            (false, _, true, _, true) => "optional",
            (false, _, true, _, _) => "required",
        };

        core::str::FromStr::from_str(freq).unwrap()
    }

    pub fn generate(&mut self) -> Result<()> {
        // Generate enums first (they may be referenced by messages)
        for enum_desc in &self.file.enum_type {
            self.generate_enum(enum_desc, &[])?;
        }

        // Generate messages
        for msg_desc in &self.file.message_type {
            self.generate_message(msg_desc, &[])?;
        }

        // Generate services
        for svc_desc in &self.file.service {
            self.generate_service(svc_desc)?;
        }

        Ok(())
    }

    fn generate_enum(&mut self, desc: &EnumDescriptorProto, parent_path: &[&str]) -> Result<()> {
        let name = desc.name.as_deref().unwrap_or("");
        let full_name = if parent_path.is_empty() {
            name.to_string()
        } else {
            format!("{}_{}", parent_path.join("_"), name)
        };

        let ident = format_ident!("{}", rustify_name(&full_name));

        let open = if self.is_proto3() {
            quote! { open }
        } else {
            quote! { closed }
        };

        let variants = desc.value.iter().map(|v| {
            let var_name = v.name.as_deref().unwrap_or("");
            let var_ident = format_ident!("{}", var_name);
            let num = v.number.unwrap_or(0);
            quote! {
                #[var(#num, #var_name)]
                pub const #var_ident: #ident = #ident(#num);
            }
        });

        self.output.push(quote! {
            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct #ident(pub i32);
            #[protoenum(#open)]
            impl #ident {
                #(#variants)*
            }
        });

        Ok(())
    }

    fn generate_message(&mut self, desc: &DescriptorProto, parent_path: &[&str]) -> Result<()> {
        let name = desc.name.as_deref().unwrap_or("");

        // Skip map entry types - they're synthetic
        if desc.options.as_ref().and_then(|o| o.map_entry) == Some(true) {
            return Ok(());
        }

        let mut path = parent_path.to_vec();
        path.push(name);

        // Generate nested enums
        for enum_desc in &desc.enum_type {
            self.generate_enum(enum_desc, &path)?;
        }

        // Generate nested messages
        for nested in &desc.nested_type {
            self.generate_message(nested, &path)?;
        }

        let full_name = path.join("_");
        let ident = format_ident!("{}", rustify_name(&full_name));
        let generics = self.options.generics.struct_def_generics();
        let pkg = self.package();

        let attrs = quote! {
            #[proto(name = #name, package = #pkg)]
        };

        // Collect fields that are part of oneofs
        let mut oneof_field_indices: HashSet<i32> = HashSet::new();
        for oneof in &desc.oneof_decl {
            let oneof_name = oneof.name.as_deref().unwrap_or("");
            for field in &desc.field {
                if let Some(idx) = field.oneof_index {
                    if desc.oneof_decl.get(idx as usize).and_then(|o| o.name.as_deref()) == Some(oneof_name) {
                        // Don't add proto3 optional synthetic oneofs to the set
                        if field.proto3_optional != Some(true) {
                            oneof_field_indices.insert(field.number.unwrap_or(0));
                        }
                    }
                }
            }
        }

        // Generate regular fields (excluding oneof members)
        let fields = desc
            .field
            .iter()
            .filter(|f| {
                // Include if not part of a oneof, or if it's a proto3 optional synthetic oneof
                if f.oneof_index.is_some() {
                    // proto3 optional creates a synthetic oneof - include it as a regular field
                    f.proto3_optional == Some(true)
                        || !oneof_field_indices.contains(&f.number.unwrap_or(0))
                } else {
                    true
                }
            })
            .map(|f| self.generate_field(f))
            .collect::<Result<Vec<_>>>()?;

        // Generate oneofs (excluding proto3 optional synthetic oneofs)
        let oneofs = desc
            .oneof_decl
            .iter()
            .enumerate()
            .filter(|(idx, _oneof)| {
                // Skip synthetic oneofs for proto3 optional
                !desc.field.iter().any(|f| {
                    f.oneof_index == Some(*idx as i32) && f.proto3_optional == Some(true)
                })
            })
            .map(|(idx, oneof)| self.generate_oneof(&full_name, idx, oneof, &desc.field))
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

        self.types.push(quote! { #ident });
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

    fn generate_field(&self, field: &FieldDescriptorProto) -> Result<TokenStream> {
        let name = field.name.as_deref().unwrap_or("");
        let fname = format_ident!("{}", rustify_name(name));
        let num = field.number.unwrap_or(0) as u32;
        let typ = self.field_type(field)?;
        let kind = self.type_marker(field);
        let freq = self.field_frequency(field);

        Ok(quote! {
            #[field(#num, #name, #kind, #freq)]
            pub #fname: #typ
        })
    }

    fn generate_oneof(
        &mut self,
        msg_name: &str,
        oneof_idx: usize,
        oneof: &OneofDescriptorProto,
        all_fields: &[FieldDescriptorProto],
    ) -> Result<TokenStream> {
        let oneof_name = oneof.name.as_deref().unwrap_or("");
        let field_ident = format_ident!("{}", rustify_name(oneof_name));
        let oneof_type = format_ident!(
            "{}OneOf{}",
            rustify_name(msg_name),
            oneof_name.to_case(Case::Pascal)
        );

        let generics = self.options.generics.struct_def_generics();
        let use_generics = self.options.generics.struct_use_generics();

        let borrow_or_static = self
            .options
            .generics
            .lifetime_arg()
            .unwrap_or_else(|| quote! { 'static });

        let mut nums = vec![];
        let mut names = vec![];
        let mut vars = vec![];
        let mut default = None;

        for field in all_fields {
            if field.oneof_index != Some(oneof_idx as i32) {
                continue;
            }
            // Skip proto3 optional synthetic oneofs
            if field.proto3_optional == Some(true) {
                continue;
            }

            let field_name = field.name.as_deref().unwrap_or("");
            let var_name = format_ident!("{}", field_name.to_case(Case::Pascal));
            let typ = self.base_type(field)?;
            let num = field.number.unwrap_or(0) as u32;
            let kind = self.type_marker(field);

            vars.push(quote! {
                #[field(#num, #field_name, #kind, raw)]
                #var_name(#typ),
            });

            if default.is_none() {
                default = Some(quote! {
                    impl #generics Default for #oneof_type #use_generics {
                        fn default() -> Self {
                            Self::#var_name(Default::default())
                        }
                    }
                });
            }

            nums.push(num);
            names.push(field_name);
        }

        self.output.push(quote! {
            #[derive(Debug, Clone, PartialEq, Proto)]
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

    fn generate_service(&mut self, svc: &ServiceDescriptorProto) -> Result<()> {
        self.output.push(self.generate_server(svc));
        self.output.push(self.generate_client(svc));
        Ok(())
    }

    fn generate_server(&self, svc: &ServiceDescriptorProto) -> TokenStream {
        grpc::generate_server(self.pool, self.options, svc)
    }

    fn generate_client(&self, svc: &ServiceDescriptorProto) -> TokenStream {
        grpc::generate_client(self.pool, self.options, svc)
    }
}

pub fn generate_file(
    pool: &DescriptorPool,
    opts: &Options,
    path: impl AsRef<Path>,
    file_idx: usize,
) -> Result<()> {
    let file = &pool.descriptor_set.file[file_idx];
    let mut generator = CodeGenerator::new(pool, file_idx, opts);
    generator.generate()?;

    // Generate extension registrations
    let mut extensions = vec![];
    for ext in &file.extension {
        let extendee = ext.extendee.as_deref().unwrap_or("");
        let name = ext.name.as_deref().unwrap_or("");
        let number = ext.number.unwrap_or(0) as u32;
        let typ = ext.r#type.map(|t| t.0 as u32).unwrap_or(0);
        let repeated = ext.label == Some(FieldDescriptorProtoLabel::LABEL_REPEATED);

        let type_name_str = ext.type_name.as_deref().map(|t| rust_type_name(pool, t)).unwrap_or_default();
        let type_name = type_name_str.as_str();

        extensions.push(quote! {
            registry.register_extension(#extendee, #number, #name, #typ, #repeated, #type_name);
        });
    }

    // Generate imports
    let imports = file.dependency.iter().map(|dep_name| {
        // Find the file in the descriptor set
        let dep_file = pool.descriptor_set.file.iter().find(|f| f.name.as_deref() == Some(dep_name));

        if let Some(dep_file) = dep_file {
            let dep_package = dep_file.package.as_deref().unwrap_or("");
            let our_package = file.package.as_deref().unwrap_or("");

            let their_name = if dep_name.contains('/') {
                &dep_name[dep_name.rfind('/').unwrap() + 1..]
            } else {
                dep_name.as_str()
            };
            let their_name = if their_name.contains('.') {
                &their_name[..their_name.rfind('.').unwrap()]
            } else {
                their_name
            };

            // Calculate relative path
            let mut our_module = our_package;
            let mut that_module = dep_package;

            while !our_module.is_empty() && !that_module.is_empty() {
                if our_module.chars().next() == that_module.chars().next() {
                    our_module = &our_module[1..];
                    that_module = &that_module[1..];
                } else {
                    break;
                }
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
                    path.push_str("::");
                }
            }
            path.push_str(&rustify_name(their_name));

            let import: TokenStream = core::str::FromStr::from_str(&path).unwrap();
            quote! {
                use #import::*;
            }
        } else {
            quote! {}
        }
    });

    let root = opts.import_root.clone();
    let output = generator.output;
    let types = generator.types;

    let maproot = if let Some(ref root) = root {
        quote! { use #root as protokit; }
    } else {
        quote! {}
    };

    let output = quote! {
        #![allow(unused_imports)]
        #![allow(nonstandard_style)]
        #![allow(unreachable_patterns)]
        #![allow(clippy::module_inception)]

        #maproot
        use protokit::*;

        pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
            #(registry.register(&#types::default());)*
            #(#extensions)*
        }

        #(#imports)*
        #(#output)*
    };

    let output = syn::parse2(output.clone())
        .with_context(|| output.to_string())
        .unwrap();
    let output = prettyplease::unparse(&output);

    let path = path.as_ref();
    create_dir_all(path.parent().unwrap())?;

    let mut f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    f.write_all(output.as_bytes())?;
    f.flush()?;

    Ok(())
}

pub fn generate_mod<'s>(
    path: impl AsRef<Path>,
    opts: &Options,
    files: impl Iterator<Item = &'s str>,
) -> Result<()> {
    let root = opts.import_root.clone();
    let files: Vec<_> = files
        .map(|v| {
            let v = v.strip_suffix(".rs").unwrap_or(v);
            format_ident!("{}", rustify_name(v))
        })
        .collect();

    let maproot = if let Some(ref root) = root {
        quote! { use #root as protokit; }
    } else {
        quote! {}
    };

    let output = quote! {
        #maproot

        #(
            pub mod #files;
        )*
        pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
            #(#files::register_types(registry);)*
        }
    };

    create_dir_all(path.as_ref())?;

    let mod_path = path.as_ref().join("mod.rs");
    let mut f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&mod_path)?;

    let output = syn::parse2(output)?;
    let output = prettyplease::unparse(&output);
    f.write_all(output.as_bytes())?;
    f.flush()?;

    Ok(())
}
