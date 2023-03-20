#![feature(trait_alias)]
#![feature(const_mut_refs)]
#![feature(new_uninit)]

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub use anyhow::Result;
use anyhow::{anyhow, bail};
use format::*;
use integer_encoding::VarInt;
pub use unknown::*;

use crate::format::Format;

mod buffer;
#[doc(hidden)]
pub mod format;
#[doc(hidden)]
pub mod unknown;
pub mod varint;

pub trait Decodable {
    fn decode<'i, 'b>(&'i mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut len = 0;
        let mut tag = 0xFF;

        buf = Format::<RawVInt>::decode(&mut len, buf)?;
        if buf.len() < len {
            bail!("Not enough data")
        }
        let (mut inner_buf, outer_buf) = buf.split_at(len);
        while !inner_buf.is_empty() {
            inner_buf = Format::<RawVInt>::decode(&mut tag, inner_buf)?;
            inner_buf = self.merge_field(tag, inner_buf)?;
        }
        Ok(outer_buf)
    }
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
    // Initialization needed here (zero-sized nested messages must be initialized to Some if present)
    fn decode<'i, 'b>(&'i mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        self.get_or_insert_with(Default::default)
            .deref_mut()
            .decode(buf)
    }
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

pub trait Message: Encodable + Decodable + Default {}

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
        if tag >> 3 == 0 {
            bail!("Field num 0 is illegal")
        }
        match (tag & 0b111) as u8 {
            VINT => {
                let (_vint, len) = u64::decode_var(buf).ok_or_else(|| anyhow!("reading uint"))?;
                Ok(&buf[len..])
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
                let (datalen, vlen) = u64::decode_var(buf).ok_or_else(|| anyhow!("reading uint"))?;
                if buf.len() < (datalen as usize).saturating_add(vlen) {
                    return Err(anyhow::Error::msg("Mising data"));
                }
                Ok(&buf[(datalen as usize) + vlen..])
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
pub fn decode<T: Decodable + Default>(buf: &[u8]) -> Result<T> {
    let mut obj = T::default();
    let left = decode_into(buf, &mut obj)?;
    assert_eq!(left.len(), 0);
    Ok(obj)
}

#[inline(never)]
pub fn encode<T: Encodable>(v: &T) -> Result<WriteBuffer> {
    let mut buf = WriteBuffer::new();
    v.encode(&mut buf)?;
    Ok(buf)
}

pub trait ShouldEncode {
    fn should_encode(&self, proto3: bool) -> bool {
        true
    }
}

macro_rules! should_not_default {
    ($($t:ty),*) => {
        $(impl ShouldEncode for $t {
            fn should_encode(&self, proto3: bool) -> bool {
                self != &<$t>::default()
            }
        })*
    };
}

should_not_default!(i32, u32, i64, u64, f32, f64, bool, String);

impl<T> ShouldEncode for Vec<T> {
    fn should_encode(&self, proto3: bool) -> bool {
        !self.is_empty()
    }
}

impl<T> ShouldEncode for Option<T> {
    fn should_encode(&self, proto3: bool) -> bool {
        self.is_some()
    }
}

impl<K, V> ShouldEncode for HashMap<K, V> {
    fn should_encode(&self, proto3: bool) -> bool {
        !self.is_empty()
    }
}
