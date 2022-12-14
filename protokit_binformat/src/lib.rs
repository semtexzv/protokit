#![feature(trait_alias)]
#![feature(const_mut_refs)]
#![feature(new_uninit)]

use std::ops::{Deref, DerefMut};

use anyhow::bail;
pub use anyhow::Result;
use bytes::BytesMut;
use format::*;
use integer_encoding::VarInt;
use unk::*;

use crate::format::{Format, RawVInt};

mod buffer;
#[doc(hidden)]
pub mod format;
#[doc(hidden)]
pub mod unk;
pub mod varint;

pub trait Decodable {
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>>;
}

impl<T> Decodable for Box<T>
where
    T: Decodable,
{
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        self.deref_mut().merge_field(tag, buf)
    }
}

impl<T> Decodable for Option<Box<T>>
where
    T: Decodable + Default,
{
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        self.get_or_insert_with(Default::default)
            .deref_mut()
            .merge_field(tag, buf)
    }
}

pub type ReadBuffer<'a> = &'a [u8];

pub type WriteBuffer = Vec<u8>;

pub trait Encodable {
    fn qualified_name(&self) -> &'static str;
    fn encode(&self, buf: &mut WriteBuffer) -> Result<()>;
}

impl<T> Encodable for Box<T>
where
    T: Encodable,
{
    fn qualified_name(&self) -> &'static str {
        self.deref().qualified_name()
    }

    fn encode(&self, buf: &mut WriteBuffer) -> Result<()> {
        self.deref().encode(buf)
    }
}

impl<T> Encodable for Option<Box<T>>
where
    T: Encodable + Default,
{
    fn qualified_name(&self) -> &'static str {
        T::default().qualified_name()
    }

    fn encode(&self, buf: &mut WriteBuffer) -> Result<()> {
        match self {
            Some(v) => Encodable::encode(v, buf),
            None => Ok(()),
        }
    }
}

pub trait Message {}

impl<T> Message for T where T: Decodable + Encodable + Default {}

impl Encodable for () {
    #[inline(always)]
    fn qualified_name(&self) -> &'static str {
        ""
    }

    #[inline(always)]
    fn encode(&self, _buf: &mut WriteBuffer) -> Result<()> {
        Ok(())
    }
}

impl Decodable for () {
    #[inline(never)]
    fn merge_field<'i, 'b>(&'i mut self, tag: u32, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        match (tag & 0b111) as u8 {
            VINT => {
                let (_vint, len) = u64::decode_var(buf).unwrap();
                Ok(&buf[len ..])
            }
            FIX64 => {
                let mut v = 0;
                buf = Format::<Fix>::decode(&mut v, buf)?;
                Ok(buf)
            }
            FIX32 => {
                let mut v = 0;
                buf = Format::<Fix>::decode(&mut v, buf)?;
                Ok(buf)
            }
            //TODO: Implement optimistic parsing into nested messages
            LENDELIM => {
                let (datalen, vlen) = u64::decode_var(buf).unwrap();
                Ok(&buf[(datalen as usize) + vlen ..])
            }
            other => bail!("Unknown wire type {other}"),
        }
    }
}

#[inline(never)]
pub fn decode_into<'a, T: Decodable + ?Sized>(mut buf: &'a [u8], obj: &mut T) -> Result<&'a [u8]> {
    let mut tag = 0xFF;
    while !buf.is_empty() {
        buf = Format::<RawVInt>::decode(&mut tag, buf)?;
        buf = obj.merge_field(tag, buf)?;
    }
    Ok(buf)
}

#[inline(never)]
pub fn from_slice<T: Decodable + Default>(buf: &[u8]) -> Result<T> {
    let mut obj = T::default();
    let left = decode_into(buf, &mut obj)?;
    assert_eq!(left.len(), 0);
    Ok(obj)
}

#[inline(never)]
pub fn to_vec<T: Encodable>(v: &T) -> Result<WriteBuffer> {
    let mut buf = WriteBuffer::new();
    v.encode(&mut buf)?;
    Ok(buf)
}
