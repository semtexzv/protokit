use crate::{BinProto, emit_raw, Error, InputStream, MAP_WIRE, OutputStream, unknown_wire};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Vec<u8>),
    Group(Vec<Field>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub num: u32,
    pub val: Value,
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownFields {
    pub fields: Option<Box<Vec<Field>>>,
}

impl<'buf> BinProto<'buf> for UnknownFields {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> crate::Result<()> {
        let f = self.fields.get_or_insert_with(Default::default);
        Self::merge_one(f, tag_wire, stream)
    }

    fn encode(&self, stream: &mut OutputStream) {
        for f in self.fields.iter().map(|v| v.iter()).flatten() {
            Self::emit_field(f, stream)
        }
    }
}

impl UnknownFields {
    fn emit_field(f: &Field, stream: &mut OutputStream) {
        match &f.val {
            Value::Varint(v) => emit_raw(v, f.num << 3 | crate::VARINT as u32, stream, OutputStream::varint),
            Value::Fixed32(v)  => emit_raw(v, f.num << 3 | crate::FIX32 as u32, stream, OutputStream::fixed32),
            Value::Fixed64(v)  => emit_raw(v, f.num << 3 | crate::FIX64 as u32, stream, OutputStream::fixed64),
            Value::Bytes(v) => emit_raw(v, f.num << 3 | crate::VARINT as u32, stream, OutputStream::bytes),
            Value::Group(v) => {
                stream._varint(f.num << 3 | crate::SGRP as u32);
                for v in v {
                    Self::emit_field(v, stream)
                }
                stream._varint(f.num << 3 | crate::EGRP as u32)
            },
        }
    }
    fn merge_one(fields: &mut Vec<Field>, tag: u32, stream: &mut InputStream) -> crate::Result<()> {
        match (tag as u8 & MAP_WIRE) as u8 {
            crate::VARINT => fields.push(Field {
                num: tag >> 3,
                val: Value::Varint(stream._varint()?),
            }),
            crate::FIX32 => fields.push(Field {
                num: tag >> 3,
                val: Value::Fixed32(stream._fixed()?),
            }),
            crate::FIX64 => fields.push(Field {
                num: tag >> 3,
                val: Value::Fixed64(stream._fixed()?),
            }),
            crate::BYTES => fields.push(Field {
                num: tag >> 3,
                val: Value::Bytes(stream._bytes()?.to_vec()),
            }),
            crate::SGRP => {
                let mut inner = vec![];
                Self::merge_until_egrp(&mut inner, ((tag >> 3) << 3) | crate::EGRP as u32, stream)?;
                fields.push(Field {
                    num: tag >> 3,
                    val: Value::Group(inner),
                });
            }
            wire => return unknown_wire(wire),
        }
        Ok(())
    }
    fn merge_until_egrp(fields: &mut Vec<Field>, gtag: u32, stream: &mut InputStream) -> crate::Result<()> {
        while stream.len() > 0 {
            let tag = stream._varint()?;
            if tag == gtag {
                return Ok(());
            }
            Self::merge_one(fields, tag, stream)?
        }
        return Err(Error::UnterminatedGroup);
    }
}