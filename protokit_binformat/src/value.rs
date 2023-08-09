use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{
    emit_raw, unknown_tag, unknown_wire, BinProto, BytesLike, Error, InputStream, OutputStream, SizeStack, MASK_WIRE,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value<B> {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Vec<u8>),
    Group(Vec<Field<B>>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field<B> {
    pub num: u32,
    pub val: Value<B>,
    pub _m: PhantomData<B>,
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownFields<B> {
    // Double indirection to keep this struct one pointer wide.
    pub fields: Option<Box<Vec<Field<B>>>>,
}

pub type UnknownFieldsOwned = UnknownFields<Vec<u8>>;
pub type UnknownFieldsBorrow<'buf> = UnknownFields<&'buf [u8]>;

impl<'buf, B: BytesLike<'buf>> BinProto<'buf> for UnknownFields<B> {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> crate::Result<()> {
        let f = self.fields.get_or_insert_with(Default::default);
        Self::merge_one(f, tag_wire, stream)
    }

    fn size(&self, _stack: &mut SizeStack) -> usize {
        todo!()
    }

    fn encode(&self, stream: &mut OutputStream) {
        for f in self.fields.iter().flat_map(|v| v.iter()) {
            Self::emit_field(f, stream)
        }
    }
}

impl<'buf, B: BytesLike<'buf>> UnknownFields<B> {
    fn emit_field(f: &Field<B>, stream: &mut OutputStream) {
        match &f.val {
            Value::Varint(v) => emit_raw(v, f.num << 3 | crate::VARINT as u32, stream, OutputStream::varint),
            Value::Fixed32(v) => emit_raw(v, f.num << 3 | crate::FIX32 as u32, stream, OutputStream::fixed32),
            Value::Fixed64(v) => emit_raw(v, f.num << 3 | crate::FIX64 as u32, stream, OutputStream::fixed64),
            Value::Bytes(v) => emit_raw(v, f.num << 3 | crate::VARINT as u32, stream, OutputStream::bytes),
            Value::Group(v) => {
                stream._varint(f.num << 3 | crate::SGRP as u32);
                for v in v {
                    Self::emit_field(v, stream)
                }
                stream._varint(f.num << 3 | crate::EGRP as u32)
            }
        }
    }
    fn merge_one(fields: &mut Vec<Field<B>>, tag: u32, stream: &mut InputStream<'buf>) -> crate::Result<()> {
        if tag >> 3 == 0 {
            return unknown_tag(tag);
        }
        match tag as u8 & MASK_WIRE {
            crate::VARINT => fields.push(Field {
                num: tag >> 3,
                val: Value::Varint(stream._varint()?),
                _m: Default::default(),
            }),
            crate::FIX32 => fields.push(Field {
                num: tag >> 3,
                val: Value::Fixed32(stream._fixed()?),
                _m: Default::default(),
            }),
            crate::FIX64 => fields.push(Field {
                num: tag >> 3,
                val: Value::Fixed64(stream._fixed()?),
                _m: Default::default(),
            }),
            crate::BYTES => fields.push(Field {
                num: tag >> 3,
                val: Value::Bytes(stream._bytes()?.to_vec()),
                _m: Default::default(),
            }),
            crate::SGRP => {
                let mut inner = vec![];
                Self::merge_until_egrp(&mut inner, ((tag >> 3) << 3) | crate::EGRP as u32, stream)?;
                fields.push(Field {
                    num: tag >> 3,
                    val: Value::Group(inner),
                    _m: Default::default(),
                });
            }
            wire => return unknown_wire(wire),
        }
        Ok(())
    }
    fn merge_until_egrp(fields: &mut Vec<Field<B>>, gtag: u32, stream: &mut InputStream<'buf>) -> crate::Result<()> {
        while !stream.is_empty() {
            let tag = stream._varint()?;
            if tag == gtag {
                return Ok(());
            }
            Self::merge_one(fields, tag, stream)?
        }
        Err(Error::UnterminatedGroup)
    }
}
