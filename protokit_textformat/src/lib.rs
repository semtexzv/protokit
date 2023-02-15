use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::{Debug, Write};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

pub use anyhow::{bail, Result};
use ast::{FieldName, FieldValue, Literal};
use nom::AsChar;
use parser::textproto;

pub mod ast;
pub mod parser;
pub mod reflect;

pub const INDENT: usize = 2;

pub trait Indent {
    fn indent(&mut self, level: usize);
}

impl Indent for String {
    fn indent(&mut self, level: usize) {
        let s = unsafe { self.as_mut_vec() };
        let len = s.len();
        if len > 0 && s[len - 1] == b'\n' {
            s.resize(len + level, b' ');
        }
    }
}

#[derive(Debug, Default)]
pub struct Context<'c> {
    pub registry: Option<&'c crate::reflect::Registry>,
}

impl Context<'_> {
    pub fn find_message(&self, name: &str) -> Option<&dyn crate::reflect::AnyMessage> {
        self.registry.and_then(|reg| reg.messages.get(name).map(|v| v.as_ref()))
    }
}

pub trait Decodable {
    fn merge_field(&mut self, ctx: &Context, name: &FieldName, value: &FieldValue) -> anyhow::Result<()>;
}

impl<T> Decodable for Box<T>
    where
        T: Decodable,
{
    fn merge_field(&mut self, ctx: &Context, name: &FieldName, value: &FieldValue) -> Result<()> {
        self.deref_mut().merge_field(ctx, name, value)
    }
}

pub trait Encodable {
    fn encode(&self, ctx: &Context, pad: usize, out: &mut String) -> Result<()>;
}

impl<T> Encodable for Box<T>
    where
        T: Encodable,
{
    fn encode(&self, ctx: &Context, pad: usize, out: &mut String) -> Result<()> {
        self.deref().encode(ctx, pad, out)
    }
}

pub trait Field {
    fn format(&self, ctx: &Context, pad: usize, out: &mut String) -> fmt::Result;
    fn merge(&mut self, ctx: &Context, value: &FieldValue) -> anyhow::Result<()> {
        match value {
            FieldValue::Scalar(s) => self.merge_scalar(ctx, s),
            FieldValue::Message(ml) => self.merge_message(ctx, ml),
            #[cfg(feature = "map_syntax")]
            FieldValue::Map(m) => self.merge_map(ctx, m),
            _other => bail!("Unimplemented default merge method for compound structures"),
        }
    }
    fn merge_scalar(&mut self, _ctx: &Context, _value: &Literal) -> Result<()> {
        panic!("Unexpected scalar: {}", std::any::type_name::<Self>())
    }
    #[cfg(feature = "map_syntax")]
    fn merge_map(&mut self, _ctx: &Context, _value: &[ast::MapField]) -> Result<()> {
        bail!("Unexpected map")
    }
    fn merge_message(&mut self, _ctx: &Context, _value: &[ast::Field]) -> Result<()> {
        bail!("Unexpected literal")
    }
}

impl<T> Field for Option<T>
    where
        T: Field + Default,
{
    fn format(&self, ctx: &Context, pad: usize, out: &mut String) -> fmt::Result {
        if let Some(v) = self {
            v.format(ctx, pad, out)
        } else {
            Ok(())
        }
    }
    fn merge(&mut self, ctx: &Context, value: &FieldValue) -> Result<()> {
        self.get_or_insert(T::default()).merge(ctx, value)
    }
}

impl Field for String {
    fn format(&self, _ctx: &Context, _pad: usize, out: &mut String) -> fmt::Result {
        write!(out, "\"{self}\"")
    }
    fn merge_scalar(&mut self, _ctx: &Context, value: &Literal) -> Result<()> {
        match value {
            Literal::String(s) => {
                self.clear();
                for s in s {
                    self.push_str(s)
                }
            }
            other => bail!("Unexpected value: {other:?}"),
        }
        Ok(())
    }
}

impl Field for Box<str> {
    fn format(&self, _ctx: &Context, _pad: usize, out: &mut String) -> fmt::Result {
        write!(out, "\"{self}\"")
    }
    fn merge_scalar(&mut self, _ctx: &Context, value: &Literal) -> Result<()> {
        match value {
            Literal::String(s) => {
                let mut buf = String::with_capacity(s.iter().map(|l| l.len()).sum());
                for s in s {
                    buf.push_str(s)
                }
                *self = buf.into_boxed_str();
            }
            other => bail!("Unexpected value: {other:?}"),
        }
        Ok(())
    }
}

macro_rules! impl_mergable_int {
    ($($t:ty)*) => {$(
        impl Field for $t {
            fn format(&self, _ctx: &Context, pad: usize, out: &mut String) -> fmt::Result {
                out.indent(pad);
                write!(out, "{self}")
            }
            fn merge(&mut self, _ctx: &Context, value: &FieldValue) -> Result<()> {
                match value {
                    FieldValue::Scalar(Literal::Int(v)) => {
                        *self = (*v).try_into().unwrap();
                    },
                    other => bail!("Unexpected value: {other:?}"),
                }
                Ok(())
            }
            fn merge_scalar(&mut self, _: &Context, value: &Literal) -> Result<()> {
                match value {
                    Literal::Int(v) => {
                        *self = (*v).try_into().unwrap();
                    },
                    other => bail!("Unexpected value: {other:?}"),
                }
                Ok(())
            }
        }
    )*};
}

impl_mergable_int!(i32 u32 i64 u64 isize usize);

impl Field for bool {
    fn format(&self, _ctx: &Context, _pad: usize, out: &mut String) -> fmt::Result {
        write!(out, "{self}")
    }
    fn merge_scalar(&mut self, _ctx: &Context, value: &Literal) -> Result<()> {
        match value {
            Literal::Int(v) => *self = *v != 0,
            Literal::Identifier(id) => match *id {
                "True" | "true" | "t" => *self = true,
                "False" | "false" | "f" => *self = false,
                other => bail!("Unrecognized bool value {other}"),
            },
            other => bail!("Unrecognized bool value {other:?}"),
        };
        Ok(())
    }
}

impl Field for f64 {
    fn format(&self, _ctx: &Context, _pad: usize, out: &mut String) -> fmt::Result {
        write!(out, "{self}")
    }

    fn merge_scalar(&mut self, _ctx: &Context, value: &Literal) -> Result<()> {
        match value {
            Literal::String(_) => {}
            Literal::Float(f) => *self = (*f).try_into().unwrap(),
            // TODO: Fix
            Literal::Int(v) => *self = (*v as f64).try_into().unwrap(),
            Literal::Identifier(_) => {}
            Literal::SignedIdentifier(_, _) => {}
        };
        Ok(())
    }
}

impl Field for f32 {
    fn format(&self, _ctx: &Context, _pad: usize, _out: &mut String) -> fmt::Result {
        todo!()
    }

    fn merge_scalar(&mut self, _ctx: &Context, value: &Literal) -> Result<()> {
        match value {
            Literal::String(_) => {}
            Literal::Float(f) => *self = (*f as f32).try_into().unwrap(),
            // TODO: Fix
            Literal::Int(v) => *self = (*v as f32).try_into().unwrap(),
            Literal::Identifier(_) => {}
            Literal::SignedIdentifier(_, _) => {}
        };
        Ok(())
    }
}

impl<T> Field for Vec<T>
    where
        T: Field + Default,
{
    fn format(&self, ctx: &Context, pad: usize, out: &mut String) -> fmt::Result {
        if self.len() != 1 {
            out.indent(pad);
            out.push_str("[\n");
        }
        for (_i, it) in self.iter().enumerate() {
            it.format(ctx, pad + INDENT, out)?;
            out.push_str(",\n");
        }
        if self.len() != 1 {
            out.indent(pad);
            out.push(']');
        }
        Ok(())
    }
    fn merge(&mut self, ctx: &Context, value: &FieldValue) -> Result<()> {
        let mut target = T::default();
        match value {
            val @ (FieldValue::Scalar(_) | FieldValue::Message(_)) => {
                target.merge(ctx, val)?;
                self.push(target);
            }

            #[cfg(feature = "map_syntax")]
            val @ FieldValue::Map(_) => target.merge(ctx, val)?,

            FieldValue::ScalarList(sl) => {
                for lit in sl {
                    target.merge_scalar(ctx, lit)?;
                    self.push(std::mem::take(&mut target));
                }
            }
            FieldValue::MessageList(ml) => {
                for lit in ml {
                    target.merge_message(ctx, lit)?;
                    self.push(std::mem::take(&mut target));
                }
            }
            #[cfg(feature = "map_syntax")]
            FieldValue::MapList(ml) => {
                for map in ml {
                    target.merge_map(ctx, map)?;
                    self.push(std::mem::take(&mut target));
                }
            }
        }
        Ok(())
    }
}

impl Field for Vec<u8> {
    fn format(&self, _ctx: &Context, _pad: usize, out: &mut String) -> fmt::Result {
        write!(out, "\"")?;
        for v in self {
            if v.is_ascii_alphanumeric() {
                out.push(v.as_char());
            } else {
                write!(out, "\\x{v:x}")?;
            }
        }
        write!(out, "\"")?;
        Ok(())
    }
    fn merge_scalar(&mut self, _ctx: &Context, _value: &Literal) -> Result<()> {
        match _value {
            Literal::String(_s) => {
                panic!()
            }
            other => bail!("Bytevec can't deserialize {other:?} "),
        }
    }
}

impl<K, V> Field for HashMap<K, V>
    where
        K: Field + Default + Hash + Eq,
        V: Field + Default,
{
    #[cfg(feature = "map_syntax")]
    fn format(&self, ctx: &Context, pad: usize, out: &mut String) -> fmt::Result {
        out.indent(pad);
        out.push_str("}\n");

        for (key, _val) in self.iter() {
            out.indent(pad);
            Field::format(key, ctx, pad, out)?;
            out.push_str(": ");
            Field::format(key, ctx, pad, out)?;
            out.push('\n');
        }

        out.indent(pad);
        out.push_str("}\n");
        Ok(())
    }

    #[cfg(not(feature = "map_syntax"))]
    fn format(&self, _ctx: &Context, _pad: usize, _out: &mut String) -> fmt::Result {
        if self.len() != 1 {
            out.indent(pad);
            out.push_str("[\n");
        }
        for (key, val) in self.iter() {
            out.indent(pad);
            out.push_str("{\n");

            out.indent(pad);
            out.push_str("key: ");
            Field::format(key, ctx, pad, out)?;
            out.push('\n');

            out.indent(pad);
            out.push_str("value: ");
            Field::format(key, ctx, pad, out)?;
            out.push('\n');

            out.indent(pad);
            out.push_str("}\n");
        }
        if self.len() != 1 {
            out.indent(pad);
            out.push(']');
        }
        Ok(())
    }

    fn merge(&mut self, ctx: &Context, value: &FieldValue) -> Result<()> {
        match value {
            FieldValue::Message(m) => self.merge_message(ctx, m)?,
            FieldValue::MessageList(ml) => {
                for m in ml {
                    self.merge_message(ctx, m)?
                }
            }
            #[cfg(feature = "map_syntax")]
            FieldValue::Map(map) => self.merge_map(ctx, map)?,
            #[cfg(feature = "map_syntax")]
            FieldValue::MapList(ml) => {
                for it in ml {
                    self.merge_map(ctx, it)?;
                }
            }

            other => bail!("HashMap can't deserialize {other:?} "),
        }
        Ok(())
    }
    #[cfg(feature = "map_syntax")]
    fn merge_map(&mut self, ctx: &Context, value: &[ast::MapField]) -> Result<()> {
        for field in value {
            if value.len() > 2 {
                bail!("Unknown fields found in map")
            }

            let mut key = K::default();
            let mut val = V::default();

            key.merge_scalar(ctx, &field.key)?;
            val.merge(ctx, &field.value)?;

            self.insert(key, val);
        }
        Ok(())
    }

    fn merge_message(&mut self, ctx: &Context, value: &[ast::Field]) -> Result<()> {
        let kf = value.iter().rfind(|f| f.name == FieldName::Normal("key")).unwrap();
        let vf = value.iter().rfind(|f| f.name == FieldName::Normal("value")).unwrap();

        if value.len() > 2 {
            bail!("Unknown fields found in map")
        }

        let mut key = K::default();
        let mut val = V::default();

        key.merge(ctx, &kf.value)?;
        val.merge(ctx, &vf.value)?;
        self.insert(key, val);
        Ok(())
    }
}

impl<K, V> Field for BTreeMap<K, V>
    where
        K: Field + Default + Ord + Eq,
        V: Field + Default,
{
    fn format(&self, _ctx: &Context, _pad: usize, _out: &mut String) -> fmt::Result {
        todo!()
    }

    fn merge(&mut self, ctx: &Context, value: &FieldValue) -> Result<()> {
        match value {
            FieldValue::Message(m) => self.merge_message(ctx, m)?,
            FieldValue::MessageList(ml) => {
                for m in ml {
                    self.merge_message(ctx, m)?
                }
            }

            #[cfg(feature = "map_syntax")]
            FieldValue::Map(map) => self.merge_map(ctx, map)?,
            #[cfg(feature = "map_syntax")]
            FieldValue::MapList(ml) => {
                for it in ml {
                    self.merge_map(ctx, it)?;
                }
            }

            other => bail!("HashMap can't deserialize {other:?} "),
        }
        Ok(())
    }

    #[cfg(feature = "map_syntax")]
    fn merge_map(&mut self, ctx: &Context, value: &[ast::MapField]) -> Result<()> {
        for field in value {
            if value.len() > 2 {
                bail!("Unknown fields found in map")
            }

            let mut key = K::default();
            let mut val = V::default();

            key.merge_scalar(ctx, &field.key)?;
            val.merge(ctx, &field.value)?;

            self.insert(key, val);
        }
        Ok(())
    }

    fn merge_message(&mut self, ctx: &Context, value: &[ast::Field]) -> Result<()> {
        let kf = value.iter().rfind(|f| f.name == FieldName::Normal("key")).unwrap();
        let vf = value.iter().rfind(|f| f.name == FieldName::Normal("value")).unwrap();

        if value.len() > 2 {
            bail!("Unknown fields found in map")
        }

        let mut key = K::default();
        let mut val = V::default();

        key.merge(ctx, &kf.value)?;
        val.merge(ctx, &vf.value)?;
        self.insert(key, val);
        Ok(())
    }
}

impl<M> Field for M
    where
        M: Decodable + Encodable + ?Sized,
{
    fn format(&self, ctx: &Context, pad: usize, out: &mut String) -> fmt::Result {
        out.indent(pad);
        out.push_str("{\n");
        self.encode(ctx, pad + INDENT, out).unwrap();
        out.indent(pad);
        out.push('}');
        Ok(())
    }

    fn merge_message(&mut self, ctx: &Context, value: &[ast::Field]) -> Result<()> {
        for f in value {
            self.merge_field(ctx, &f.name, &f.value)?
        }
        Ok(())
    }
}

pub fn decode_into(i: &str, registry: &crate::reflect::Registry, o: &mut impl Decodable) -> Result<()> {
    let (rest, tp) = textproto(i).unwrap();
    assert_eq!(rest, "");
    let ctx = Context {
        registry: Some(registry),
    };

    for f in tp.fields.iter() {
        o.merge_field(&ctx, &f.name, &f.value)?
    }

    Ok(())
}

#[inline(never)]
pub fn decode<D: Default + Decodable>(i: &str, registry: &crate::reflect::Registry) -> Result<D> {
    let mut out = D::default();
    decode_into(i, registry, &mut out)?;
    Ok(out)
}


pub fn encode_into(o: &impl Encodable, reg: crate::reflect::Registry, out: &mut String) -> Result<()> {
    o.encode(&Context { registry: Some(&reg) }, 0, out)?;

    Ok(())
}

#[inline(never)]
pub fn encode(o: &impl Encodable, reg: &crate::reflect::Registry) -> Result<String> {
    let mut out = String::new();
    let ctx = Context { registry: Some(reg) };

    o.encode(&ctx, 0, &mut out)?;

    Ok(out)
}
