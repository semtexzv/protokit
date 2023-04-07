use core::fmt::{Display, Formatter};

use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parenthesized, Error, Lifetime, LitInt, LitStr, Token};

pub const VARINT: u8 = 0;
pub const FIX64: u8 = 1;
pub const BYTES: u8 = 2;
pub const SGRP: u8 = 3;
pub const _EGRP: u8 = 4;
pub const FIX32: u8 = 5;

#[derive(Default)]
pub struct ProtoMeta {
    pub buf: Option<Lifetime>,
    pub name: Option<LitStr>,
    pub file: Option<LitStr>,
    pub package: Option<LitStr>,
}

impl Parse for ProtoMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut out = Self::default();
        let mut oname: Option<Ident> = input.parse()?;
        for _ in 0 .. 100 {
            let Some(name) = &oname else {
                return Ok(out);
            };

            let _: Token![=] = input.parse()?;
            if name == "buf" || name == "borrow" {
                out.buf = Some(input.parse()?);
            } else if name == "name" {
                out.name = Some(input.parse()?);
            }

            let _: Option<Token![,]> = input.parse()?;

            oname = input.parse()?;
        }
        panic!()
        // for f in fields {
        //     if if f.name == "buf" || f.name == "borrow" {
        //
        //         out.buf = Some(f.value);
        //     } else if f.name == "name" {
        //         out.name = Some(f.value);
        //     } else if f.name == "file" {
        //         out.file = Some(f.value);
        //     } else if f.name == "package" {
        //         out.package = Some(f.value)
        //     } else {
        //         return Err(syn::Error::new(
        //             input.span(),
        //             format!("Unknown key: {}, expected name, file or package", f.name),
        //         ));
        //     }
        // }
        // Ok(out)
    }
}

pub struct VarMeta {
    pub num: LitInt,
    pub name: LitStr,
}

impl Parse for VarMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let num: LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;
        let name: LitStr = input.parse()?;

        Ok(Self { num, name })
    }
}

pub struct FieldMeta {
    pub num: LitInt,
    pub name: LitStr,
    pub kind: FieldKind,
    pub freq: Frequency,
}

fn err(s: Span, m: impl Display) -> syn::Error {
    syn::Error::new(s, m)
}

impl Parse for FieldMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let num: LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;
        let name: LitStr = input.parse()?;
        let _: Token![,] = input.parse().map_err(|e| err(e.span(), "Expected field kind"))?;
        let kind: FieldKind = input.parse()?;
        let freq = if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            input.parse()?
        } else {
            Frequency::Singular
        };

        Ok(Self { num, name, kind, freq })
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldKind {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
            FieldKind::Map(k) => write!(f, "map({},{})", &k.0, &k.1),
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
pub enum Frequency {
    #[default]
    Raw,
    Singular,
    Optional,
    Repeated,
    Required,
    Packed,
}

impl Frequency {
    pub fn is_multi(&self) -> bool {
        matches!(self, Self::Repeated | Self::Packed)
    }
    pub fn binformat_suffix(&self) -> &'static str {
        match self {
            Frequency::Raw => "raw",
            Frequency::Singular | Frequency::Required => "single",
            Frequency::Packed => "packed",
            Frequency::Repeated => "repeated",
            Frequency::Optional => "optional",
        }
    }
    pub fn textformat_suffix(&self) -> &'static str {
        match self {
            Frequency::Raw => "raw",
            Frequency::Singular | Frequency::Required => "single",
            Frequency::Repeated | Frequency::Packed => "repeated",
            Frequency::Optional => "optional",
        }
    }
}

impl Parse for Frequency {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(if ident == "raw" {
            Frequency::Raw
        } else if ident == "singular" {
            Frequency::Singular
        } else if ident == "optional" {
            Frequency::Optional
        } else if ident == "repeated" {
            Frequency::Repeated
        } else if ident == "required" {
            Frequency::Required
        } else if ident == "packed" {
            Frequency::Packed
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

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Frequency::Raw => write!(f, "raw"),
            Frequency::Singular => write!(f, "singular"),
            Frequency::Optional => write!(f, "optional"),
            Frequency::Repeated => write!(f, "repeated"),
            Frequency::Required => write!(f, "singular"),
            Frequency::Packed => write!(f, "packed"),
        }
    }
}

pub struct OneOfMeta {
    pub nums: Vec<LitInt>,
    pub names: Vec<LitStr>,
}

impl Parse for OneOfMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nums;
        let _ = bracketed!(nums in input);
        let nums = Punctuated::<LitInt, Token![,]>::parse_terminated(&nums)?;
        let _: Token![,] = input.parse()?;
        let names;
        let _ = bracketed!(names in input);
        let names = Punctuated::<LitStr, Token![,]>::parse_terminated(&names)?;

        Ok(Self {
            nums: nums.into_iter().collect(),
            names: names.into_iter().collect(),
        })
    }
}
