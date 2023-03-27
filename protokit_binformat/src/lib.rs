mod stream;

use std::collections::BTreeMap;
use std::mem  ::size_of;
use std::ops::{BitAnd, BitOr, Deref, DerefMut, Shl, Shr};
use std::str::Utf8Error;

pub use stream::{InputStream, OutputStream};
use thiserror::Error;

pub const VARINT: u8 = 0;
pub const FIX64: u8 = 1;
pub const BYTES: u8 = 2;
pub const SGRP: u8 = 3;
pub const EGRP: u8 = 4;
pub const FIX32: u8 = 5;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Run out of data")]
    Empty,
    #[error("Unknown tag: {0}")]
    Tag(u32),
    #[error("Unknown wire type: {0}")]
    Wire(u8),
    #[error("UTF8: {0}")]
    Utf8(#[from] Utf8Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cold]
pub fn unknown_tag<T>(tag: u32) -> Result<T> {
    Err(Error::Tag(tag))
}

#[cold]
pub fn unknown_wire<T>(w: u8) -> Result<T> {
    Err(Error::Wire(w))
}

pub trait BinProto {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream) -> Result<()>;
    fn encode(&self, stream: &mut OutputStream);
}

impl<T> BinProto for Box<T>
where
    T: BinProto,
{
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream) -> Result<()> {
        self.deref_mut().merge_field(tag_wire, stream)
    }

    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

impl<T> BinProto for Option<T>
where
    T: BinProto + Default,
{
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream) -> Result<()> {
        self.get_or_insert_with(Default::default).merge_field(tag_wire, stream)
    }

    fn encode(&self, stream: &mut OutputStream) {
        if let Some(v) = self {
            v.encode(stream)
        }
    }
}

pub trait Varint:
    Default
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Shl<i32, Output = Self>
    + Shr<i32, Output = Self>
    + BitAnd
    + BitOr<Self, Output = Self>
    + From<u8>
{
    fn max_shift() -> i32 {
        (size_of::<Self>() * 8) as i32
    }
    fn low_byte(self) -> u8;
}

impl Varint for u32 {
    fn low_byte(self) -> u8 {
        self as _
    }
}

impl Varint for u64 {
    fn low_byte(self) -> u8 {
        self as _
    }
}

impl Varint for i32 {
    fn low_byte(self) -> u8 {
        self as _
    }
}

impl Varint for i64 {
    fn low_byte(self) -> u8 {
        self as _
    }
}

impl Varint for usize {
    fn low_byte(self) -> u8 {
        self as _
    }
}

pub trait Sigint: Default + Clone + Copy {
    type Varint: Varint;
    fn encode(self) -> Self::Varint;
    fn decode(from: Self::Varint) -> Self;
}

impl Sigint for i32 {
    type Varint = u32;
    fn encode(self) -> Self::Varint {
        ((self << 1) ^ (self >> 31)) as u32
    }

    fn decode(from: Self::Varint) -> Self {
        ((from >> 1) ^ (-((from & 1) as i32)) as u32) as i32
    }
}

impl Sigint for i64 {
    type Varint = u64;
    fn encode(self) -> Self::Varint {
        ((self << 1) ^ (self >> 63)) as u64
    }

    fn decode(from: Self::Varint) -> Self {
        ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
    }
}

pub trait Fixed: Default + Sized + Clone + Copy {
    type WIRE: Sized;
    fn from_wire(v: Self::WIRE) -> Self;
    fn to_wire(self) -> Self::WIRE;
}

impl Fixed for i32 {
    type WIRE = Self;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for i64 {
    type WIRE = Self;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for u32 {
    type WIRE = u32;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for u64 {
    type WIRE = Self;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for f32 {
    type WIRE = u32;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_bits(Self::WIRE::from_le(v))
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_bits().to_le()
    }
}

impl Fixed for f64 {
    type WIRE = u64;

    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_bits(Self::WIRE::from_le(v))
    }

    fn to_wire(self) -> Self::WIRE {
        self.to_bits().to_le()
    }
}

pub trait Bytes {
    fn blen(&self) -> usize;
    fn bytes(&self) -> &[u8];
    fn push(&mut self, b: &[u8]) -> Result<()>;
}

impl Bytes for String {
    fn blen(&self) -> usize {
        self.len()
    }

    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn push(&mut self, b: &[u8]) -> Result<()> {
        self.push_str(std::str::from_utf8(b)?);
        Ok(())
    }
}

impl Bytes for Vec<u8> {
    fn blen(&self) -> usize {
        self.len()
    }

    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }

    fn push(&mut self, b: &[u8]) -> Result<()> {
        self.extend_from_slice(b);
        Ok(())
    }
}

pub fn merge_single<'a, T>(
    this: &mut T,
    stream: &mut InputStream<'a>,
    mapper: fn(&mut InputStream<'a>, &mut T) -> Result<()>,
) -> Result<()> {
    mapper(stream, this)
}

pub fn merge_optional<'a, T: Default>(
    this: &mut Option<T>,
    stream: &mut InputStream<'a>,
    mapper: fn(&mut InputStream<'a>, &mut T) -> Result<()>,
) -> Result<()> {
    mapper(stream, this.get_or_insert_with(Default::default))
}

pub fn merge_repeated<'a, T: Default>(
    this: &mut Vec<T>,
    stream: &mut InputStream<'a>,
    mapper: fn(&mut InputStream<'a>, &mut T) -> Result<()>,
) -> Result<()> {
    this.push(T::default());
    mapper(stream, this.last_mut().unwrap())
}

pub fn merge_packed<'a, T: Default>(
    this: &mut Vec<T>,
    stream: &mut InputStream<'a>,
    mapper: fn(&mut InputStream<'a>, &mut T) -> Result<()>,
) -> Result<()> {
    this.push(T::default());
    let len = stream._varint::<usize>()?;
    if stream.len() < len {
         return Err(Error::Empty);
    }
    let mut is = InputStream {
        buf: &stream.buf[stream.pos .. stream.pos + len],
        pos: 0,
        limit: len,
    };
    mapper(&mut is, this.last_mut().unwrap())?;
    stream.pos += len;
    Ok(())
}

pub fn merge_map<'a, K: Default + Ord, V: Default>(
    this: &mut BTreeMap<K, V>,
    stream: &mut InputStream<'a>,
    kmapper: fn(&mut InputStream<'a>, &mut K) -> Result<()>,
    vmapper: fn(&mut InputStream<'a>, &mut V) -> Result<()>,
) -> Result<()> {
    let len = stream._varint::<usize>()?;
    stream.limit(len);

    let mut key = K::default();
    let mut val = V::default();
    while stream.len() > 0 {
        // TODO: verify wire types
        match stream._varint::<u32>()? >> 3 {
            1 => merge_single(&mut key, stream, kmapper)?,
            2 => merge_single(&mut val, stream, vmapper)?,
            tag => unknown_tag(tag)?,
        }
    }
    stream.bump(len);
    this.insert(key, val);
    Ok(())
}

pub fn emit_single<T>(this: &T, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    stream._varint(tag);
    mapper(stream, tag, this);
}

pub fn emit_optional<T>(this: &Option<T>, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    if let Some(v) = this {
        stream._varint(tag);
        mapper(stream, tag, v);
    }
}

pub fn emit_repeated<T>(this: &Vec<T>, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    for v in this {
        stream._varint(tag);
        mapper(stream, tag, v)
    }
}

pub fn emit_packed<T>(this: &Vec<T>, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    if this.len() > 0 {
        stream._varint(tag);
        let mut o = OutputStream::default();
        for v in this {
            mapper(&mut o, tag, v);
        }
        stream._varint(o.len());
        stream._bytes(o.buf.as_slice());
    }
}

pub fn emit_map<K, V>(
    this: &BTreeMap<K, V>,
    tag: u32,
    ktag: u32,
    vtag: u32,
    stream: &mut OutputStream,
    kmapper: fn(&mut OutputStream, u32, &K),
    vmapper: fn(&mut OutputStream, u32, &V),
) {
    if this.len() > 0 {
        for (k, v) in this {
            stream._varint(tag);
            let mut o = OutputStream::default();
            kmapper(&mut o, ktag, k);
            vmapper(&mut o, vtag, v);
            stream._varint(o.len());
            stream._bytes(o.buf.as_slice());
        }
    }
}

pub fn decode<T: Default + BinProto>(b: &[u8]) -> Result<T> {
    let mut out = T::default();
    let mut is = InputStream::new(b);
    while is.len() > 0 {
        let tag = is._varint()?;
        out.merge_field(tag, &mut is)?;
    }
    Ok(out)
}

pub fn encode<T: BinProto>(b: &T) -> Result<Vec<u8>> {
    let mut out = OutputStream::default();
    b.encode(&mut out);
    Ok(out.buf)
}
