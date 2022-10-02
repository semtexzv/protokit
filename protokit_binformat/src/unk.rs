use anyhow::{bail, Result};
use integer_encoding::VarInt;

use crate::{WriteBuffer, ReadBuffer, Decodable, Format, Encodable, Fix};

pub(crate) const VINT: u8 = 0;
pub(crate) const FIX64: u8 = 1;
pub(crate) const LENDELIM: u8 = 2;
pub(crate) const FIX32: u8 = 5;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtoValue {
    VInt(u64),
    Fix32(u32),
    Fix64(u64),

    // TODO: use boxed slice
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtoField {
    pub tag: u32,
    pub value: ProtoValue,
}

impl Default for ProtoField {
    fn default() -> Self {
        Self {
            tag: 0,
            value: ProtoValue::VInt(0),
        }
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownFields {
    pub fields: Option<Box<Vec<ProtoField>>>,
}

impl Decodable for ProtoField {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        self.tag = tag;
        match (tag & 0b111) as u8 {
            VINT => {
                let (vint, len) = u64::decode_var(buf).unwrap();
                self.value = ProtoValue::VInt(vint);
                Ok(&buf[len ..])
            }
            FIX64 => {
                let mut v = 0;
                buf = Format::<Fix>::decode(&mut v, buf)?;
                self.value = ProtoValue::Fix64(v);
                Ok(buf)
            }
            FIX32 => {
                let mut v = 0;
                buf = Format::<Fix>::decode(&mut v, buf)?;
                self.value = ProtoValue::Fix32(v);
                Ok(buf)
            }
            //TODO: Implement optimistic parsing into nested messages
            LENDELIM => {
                let (datalen, vlen) = u64::decode_var(buf).unwrap();
                self.value = ProtoValue::Bytes(Vec::from(&buf[vlen .. datalen as usize + vlen]));
                Ok(&buf[(datalen as usize) + vlen ..])
            }
            other => bail!("Unknown wire type {other}"),
        }
    }
}

impl Encodable for ProtoField {
    fn qualified_name(&self) -> &'static str {
        ""
    }

    fn encode(&self, _buf: &mut crate::WriteBuffer) -> Result<()> {
        Ok(())
        // match &self.value {
        //     ProtoValue::VInt(v) => VInt::encode(v, self.tag, buf),
        //     ProtoValue::Fix32(v) => Fix::encode(v, self.tag, buf),
        //     ProtoValue::Fix64(v) => Fix::encode(v, self.tag, buf),
        //     ProtoValue::Bytes(v) => Bytes::encode(v, self.tag, buf),
        // }
    }
}

impl Decodable for UnknownFields {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let fields = self.fields.get_or_insert_with(Default::default);
        let mut field = ProtoField::default();
        buf = field.merge_field(tag, buf)?;
        fields.push(field);
        Ok(buf)
    }
}

impl Encodable for UnknownFields {
    fn qualified_name(&self) -> &'static str {
        ""
    }

    fn encode(&self, buf: &mut WriteBuffer) -> Result<()> {
        if let Some(f) = &self.fields {
            for f in f.iter() {
                Encodable::encode(f, buf)?;
            }
        }
        Ok(())
    }
}
