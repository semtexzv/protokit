extern crate proc_macro;

use std::fmt::{Display, Formatter};
use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{format_ident, quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    bracketed, parenthesized, parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields,
    ImplItem, LitInt, LitStr, Token, Type,
};
use syn::token::Token;

const VARINT: u8 = 0;
const FIX64: u8 = 1;
const BYTES: u8 = 2;
const SGRP: u8 = 3;
const EGRP: u8 = 4;
const FIX32: u8 = 5;

#[proc_macro_error]
#[proc_macro_derive(Proto, attributes(proto, field, oneof))]
pub fn proto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(s) => _impl_proto(s, input.ident, input.attrs).unwrap_or_else(Error::into_compile_error),
        Data::Enum(s) => _impl_oneof(s, input.ident, input.attrs).unwrap_or_else(Error::into_compile_error),
        _ => abort_call_site!("Unsupported: {data:?}"),
    }
        .into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn protoenum(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut block = parse_macro_input!(input as syn::ItemImpl);
    let ident = &block.self_ty;
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];
    for b in &mut block.items {
        match b {
            ImplItem::Const(c) => c.attrs.retain(|a| {
                if a.path().is_ident(&format_ident!("var")) {
                    let m = a.parse_args::<VarMeta>().unwrap();
                    let name = m.name;
                    let num = m.tag;
                    merge_txt.push(quote! { #name => *self = Self::from(#num), });
                    emit_txt.push(quote! { #num => stream.ident(#name), });
                    false
                } else {
                    true
                }
            }),
            _ => {}
        }
    }

    (quote! {
        #block
        impl From<u32> for #ident {
            fn from(v: u32) -> Self { Self(v) }
        }
        impl From<#ident> for u32 {
            fn from(v: #ident) -> Self { v.0 }
        }
        impl textformat::TextField for #ident {
             fn merge_value(&mut self, stream: &mut textformat::InputStream) -> textformat::Result<()> {
                match stream.field() {
                    #(#merge_txt)*
                    name => return textformat::unknown(name),
                }

                Ok(())
            }
            fn emit_value(&self, stream: &mut textformat::OutputStream) {
                match self.0 {
                    #(#emit_txt)*
                    other => stream.disp(&other),
                }
            }
        }

    })
        .into()
}

struct Kv<V: Parse> {
    name: Ident,
    value: V,
}

impl<V: Parse> Parse for Kv<V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(Self { name, value })
    }
}

#[derive(Default)]
struct ProtoMeta {
    name: Option<LitStr>,
    file: Option<LitStr>,
    package: Option<LitStr>,
}

impl Parse for ProtoMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields = Punctuated::<Kv<LitStr>, Token![,]>::parse_terminated(input)?;

        let mut out = Self::default();
        for f in fields {
            if f.name == "name" {
                out.name = Some(f.value);
            } else if f.name == "file" {
                out.file = Some(f.value);
            } else if f.name == "package" {
                out.package = Some(f.value)
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    format!("Unknown key: {}, expected name, file or package", f.name),
                ));
            }
        }
        Ok(out)
    }
}

struct VarMeta {
    tag: LitInt,
    name: LitStr,
}

impl Parse for VarMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag: LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;
        let name: LitStr = input.parse()?;

        Ok(Self { tag, name })
    }
}

struct FieldMeta {
    tag: LitInt,
    name: LitStr,
    kind: FieldKind,
    freq: FieldFreq,
}

fn err(s: Span, m: impl Display) -> syn::Error {
    syn::Error::new(s, m)
}

impl Parse for FieldMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag: LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;
        let name: LitStr = input.parse()?;
        let _: Token![,] = input.parse().map_err(|e| err(e.span(), "Expected field kind"))?;
        let kind: FieldKind = input.parse()?;
        let freq = if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            input.parse()?
        } else {
            FieldFreq::Singular
        };

        Ok(Self { tag, name, kind, freq })
    }
}

#[derive(Debug, PartialEq)]
enum FieldKind {
    Varint,
    Sigint,
    Bool,
    ProtoEnum,
    Fixed32,
    Fixed64,
    Bytes,
    String,
    Nested,
    Group,
    Map(Box<(FieldKind, FieldKind)>),
}

impl FieldKind {
    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Varint | Self::Sigint | Self::ProtoEnum | Self::Bool | Self::Fixed32 | Self::Fixed64
        )
    }
    pub fn wire_type(&self) -> u8 {
        match self {
            FieldKind::Varint | FieldKind::Sigint | FieldKind::ProtoEnum | FieldKind::Bool => VARINT,
            FieldKind::Fixed32 => FIX32,
            FieldKind::Fixed64 => FIX64,
            FieldKind::Bytes | FieldKind::String | FieldKind::Nested | FieldKind::Map(_) => BYTES,
            FieldKind::Group => SGRP,
        }
    }
}

impl Display for FieldKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldKind::Varint => write!(f, "varint"),
            FieldKind::Sigint => write!(f, "sigint"),
            FieldKind::ProtoEnum => write!(f, "protoenum"),
            FieldKind::Bool => write!(f, "bool"),
            FieldKind::Fixed32 => write!(f, "fixed32"),
            FieldKind::Fixed64 => write!(f, "fixed64"),
            FieldKind::Bytes => write!(f, "bytes"),
            FieldKind::String => write!(f, "string"),
            FieldKind::Nested => write!(f, "nested"),
            FieldKind::Group => write!(f, "group"),
            FieldKind::Map(k) => write!(f, "map({},{})", k.deref().0, k.deref().1),
        }
    }
}

impl Parse for FieldKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(if ident == "varint" {
            FieldKind::Varint
        } else if ident == "sigint" {
            FieldKind::Sigint
        } else if ident == "protoenum" {
            FieldKind::ProtoEnum
        } else if ident == "bool" {
            FieldKind::Bool
        } else if ident == "fixed32" {
            FieldKind::Fixed32
        } else if ident == "fixed64" {
            FieldKind::Fixed64
        } else if ident == "bytes" {
            FieldKind::Bytes
        } else if ident == "string" {
            FieldKind::String
        } else if ident == "nested" {
            FieldKind::Nested
        } else if ident == "group" {
            FieldKind::Group
        } else if ident == "map" {
            let params;
            let _ = parenthesized!(params in input);
            let k: FieldKind = params.parse()?;
            let _: Token![,] = params.parse()?;
            let v: FieldKind = params.parse()?;
            FieldKind::Map(Box::new((k, v)))
        } else {
            return Err(Error::new(
                ident.span(),
                format!(
                    "Unknown field kind: {}, expected varint, fixed, nested, or group",
                    ident
                ),
            ));
        })
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
enum FieldFreq {
    #[default]
    Singular,
    Optional,
    Repeated,
    Required,
    Packed,
}

impl FieldFreq {
    fn is_multi(&self) -> bool {
        matches!(self, Self::Repeated | Self::Packed)
    }
    fn method_suffix(&self) -> &'static str {
        match self {
            FieldFreq::Singular | FieldFreq::Required => "single",
            FieldFreq::Packed => "packed",
            FieldFreq::Repeated => "repeated",
            FieldFreq::Optional => "optional",
        }
    }
}

impl Parse for FieldFreq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(if ident == "singular" {
            FieldFreq::Singular
        } else if ident == "optional" {
            FieldFreq::Optional
        } else if ident == "repeated" {
            FieldFreq::Repeated
        } else if ident == "required" {
            FieldFreq::Required
        } else if ident == "packed" {
            FieldFreq::Packed
        } else {
            return Err(syn::Error::new(
                ident.span(),
                format!(
                    "Unknown field frequency: {}, expected singular, optional or repeated",
                    ident
                ),
            ));
        })
    }
}

impl Display for FieldFreq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldFreq::Singular => write!(f, "singular"),
            FieldFreq::Optional => write!(f, "optinal"),
            FieldFreq::Repeated => write!(f, "repeated"),
            FieldFreq::Required => write!(f, "singular"),
            FieldFreq::Packed => write!(f, "packed"),
        }
    }
}

struct OneOfMeta {
    tags: Vec<LitInt>,
    names: Vec<LitStr>,
}

impl Parse for OneOfMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tags;
        let _ = bracketed!(tags in input);
        let tags = Punctuated::<LitInt, Token![,]>::parse_terminated(&tags)?;
        let _: Token![,] = input.parse()?;
        let names;
        let _ = bracketed!(names in input);
        let names = Punctuated::<LitStr, Token![,]>::parse_terminated(&names)?;

        Ok(Self {
            tags: tags.into_iter().collect(),
            names: names.into_iter().collect(),
        })
    }
}

enum Item {
    Field {
        ident: Ident,
        tag: LitInt,
        name: LitStr,
        kind: FieldKind,
        freq: FieldFreq,
    },
    Oneof {
        ident: Ident,
        tags: Vec<LitInt>,
        names: Vec<LitStr>,
    },
}

fn merge_arm(
    ident: &Ident,
    num: &LitInt,
    freq: &FieldFreq,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let num = num.base10_parse::<u32>().unwrap();
    let mut wt = kind.wire_type();
    if matches!(freq, FieldFreq::Packed) && kind.is_scalar() {
        wt = BYTES;
    }
    let tag = num << 3 | wt as u32;
    let merge_fn = format_ident!("merge_{}", freq.method_suffix(), span = ident.span());
    match kind {
        FieldKind::Map(t) => {
            let key_fn = format_ident!("{}", t.deref().0.to_string());
            let val_fn = format_ident!("{}", t.deref().1.to_string());
            quote_spanned! { ident.span() =>
                #tag => binformat::merge_map(#this, stream, binformat::InputStream::#key_fn, binformat::InputStream::#val_fn),
            }
        }
        other => {
            let parse_fn = format_ident!("{}", other.to_string());
            quote_spanned! { ident.span() =>
                #tag => binformat::#merge_fn(#this, stream, binformat::InputStream::#parse_fn),
            }
        }
    }
}

fn emit_arm(
    ident: &Ident,
    num: &LitInt,
    freq: &FieldFreq,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let tag = num.base10_parse::<u32>().unwrap() << 3 | kind.wire_type() as u32;
    let emit = format_ident!("emit_{}", freq.method_suffix(), span = ident.span());

    match kind {
        FieldKind::Map(t) => {
            let key_fn = format_ident!("{}", t.deref().0.to_string());
            let val_fn = format_ident!("{}", t.deref().1.to_string());
            let ktag = 1u32 << t.deref().0.wire_type();
            let vtag = 2u32 << t.deref().1.wire_type();
            quote_spanned! { ident.span() =>
                binformat::emit_map(#this, #tag, #ktag, #vtag, stream, binformat::OutputStream::#key_fn, binformat::OutputStream::#val_fn);
            }
        }
        other => {
            let parse_fn = format_ident!("{}", other.to_string());
            quote_spanned! { ident.span() =>
                binformat::#emit(#this, #tag, stream, binformat::OutputStream::#parse_fn);
            }
        }
    }
}

fn _impl_proto(s: DataStruct, ident: Ident, _: Vec<Attribute>) -> syn::Result<proc_macro2::TokenStream> {
    let items = s
        .fields
        .iter()
        .map(|field| {
            for a in &field.attrs {
                if a.path().is_ident(&format_ident!("field")) {
                    let fmeta: FieldMeta = a.parse_args()?;
                    return Ok(Item::Field {
                        ident: field.ident.clone().unwrap(),
                        tag: fmeta.tag,
                        name: fmeta.name,
                        kind: fmeta.kind,
                        freq: fmeta.freq,
                    });
                } else if a.path().is_ident(&format_ident!("oneof")) {
                    let ometa: OneOfMeta = a.parse_args()?;
                    return Ok(Item::Oneof {
                        ident: field.ident.clone().unwrap(),
                        tags: ometa.tags,
                        names: ometa.names,
                    });
                }
            }
            Err(Error::new(
                field.span(),
                "Missing either #[field], or #[oneof] attribute",
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut merge_bin = vec![];
    let mut emit_bin = vec![];
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];

    for it in items.iter() {
        match it {
            Item::Field {
                ident,
                name,
                tag,
                kind,
                freq,
                ..
            } => {
                let this = quote! { &mut self.#ident };
                merge_bin.push(if kind.is_scalar() && freq.is_multi() {
                    let a = merge_arm(ident, tag, &FieldFreq::Repeated, kind, &this);
                    let b = merge_arm(ident, tag, &FieldFreq::Packed, kind, &this);
                    quote_spanned!( ident.span() => #a #b )
                } else {
                    merge_arm(ident, tag, freq, kind, &this)
                });

                emit_bin.push(emit_arm(ident, tag, freq, kind, &quote! { &self.#ident}));
                merge_txt.push(quote_spanned! { ident.span() =>
                    #name => self.#ident.merge_text(stream),
                });
                emit_txt.push(quote_spanned! { ident.span() =>
                    stream.emit_field(#name, &self.#ident);
                });
            }

            Item::Oneof { ident, names, tags, .. } => {
                let tags = tags
                    .iter()
                    .map(|t| {
                        let v = t.base10_parse::<u32>().unwrap() << 3;
                        [v, v + 1, v + 2, v + 3, v + 4, v + 5, v + 6, v + 7]
                    })
                    .flatten()
                    .collect::<Vec<_>>();

                merge_bin.push(quote_spanned! { ident.span() =>
                    #(#tags)|* => self.#ident.merge_field(tag, stream),
                });
                emit_bin.push(quote_spanned! { ident.span() =>
                    self.#ident.encode(stream);
                });
                merge_txt.push(quote_spanned! { ident.span() =>
                    #(#names)|* => self.#ident.merge_text(stream),
                });
                emit_txt.push(quote_spanned! { ident.span() =>
                    stream.emit_oneof(&self.#ident);
                });
            }
        }
    }

    Ok(quote! {
        impl binformat::BinProto for #ident {
            fn merge_field(&mut self, tag: u32, stream: &mut binformat::InputStream) -> binformat::Result<()> {
                match tag {
                    #(#merge_bin)*
                    other => stream.skip(other),
                    // other => binformat::unknown_tag(other),
                }
            }
            fn encode(&self, stream: &mut binformat::OutputStream) {
                #(#emit_bin)*
            }
        }

        impl textformat::TextProto for #ident {
            fn merge_field(&mut self, stream: &mut textformat::InputStream) -> textformat::Result<()> {
                match stream.field() {
                    #(#merge_txt)*
                    name => textformat::unknown(name),
                }
            }
            fn encode(&self, stream: &mut textformat::OutputStream) {
                #(#emit_txt)*
            }
        }
    })
}

struct OneOfField {
    ident: Ident,
    kind: FieldKind,
    typ: Type,
    setter: Ident,
    tag: LitInt,
    name: LitStr,
}

fn _impl_oneof(s: DataEnum, ident: Ident, _: Vec<Attribute>) -> syn::Result<proc_macro2::TokenStream> {
    let items = s
        .variants
        .iter()
        .map(|variant| {
            for a in &variant.attrs {
                if a.path().is_ident(&format_ident!("field")) {
                    let fmeta: FieldMeta = a.parse_args()?;
                    return Ok(OneOfField {
                        ident: variant.ident.clone(),
                        kind: fmeta.kind,
                        typ: match &variant.fields {
                            Fields::Unnamed(f) => f.unnamed.first().unwrap().ty.clone(),
                            _ => panic!(),
                        },
                        setter: format_ident!("make_{}_mut", variant.ident),
                        tag: fmeta.tag,
                        name: fmeta.name,
                    });
                }
            }
            Err(Error::new(variant.span(), "Missing #[field] attribute"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut setters = items.iter().map(|item| {
        let OneOfField { ident, typ, setter, .. } = item;

        quote_spanned! { ident.span() =>
            fn #setter(&mut self) -> &mut #typ {
                loop {
                    match self {
                        Self::#ident(v) => return v,
                        _ => *self = Self::#ident(Default::default()),
                    }
                }
            }
        }
    });

    let mut merge_bin = vec![];
    let mut emit_bin = vec![];
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];

    for it in items.iter() {
        let OneOfField {
            ident,
            tag,
            name,
            setter,
            kind,
            ..
        } = it;

        let this = quote! { self.#setter() };
        merge_bin.push(merge_arm(ident, tag, &FieldFreq::Singular, kind, &this));
        let emit = emit_arm(&ident, tag, &FieldFreq::Singular, kind, &quote! { v });
        emit_bin.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => { #emit },
        });
        merge_txt.push(quote_spanned! { ident.span() =>
            #name => self.#setter().merge_text(stream),
        });
        emit_txt.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => stream.emit_field(#name, v),
        });
    }

    Ok(quote! {
        impl #ident {
            #(#setters)*
        }
        impl binformat::BinProto for #ident {
            fn merge_field(&mut self, tag: u32, stream: &mut binformat::InputStream) -> binformat::Result<()> {
                match tag {
                    #(#merge_bin)*
                    other => stream.skip(other),
                    // other => binformat::unknown_tag(other),
                }
            }
            fn encode(&self, stream: &mut binformat::OutputStream) {
                match self {
                    #(#emit_bin)*
                }
            }
        }
        impl textformat::TextProto for #ident {
            fn merge_field(&mut self, stream: &mut textformat::InputStream) -> textformat::Result<()> {
                match stream.field() {
                    #(#merge_txt)*
                    name => textformat::unknown(name),
                }
            }
            fn encode(&self, stream: &mut textformat::OutputStream) {
                match self {
                    #(#emit_txt)*
                }
            }
        }
    })
}
