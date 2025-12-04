mod util;

extern crate proc_macro;

use core::ops::Deref;

use convert_case::{Case, Casing};
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Generics, ImplItem, LitInt,
    LitStr, Type,
};

use crate::util::{FieldKind, FieldMeta, Frequency, OneOfMeta, ProtoMeta, VarMeta};

#[proc_macro_attribute]
pub fn protoenum(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut block = parse_macro_input!(input as syn::ItemImpl);
    let ident = &block.self_ty;
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];
    for b in &mut block.items {
        if let ImplItem::Const(c) = b {
            c.attrs.retain(|a| {
                if a.path().is_ident(&format_ident!("var")) {
                    let m = a.parse_args::<VarMeta>().unwrap();
                    let name = m.name;
                    let num = m.num;
                    merge_txt.push(quote! { #name => *self = Self::from(#num), });
                    emit_txt.push(quote! { #num => stream.ident(#name), });
                    false
                } else {
                    true
                }
            });
        }
    }

    (quote! {
        #block
        impl From<i32> for #ident {
            fn from(v: i32) -> Self { Self(v) }
        }
        impl From<#ident> for i32 {
            fn from(v: #ident) -> Self { v.0 }
        }
        impl<'buf> protokit::textformat::TextField<'buf> for #ident {
             fn merge_value(&mut self, stream: &mut protokit::textformat::InputStream<'buf>) -> protokit::textformat::Result<()> {
                match stream.field() {
                    #(#merge_txt)*
                    name => return protokit::textformat::unknown(name),
                }

                Ok(())
            }
            fn emit_value(&self, stream: &mut protokit::textformat::OutputStream) {
                match self.0 {
                    #(#emit_txt)*
                    other => stream.disp(&other),
                }
            }
        }

    })
        .into()
}

#[proc_macro_derive(BinProto, attributes(proto, field, oneof, unknown))]
pub fn binproto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(s) => _impl_proto(s, input.ident, input.attrs, input.generics, true, false)
            .unwrap_or_else(Error::into_compile_error),
        Data::Enum(s) => _impl_oneof(s, input.ident, input.attrs, input.generics, true, false)
            .unwrap_or_else(Error::into_compile_error),
        Data::Union(_) => {
            panic!("Unions are not supported")
        }
    }
    .into()
}

#[proc_macro_derive(Proto, attributes(proto, field, oneof, unknown))]
pub fn proto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(s) => _impl_proto(s, input.ident, input.attrs, input.generics, true, true)
            .unwrap_or_else(Error::into_compile_error),
        Data::Enum(s) => _impl_oneof(s, input.ident, input.attrs, input.generics, true, true)
            .unwrap_or_else(Error::into_compile_error),
        Data::Union(_) => {
            panic!("Unions are not supported")
        }
    }
    .into()
}

enum Item {
    Unknowns {
        ident: Ident,
    },
    Field {
        ident: Ident,
        num: LitInt,
        name: LitStr,
        kind: FieldKind,
        freq: Frequency,
    },
    Oneof {
        ident: Ident,
        nums: Vec<LitInt>,
        names: Vec<LitStr>,
    },
}

fn bin_merge_arm(
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
                #tag => protokit::binformat::merge_map(#this, stream, protokit::binformat::InputStream::#key_fn, protokit::binformat::InputStream::#val_fn),
            }
        }
        other => {
            let parse_fn = format_ident!("{}", other.to_string());
            quote_spanned! { ident.span() =>
                #tag => protokit::binformat::#merge_fn(#this, stream, protokit::binformat::InputStream::#parse_fn),
            }
        }
    }
}

fn bin_emit_arm(
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
                protokit::binformat::emit_map(#this, #tag, #ktag, #vtag, stream, protokit::binformat::OutputStream::#key_fn, protokit::binformat::OutputStream::#val_fn);
            }
        }
        other => {
            let parse_fn = format_ident!("{}", other.to_string());
            quote_spanned! { ident.span() =>
                protokit::binformat::#emit(#this, #tag, stream, protokit::binformat::OutputStream::#parse_fn);
            }
        }
    }
}

fn bin_size_arm(
    ident: &Ident,
    num: &LitInt,
    freq: &Frequency,
    kind: &FieldKind,
    this: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if let FieldKind::Map(kind) = kind {
        let kv = format_ident!("size_{}", kind.0.to_string());
        let vv = format_ident!("size_{}", kind.1.to_string());

        let num = num.base10_parse::<u32>().unwrap();
        let tag = num << 3 | util::BYTES as u32;

        let ktag = 1u32 << 3 | kind.deref().0.wire_type() as u32;
        let vtag = 2u32 << 3 | kind.deref().1.wire_type() as u32;

        quote_spanned! { ident.span() => protokit::binformat::size_map(#this, #tag, #ktag, #vtag, stack, protokit::binformat::#kv, protokit::binformat::#vv) }
    } else {
        let wrapper = format_ident!("size_{}", freq.to_string());
        let sizer = format_ident!("size_{}", kind.to_string());

        let num = num.base10_parse::<u32>().unwrap();
        let mut wt = kind.wire_type();
        if matches!(freq, Frequency::Packed) && kind.is_scalar() {
            wt = util::BYTES;
        }
        let tag = num << 3 | wt as u32;

        quote_spanned! { ident.span() => protokit::binformat::#wrapper(#this, #tag, stack, protokit::binformat::#sizer) }
    }
}

fn _impl_proto(
    s: DataStruct,
    ident: Ident,
    attrs: Vec<Attribute>,
    generics: Generics,
    bin: bool,
    text: bool,
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
                    let ident = field.ident.clone().unwrap();
                    return Ok(Item::Field {
                        name: fmeta
                            .name
                            .unwrap_or_else(|| LitStr::new(&ident.to_string(), ident.span())),
                        ident,
                        num: fmeta.num,
                        kind: fmeta.kind,
                        freq: fmeta.freq,
                    });
                } else if a.path().is_ident(&format_ident!("oneof")) {
                    let ometa: OneOfMeta = a.parse_args()?;
                    return Ok(Item::Oneof {
                        ident: field.ident.clone().unwrap(),
                        nums: ometa.nums,
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

    let mut text_names = vec![];
    let mut merge_txt = vec![];
    let mut emit_txt = vec![];

    for it in items.iter() {
        match it {
            Item::Unknowns { ident } => {
                if bin {
                    merge_bin.push(quote_spanned! { ident.span() =>
                        tag => protokit::binformat::BinProto::merge_field(&mut self.#ident, tag, stream),
                    });
                    emit_bin.push(quote_spanned! { ident.span() =>
                        protokit::binformat::BinProto::encode(&self.#ident, stream);
                    });
                    size_bin.push(quote_spanned! { ident.span() =>
                        protokit::binformat::BinProto::size(&self.#ident, stack)
                    });
                }
                if text {
                    emit_txt.push(quote_spanned! { ident.span() =>
                        protokit::textformat::TextProto::encode(&self.#ident, stream);
                    });
                }
            }
            Item::Field {
                ident,
                name,
                num: tag,
                kind,
                freq,
                ..
            } => {
                let this_mut = quote! { &mut self.#ident };
                let this_ref = quote! { &self.#ident};

                if bin {
                    merge_bin.push(if kind.is_scalar() && freq.is_multi() {
                        let a = bin_merge_arm(ident, tag, &Frequency::Repeated, kind, &this_mut);
                        let b = bin_merge_arm(ident, tag, &Frequency::Packed, kind, &this_mut);
                        quote_spanned!( ident.span() => #a #b )
                    } else {
                        bin_merge_arm(ident, tag, freq, kind, &this_mut)
                    });

                    emit_bin.push(bin_emit_arm(ident, tag, freq, kind, &this_ref));
                    size_bin.push(bin_size_arm(ident, tag, freq, kind, &this_ref));
                }
                if text {
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

                    text_names.push(quote! { (#name, #tag) });
                    merge_txt.push(quote_spanned! { ident.span() =>
                        #tag => protokit::textformat::#merge(&mut self.#ident, stream),
                    });
                    emit_txt.push(quote_spanned! { ident.span() =>
                        protokit::textformat::#emit(&self.#ident, #name, stream);
                    });
                }
            }

            Item::Oneof { ident, names, nums, .. } => {
                let tags = nums
                    .iter()
                    .flat_map(|t| {
                        let v = t.base10_parse::<u32>().unwrap() << 3;
                        [v, v + 1, v + 2, v + 3, v + 4, v + 5, v + 6, v + 7]
                    })
                    .collect::<Vec<_>>();

                if bin {
                    merge_bin.push(quote_spanned! { ident.span() =>
                        #(#tags)|* => protokit::binformat::merge_oneof(&mut self.#ident, tag, stream),
                    });
                    emit_bin.push(quote_spanned! { ident.span() =>
                        protokit::binformat::emit_oneof(&self.#ident, stream);
                    });
                    size_bin.push(quote_spanned! { ident.span() =>
                        protokit::binformat::size_oneof(&self.#ident, stack)
                    });
                }

                if text {
                    for (n, t) in names.iter().zip(nums) {
                        text_names.push(quote! { (#n, #t) });
                    }
                    merge_txt.push(quote_spanned! { ident.span() =>
                        #(#nums)|* => protokit::textformat::merge_oneof(&mut self.#ident, stream),
                    });
                    emit_txt.push(quote_spanned! { ident.span() =>
                        protokit::textformat::emit_oneof(&self.#ident, stream);
                    });
                }
            }
        }
    }

    size_bin.reverse();
    let (_, type_gen, where_gen) = generics.split_for_impl();
    let lp = generics.lifetimes();
    let tp = generics.type_params();
    let cp = generics.const_params();

    let (buf_param, additional_lifetime) = match meta.buf {
        None => (quote! { 'buf }, quote! { 'buf, }),
        Some(borrow) => (quote! { #borrow }, quote! {}),
    };
    let text_impl_params = quote! { #additional_lifetime #(#lp,)* #(#tp,)* #(#cp,)* };

    let qname = match (meta.package, meta.name) {
        (Some(pkg), Some(nam)) => {
            let name = format!("{}.{}", pkg.value(), nam.value());
            quote! { #name }
        }
        (None, Some(name)) => {
            quote! { #name }
        }
        _ => {
            let msg = format!("Qualified name not supported for {}", ident.to_string());
            quote! { panic!(#msg) }
        }
    };

    let proto_name_impl = quote! {
        impl <#text_impl_params> protokit::binformat::ProtoName for #ident #type_gen #where_gen {
            fn qualified_name(&self) -> &'static str {
                #qname
            }
        }
        impl <#text_impl_params> protokit::binformat::TypedProtoName for #ident #type_gen #where_gen {
            fn qualified_name() -> &'static str {
                #qname
            }
        }
    };

    let bin_impl = if bin {
        Some(quote! {
             impl <#text_impl_params> protokit::binformat::BinProto<#buf_param> for #ident #type_gen #where_gen {
                fn merge_field(&mut self, tag: u32, stream: &mut protokit::binformat::InputStream<#buf_param>) -> protokit::binformat::Result<()> {
                    #![deny(unreachable_patterns)]
                    match tag {
                        #(#merge_bin)*
                        other => stream.skip(other),
                    }
                }
                fn size(&self, stack: &mut protokit::binformat::SizeStack) -> usize {
                    0 #(+ #size_bin)*
                }
                fn encode(&self, stream: &mut protokit::binformat::OutputStream) {
                    #(#emit_bin)*
                }
            }
        })
    } else {
        None
    };

    let text_impl = if text {
        Some(quote! {
            impl<#text_impl_params> protokit::textformat::TextProto< #buf_param > for #ident #type_gen #where_gen {
                fn decode(&mut self, stream: &mut protokit::textformat::InputStream<#buf_param>) -> protokit::textformat::Result<()> {
                    if let Some(proxy) = stream.reg.find_proxy(<Self as protokit::binformat::ProtoName>::qualified_name(self)) {
                        return proxy.merge(self, stream);
                    }
                    stream.message_fields(self)
                }
                fn merge_field(&mut self, stream: &mut protokit::textformat::InputStream<#buf_param>) -> protokit::textformat::Result<()> {
                    const FIELDS: &[(&str, u32)] = &[#(#text_names,)*];
                    match protokit::textformat::_find(stream, FIELDS) {
                        #(#merge_txt)*
                        name => protokit::textformat::unknown(stream.field()),
                    }
                }
                fn encode(&self, stream: &mut protokit::textformat::OutputStream) {
                    if let Some(proxy) = stream.reg.find_proxy(<Self as protokit::binformat::ProtoName>::qualified_name(self)) {
                        return proxy.encode(self, stream);
                    }
                    #(#emit_txt)*
                }
            }
        })
    } else {
        None
    };

    Ok(quote! {
        #proto_name_impl
        #bin_impl
        #text_impl
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
    bin: bool,
    text: bool,
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
                        tag: fmeta.num,
                        name: fmeta
                            .name
                            .unwrap_or_else(|| LitStr::new(&variant.ident.to_string(), variant.ident.span())),
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

        let size = bin_size_arm(ident, tag, &Frequency::Raw, kind, &quote! { v });
        size_bin.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => #size,
        });

        merge_bin.push(bin_merge_arm(ident, tag, &Frequency::Singular, kind, &this));
        let emit = bin_emit_arm(ident, tag, &Frequency::Raw, kind, &quote! { v });
        emit_bin.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => { #emit },
        });

        let emit = format_ident!("emit_raw", span = ident.span());
        merge_txt.push(quote_spanned! { ident.span() =>
            #name =>  self.#setter().merge_text(stream),
        });
        emit_txt.push(quote_spanned! { ident.span() =>
            Self::#ident(v) => protokit::textformat::#emit(v, #name, stream),
        });
    }
    size_bin.reverse();

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

    let qname = match (meta.package, meta.name) {
        (Some(pkg), Some(nam)) => {
            let name = format!("{}.{}", pkg.value(), nam.value());
            quote! { #name }
        }
        (None, Some(name)) => {
            quote! { #name }
        }
        _ => {
            quote! { "" }
        }
    };

    let proto_name_impl = quote! {
        impl <#text_impl_params> protokit::binformat::ProtoName for #ident #type_gen #where_gen {
            fn qualified_name(&self) -> &'static str {
                #qname
            }
        }
        impl <#text_impl_params> protokit::binformat::TypedProtoName for #ident #type_gen #where_gen {
            fn qualified_name() -> &'static str {
                #qname
            }
        }
    };

    let bin_impl = if bin {
        Some(quote! {
            impl <#text_impl_params> protokit::binformat::BinProto<#buf_param> for #ident #type_gen #where_gen {
                fn merge_field(&mut self, tag: u32, stream: &mut protokit::binformat::InputStream<#buf_param>) -> protokit::binformat::Result<()> {
                    #![deny(unreachable_patterns)]
                    match tag {
                        #(#merge_bin)*
                        other => stream.skip(other),
                    }
                }
                fn size(&self, stack: &mut protokit::binformat::SizeStack) -> usize {
                    match self {
                        #(#size_bin)*
                        _ => 0,
                    }
                }
                fn encode(&self, stream: &mut protokit::binformat::OutputStream) {
                    match self {
                        #(#emit_bin)*
                        _ => {},
                    }
                }
            }
        })
    } else {
        None
    };

    let text_impl = if text {
        Some(quote! {
            impl<#text_impl_params> protokit::textformat::TextProto<#buf_param> for #ident #type_gen #where_gen {
                fn decode(&mut self, stream: &mut protokit::textformat::InputStream<#buf_param>) -> protokit::textformat::Result<()> {
                    stream.message_fields(self)
                }
                fn merge_field(&mut self, stream: &mut protokit::textformat::InputStream<#buf_param>) -> protokit::textformat::Result<()> {
                    match stream.field() {
                        #(#merge_txt)*
                        name => protokit::textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut protokit::textformat::OutputStream) {
                    match self {
                        #(#emit_txt)*
                        _ => {},
                    }
                }
            }
        })
    } else {
        None
    };

    Ok(quote! {
        impl #orig_impl_gen #ident #type_gen #where_gen {
            #(#setters)*
        }
        #proto_name_impl
        #bin_impl
        #text_impl
    })
}
