#![allow(nonstandard_style)]
#![allow(unused_braces)]

use std::fmt::Write;

use anyhow::{bail, Result};
use binformat::ReadBuffer;
use textformat::{Indent, INDENT};

use crate::textformat::ast::{FieldName, FieldValue};
use crate::textformat::{Context, Field};
use crate::{binformat, textformat};

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Any {
    pub type_url: String,
    pub value: Vec<u8>,
}

impl Any {
    pub fn pack<O: binformat::Encodable>(obj: &O) -> Result<Self> {
        let value = binformat::encode(obj)?;
        Ok(Self {
            type_url: obj.qualified_name().to_string(),
            value,
        })
    }

    pub fn unpack<O: binformat::Decodable + binformat::Encodable + Default>(&self) -> Result<O> {
        let mut obj = O::default();
        if !self.type_url.contains(obj.qualified_name()) {
            bail!(
                "Any contains {}, tried to deserialize type: {}",
                self.type_url,
                obj.qualified_name()
            );
        }
        binformat::decode_into(self.value.as_slice(), &mut obj)?;
        Ok(obj)
    }

    pub fn set(type_url: String, value: Vec<u8>) -> Self {
        Self { type_url, value }
    }
}

impl binformat::Decodable for Any {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        use crate::binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.type_url, buf)?;
            }
            18u32 => {
                buf = Format::<Bytes>::decode(&mut self.value, buf)?;
            }
            tag => bail!("Unsupported tag: {tag}"),
        }

        Ok(buf)
    }
}

impl binformat::Encodable for Any {
    fn qualified_name(&self) -> &'static str {
        "google.protobuf.Any"
    }

    fn encode(&self, buf: &mut binformat::WriteBuffer) -> Result<()> {
        use crate::binformat::format::*;
        Format::<Bytes>::encode(&self.type_url, 10u32 >> 3, buf)?;
        Format::<Bytes>::encode(&self.value, 18u32 >> 3, buf)?;
        Ok(())
    }
}

impl textformat::Decodable for Any {
    fn merge_field(&mut self, ctx: &Context, name: &FieldName, value: &FieldValue) -> Result<()> {
        match name {
            FieldName::Normal("type_url") => {
                Field::merge(&mut self.type_url, ctx, value)?;
            }
            FieldName::Normal("value") => {
                Field::merge(&mut self.value, ctx, value)?;
            }
            FieldName::Any(_domain, msg_name) => {
                if let Some(msg) = ctx.find_message(msg_name) {
                    let mut newmsg = msg.new();
                    self.type_url = format!("type.googleapis.com/{}", newmsg.qualified_name());

                    Field::merge(newmsg.as_mut(), ctx, value)?;
                    newmsg.as_bin_encodable().encode(&mut self.value)?;
                    return Ok(());
                } else {
                    bail!("Could not find {msg_name} in internal type regitry: {:?}", ctx)
                }
            }
            _ => bail!("{name:?} is not recognized"),
        }
        Ok(())
    }
}

impl textformat::Encodable for Any {
    fn encode(&self, ctx: &Context, pad: usize, out: &mut String) -> Result<()> {
        if let Some(reg) = ctx.registry {
            let typ = self.type_url.find('/').map(|pos| &self.type_url[pos + 1 ..]);
            if let Some(msg) = typ.and_then(|typ| reg.messages.get(typ)) {
                let mut msg = msg.new();
                out.indent(pad);
                writeln!(out, "[{}] {{", self.type_url)?;
                binformat::decode_into(&self.value, msg.as_bin_decodable())?;
                msg.as_text_encodable().encode(ctx, pad + INDENT, out)?;
                out.indent(pad);
                writeln!(out, "}}")?;
                return Ok(());
            }
        }
        out.indent(pad);
        out.push_str("type_url: ");
        Field::format(&self.type_url, ctx, pad + INDENT, out)?;
        out.push('\n');

        out.indent(pad);
        out.push_str("value: ");
        Field::format(&self.value, ctx, pad + INDENT, out)?;
        out.push('\n');

        Ok(())
    }
}
