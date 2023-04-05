mod util;

extern crate proc_macro;

use std::ops::Deref;
use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Generics,
    ImplItem, LitInt, LitStr, Type,
};

use crate::util::{FieldKind, FieldMeta, Frequency, OneOfMeta, ProtoMeta, VarMeta};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn protoenum(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        impl<'buf> textformat::TextField<'buf> for #ident {
             fn merge_value(&mut self, stream: &mut textformat::InputStream<'buf>) -> textformat::Result<()> {
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

#[proc_macro_error]
#[proc_macro_derive(Proto, attributes(proto, field, oneof, unknown))]
pub fn proto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(s) => {
            _impl_proto(s, input.ident, input.attrs, input.generics).unwrap_or_else(Error::into_compile_error)
        }
        Data::Enum(s) => {
            _impl_oneof(s, input.ident, input.attrs, input.generics).unwrap_or_else(Error::into_compile_error)
        }
        _ => abort_call_site!("Unsupported: {data:?}"),
    }
        .into()
}

enum Item {
    Unknowns {
        ident: Ident,
    },
    Field {
        ident: Ident,
        tag: LitInt,
        name: LitStr,
        kind: FieldKind,
        freq: Frequency,
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
    freq: &Frequency,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let num = num.base10_parse::<u32>().unwrap();
    let mut wt = kind.wire_type();
    if matches!(freq, Frequency::Packed) && kind.is_scalar() {
        wt = util::BYTES;
    }
    let tag = num << 3 | wt as u32;
    let merge_fn = format_ident!("merge_{}", freq.binformat_suffix(), span = ident.span());
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
    freq: &Frequency,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut wt = kind.wire_type();
    if matches!(freq, Frequency::Packed) && kind.is_scalar() {
        wt = util::BYTES;
    }

    let tag = num.base10_parse::<u32>().unwrap() << 3 | wt as u32;
    let emit = format_ident!("emit_{}", freq.binformat_suffix(), span = ident.span());

    match kind {
        FieldKind::Map(t) => {
            let key_fn = format_ident!("{}", t.deref().0.to_string());
            let val_fn = format_ident!("{}", t.deref().1.to_string());
            let ktag = 1u32 << 3 | t.deref().0.wire_type() as u32;
            let vtag = 2u32 << 3 | t.deref().1.wire_type() as u32;
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

fn size_arm(
    ident: &Ident,
    freq: &Frequency,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let sizer = if let FieldKind::Map(kind) = kind {
        let id = format_ident!("size_map_{}", freq.to_string()).into_token_stream();
        let kv = format_ident!("{}", kind.0.to_string());
        let vv = format_ident!("{}", kind.0.to_string());
        quote_spanned! { ident.span() => #id::<#kv, #vv> }
    } else {
        format_ident!("size_{}_{}", kind.to_string(), freq.to_string()).into_token_stream()
    };
    quote_spanned! { ident.span() => binformat::#sizer(#this) }
}

fn _impl_proto(
    s: DataStruct,
    ident: Ident,
    attrs: Vec<Attribute>,
    generics: Generics,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut meta: Option<ProtoMeta> = None;
    for a in attrs {
        if a.path().is_ident("proto") {
            meta = Some(a.parse_args()?);
        }
    }
    let meta = meta.unwrap_or_default();

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
                } else if a.path().is_ident(&format_ident!("unknown")) {
                    return Ok(Item::Unknowns {
                        ident: field.ident.clone().unwrap(),
                    });
                }
            }
            Err(Error::new(
                field.span(),
                "Missing either #[field], or #[oneof] attribute",
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut size_bin = vec![];
    let mut merge_bin = vec![];
    let mut emit_bin = vec![];
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];

    for it in items.iter() {
        match it {
            Item::Unknowns { ident } => {
                merge_bin.push(quote_spanned! { ident.span() =>
                    tag => self.#ident.merge_field(tag, stream),
                });
                emit_bin.push(quote_spanned! { ident.span() =>
                    self.#ident.encode(stream);
                });
            }
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
                    let a = merge_arm(ident, tag, &Frequency::Repeated, kind, &this);
                    let b = merge_arm(ident, tag, &Frequency::Packed, kind, &this);
                    quote_spanned!( ident.span() => #a #b )
                } else {
                    merge_arm(ident, tag, freq, kind, &this)
                });
                let this = quote! { &self.#ident};
                emit_bin.push(emit_arm(ident, tag, freq, kind, &this));
                size_bin.push(size_arm(&ident, freq, kind, &this));


                let emit = if let FieldKind::Map(..) = kind {
                    format_ident!("emit_map", span = ident.span())
                } else {
                    format_ident!("emit_{}", freq.textformat_suffix(), span = ident.span())
                };

                let merge = if let FieldKind::Map(..) = kind {
                    format_ident!("merge_map", span = ident.span())
                } else {
                    format_ident!("merge_{}", freq.textformat_suffix(), span = ident.span())
                };


                merge_txt.push(quote_spanned! { ident.span() =>
                    #name => textformat::#merge(&mut self.#ident, stream),
                });
                emit_txt.push(quote_spanned! { ident.span() =>
                    textformat::#emit(&self.#ident, #name, stream);
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
                    #(#tags)|* => binformat::merge_oneof(&mut self.#ident, tag, stream),
                });
                emit_bin.push(quote_spanned! { ident.span() =>
                    binformat::emit_oneof(&self.#ident, stream);
                });
                size_bin.push(quote_spanned! { ident.span() =>
                    self.#ident.size()
                });

                merge_txt.push(quote_spanned! { ident.span() =>
                    #(#names)|* => textformat::merge_oneof(&mut self.#ident, stream),
                });
                emit_txt.push(quote_spanned! { ident.span() =>
                    textformat::emit_oneof(&self.#ident, stream);
                });
            }
        }
    }

    let (_, type_gen, where_gen) = generics.split_for_impl();
    let lp = generics.lifetimes();
    let tp = generics.type_params();
    let cp = generics.const_params();

    let (buf_param, additional_lifetime) = match meta.buf {
        None => (quote! { 'buf }, quote! { 'buf, }),
        Some(borrow) => (quote! { #borrow }, quote! {}),
    };
    let text_impl_params = quote! { #additional_lifetime #(#lp,)* #(#tp,)* #(#cp,)* };

    Ok(quote! {
        impl <#text_impl_params> binformat::BinProto<#buf_param> for #ident #type_gen #where_gen {
            fn merge_field(&mut self, tag: u32, stream: &mut binformat::InputStream<#buf_param>) -> binformat::Result<()> {
                match tag {
                    #(#merge_bin)*
                    other => stream.skip(other),
                }
            }
            fn encode(&self, stream: &mut binformat::OutputStream) {
                #(#emit_bin)*
            }
        }


        impl<#text_impl_params> textformat::TextProto< #buf_param > for #ident #type_gen #where_gen {
            fn merge_field(&mut self, stream: &mut textformat::InputStream< #buf_param >) -> textformat::Result<()> {
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

fn _impl_oneof(
    s: DataEnum,
    ident: Ident,
    attrs: Vec<Attribute>,
    generics: Generics,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut meta: Option<ProtoMeta> = None;
    for a in attrs {
        if a.path().is_ident("proto") {
            meta = Some(a.parse_args()?);
        }
    }
    let meta = meta.unwrap_or_default();

    let items = s
        .variants
        .iter()
        .filter_map(|variant| {
            for a in &variant.attrs {
                if a.path().is_ident(&format_ident!("field")) {
                    let fmeta: FieldMeta = a.parse_args().unwrap();
                    return Some(OneOfField {
                        ident: variant.ident.clone(),
                        kind: fmeta.kind,
                        typ: match &variant.fields {
                            Fields::Unnamed(f) => f.unnamed.first().unwrap().ty.clone(),
                            _ => panic!(),
                        },
                        setter: format_ident!("make_{}_mut", variant.ident.to_string().to_case(Case::Snake)),
                        tag: fmeta.tag,
                        name: fmeta.name,
                    });
                }
            }
            None
            // Err(Error::new(variant.span(), "Missing #[field] attribute"))
        })
        .collect::<Vec<_>>();

    let setters = items.iter().map(|item| {
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

    let mut size_bin = vec![];
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
        merge_bin.push(merge_arm(ident, tag, &Frequency::Singular, kind, &this));
        let emit = emit_arm(&ident, tag, &Frequency::Raw, kind, &quote! { v });
        emit_bin.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => { #emit },
        });
        size_bin.push(size_arm(&ident, &Frequency::Raw, kind, &this));

        let emit = format_ident!("emit_raw", span = ident.span());

        merge_txt.push(quote_spanned! { ident.span() =>
            #name =>  self.#setter().merge_text(stream),
        });
        emit_txt.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => textformat::#emit(v, #name, stream),
        });
    }

    let (orig_impl_gen, type_gen, where_gen) = generics.split_for_impl();
    let lp = generics.lifetimes();
    let tp = generics.type_params();
    let cp = generics.const_params();

    // We add the 'buf lifetime only if user did not borrow the input.
    let (buf_param, additional_lifetime) = match meta.buf {
        None => (quote! { 'buf }, quote! { 'buf, }),
        Some(borrow) => (quote! { #borrow }, quote! {}),
    };

    let text_impl_params = quote! { #additional_lifetime #(#lp,)* #(#tp,)* #(#cp,)* };

    Ok(quote! {
        impl #orig_impl_gen #ident #type_gen #where_gen {
            #(#setters)*
        }
        impl <#text_impl_params> binformat::BinProto<#buf_param> for #ident #type_gen #where_gen {
            fn merge_field(&mut self, tag: u32, stream: &mut binformat::InputStream<#buf_param>) -> binformat::Result<()> {
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
        impl<#text_impl_params> textformat::TextProto<#buf_param> for #ident #type_gen #where_gen {
            fn merge_field(&mut self, stream: &mut textformat::InputStream<#buf_param>) -> textformat::Result<()> {
                println!("Field {} {}", stream.field(), std::any::type_name::<Self>());
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
