mod stream;
mod value;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::str::Utf8Error;

pub use stream::{InputStream, OutputStream};
use thiserror::Error;
pub use value::{Field, UnknownFields, Value};

pub const MAP_WIRE: u8 = 0b111;
pub const VARINT: u8 = 0;
pub const FIX64: u8 = 1;
pub const BYTES: u8 = 2;
pub const SGRP: u8 = 3;
pub const EGRP: u8 = 4;
pub const FIX32: u8 = 5;

#[repr(u8)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Length of submessage exceeds the length of message")]
    InvalidLimit,
    #[error("Unterminated group")]
    UnterminatedGroup,
    #[error("Unknown tag: {0}")]
    Tag(u32),
    #[error("Unknown wire type: {0}")]
    Wire(u8),
    #[error("String is not UTF8: {0}")]
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

pub trait BinProto<'buf>: Debug {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()>;
    fn size(&self) -> usize;
    fn encode(&self, stream: &mut OutputStream);
}

impl<'buf, T> BinProto<'buf> for Box<T>
    where
        T: BinProto<'buf>,
{
    #[inline(always)]
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()> {
        self.deref_mut().merge_field(tag_wire, stream)
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.deref().size()
    }

    #[inline(always)]
    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

pub trait Varint: Default + Debug + Clone + Copy + PartialEq + PartialOrd {
    fn from_u64(v: u64) -> Self;

    fn into_u64(self) -> u64;
}

impl Varint for u32 {
    #[inline(always)]
    fn from_u64(v: u64) -> Self {
        v as _
    }
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as _
    }
}

impl Varint for u64 {
    #[inline(always)]
    fn from_u64(v: u64) -> Self {
        v as _
    }
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as _
    }
}

impl Varint for i32 {
    #[inline(always)]
    fn from_u64(v: u64) -> Self {
        v as _
    }
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as _
    }
}

impl Varint for i64 {
    #[inline(always)]
    fn from_u64(v: u64) -> Self {
        v as _
    }
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as _
    }
}

impl Varint for usize {
    #[inline(always)]
    fn from_u64(v: u64) -> Self {
        v as _
    }
    #[inline(always)]
    fn into_u64(self) -> u64 {
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

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for i64 {
    type WIRE = Self;

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for u32 {
    type WIRE = u32;

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for u64 {
    type WIRE = Self;

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_le()
    }
}

impl Fixed for f32 {
    type WIRE = u32;

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_bits(Self::WIRE::from_le(v))
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_bits().to_le()
    }
}

impl Fixed for f64 {
    type WIRE = u64;

    #[inline(always)]
    fn from_wire(v: Self::WIRE) -> Self {
        Self::from_bits(Self::WIRE::from_le(v))
    }

    #[inline(always)]
    fn to_wire(self) -> Self::WIRE {
        self.to_bits().to_le()
    }
}

pub trait Bytes<'a> {
    fn blen(&self) -> usize;
    fn bytes(&self) -> &[u8];
    fn clear(&mut self);
    fn push(&mut self, b: &'a [u8]) -> Result<()>;
}

impl<'a> Bytes<'a> for &'a str {
    #[inline(always)]
    fn blen(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[inline(always)]
    fn clear(&mut self) {
        *self = "";
    }

    #[inline(always)]
    fn push(&mut self, b: &'a [u8]) -> Result<()> {
        *self = std::str::from_utf8(b)?;
        Ok(())
    }
}

impl<'any> Bytes<'any> for String {
    #[inline(always)]
    fn blen(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn push(&mut self, b: &[u8]) -> Result<()> {
        self.push_str(std::str::from_utf8(b)?);
        Ok(())
    }
}

impl<'a> Bytes<'a> for &'a [u8] {
    #[inline(always)]
    fn blen(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn bytes(&self) -> &[u8] {
        self
    }

    #[inline(always)]
    fn clear(&mut self) {
        *self = &[];
    }

    #[inline(always)]
    fn push(&mut self, b: &'a [u8]) -> Result<()> {
        *self = b;
        Ok(())
    }
}

impl<'any> Bytes<'any> for Vec<u8> {
    #[inline(always)]
    fn blen(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn push(&mut self, b: &[u8]) -> Result<()> {
        self.extend_from_slice(b);
        Ok(())
    }
}

#[inline(never)]
pub fn merge_single<'buf, T>(
    this: &mut T,
    stream: &mut InputStream<'buf>,
    mapper: fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
) -> Result<()> {
    mapper(stream, this)
}

#[inline(never)]
pub fn merge_optional<'buf, T: Default>(
    this: &mut Option<T>,
    stream: &mut InputStream<'buf>,
    mapper: fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
) -> Result<()> {
    mapper(stream, this.get_or_insert_with(Default::default))
}

#[inline(never)]
pub fn merge_repeated<'buf, T: Default>(
    this: &mut Vec<T>,
    stream: &mut InputStream<'buf>,
    mapper: fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
) -> Result<()> {
    this.push(T::default());
    mapper(stream, this.last_mut().unwrap())
}

#[inline(never)]
pub fn merge_oneof<'buf, T: Default + BinProto<'buf>>(
    this: &mut Option<T>,
    tag: u32,
    stream: &mut InputStream<'buf>,
) -> Result<()> {
    this.get_or_insert_with(Default::default).merge_field(tag, stream)
}

#[inline(never)]
pub fn merge_packed<'buf, T: Default>(
    this: &mut Vec<T>,
    stream: &mut InputStream<'buf>,
    mapper: fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
) -> Result<()> {
    this.clear();
    let len = stream._varint::<usize>()?;
    if stream.len() < len {
        return Err(Error::UnexpectedEOF);
    }
    let mut is = InputStream {
        buf: &stream.buf[stream.pos..stream.pos + len],
        pos: 0,
        limit: len,
    };
    while is.len() > 0 {
        this.push(T::default());
        // Safe, we've just pushed an elem into this
        unsafe {
            mapper(&mut is, this.last_mut().unwrap_unchecked())?;
        }
    }
    stream.pos += len;
    Ok(())
}

#[inline(never)]
pub fn merge_map<'buf, K: Default + Ord, V: Default>(
    this: &mut BTreeMap<K, V>,
    stream: &mut InputStream<'buf>,
    kmapper: fn(&mut InputStream<'buf>, &mut K) -> Result<()>,
    vmapper: fn(&mut InputStream<'buf>, &mut V) -> Result<()>,
) -> Result<()> {
    let len = stream._varint::<usize>()?;
    let olimit = stream.limit(len)?;

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
    stream.unlimit(olimit);
    this.insert(key, val);
    Ok(())
}

#[inline(never)]
pub fn emit_raw<T>(this: &T, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    stream._varint(tag);
    mapper(stream, tag, this);
}

#[inline(never)]
pub fn emit_single<T: Default + PartialEq>(
    this: &T,
    tag: u32,
    stream: &mut OutputStream,
    mapper: fn(&mut OutputStream, u32, &T),
) {
    if this != &T::default() {
        emit_raw(this, tag, stream, mapper)
    }
}

#[inline(never)]
pub fn emit_optional<T>(this: &Option<T>, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    if let Some(v) = this {
        stream._varint(tag);
        mapper(stream, tag, v);
    }
}

#[inline(never)]
pub fn emit_repeated<T>(this: &Vec<T>, tag: u32, stream: &mut OutputStream, mapper: fn(&mut OutputStream, u32, &T)) {
    for v in this {
        stream._varint(tag);
        mapper(stream, tag, v)
    }
}

#[inline(never)]
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

#[inline(never)]
pub fn emit_map<K: Default + PartialEq, V: Default + PartialEq>(
    this: &BTreeMap<K, V>,
    tag: u32,
    ktag: u32,
    vtag: u32,
    stream: &mut OutputStream,
    kmapper: fn(&mut OutputStream, u32, &K),
    vmapper: fn(&mut OutputStream, u32, &V),
) {
    for (k, v) in this {
        stream._varint(tag);
        let mut o = OutputStream::default();
        emit_raw(k, ktag, &mut o, kmapper);
        emit_raw(v, vtag, &mut o, vmapper);
        stream._varint(o.len());
        stream._bytes(o.buf.as_slice());
    }
}

#[inline(never)]
pub fn emit_oneof<'buf, T: BinProto<'buf>>(o: &Option<T>, stream: &mut OutputStream) {
    if let Some(o) = o {
        o.encode(stream)
    }
}

pub fn size_bytes<'x, T: Bytes<'x>>(v: &T) -> usize {
    size_varint(&v.blen()) + v.blen()
}

pub fn size_string<'x, T: Bytes<'x>>(v: &T) -> usize {
    size_varint(&v.blen()) + v.blen()
}

#[inline(always)]
pub fn size_protoenum<T: Copy + Into<u32>>(v: &T) -> usize {
    size_varint(&Into::<u32>::into(*v))
}

#[inline(always)]
pub fn size_bool(_: &bool) -> usize {
    1
}

#[inline(always)]
pub fn size_fixed32<T>(_: &T) -> usize {
    4
}

#[inline(always)]
pub fn size_fixed64<T>(_: &T) -> usize {
    8
}

pub fn size_varint<T: Varint + Debug>(value: &T) -> usize {
    let mut v = value.into_u64();
    if v == 0 {
        return 1;
    }

    let mut out = 0;
    while v > 0 {
        v >>= 7;
        out += 1;
    }
    out
}

pub fn size_sigint<T: Sigint>(v: &T) -> usize {
    size_varint(&v.encode().into_u64())
}

pub fn size_nested<'a, T: BinProto<'a>>(v: &T) -> usize {
    let s = v.size();
    size_varint(&s) + s
}

pub fn size_group<'a, T: BinProto<'a>>(v: &T) -> usize {
    // v.size()
    panic!()
}

#[inline(never)]
pub fn size_raw<T>(v: &T, tag: u32, sizer: fn(&T) -> usize) -> usize {
    size_varint(&tag) + sizer(v)
}

#[inline(never)]
pub fn size_singular<T: PartialEq + Default>(v: &T, tag: u32, sizer: fn(&T) -> usize) -> usize {
    if v != &Default::default() {
        size_raw(v, tag, sizer)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_optional<T>(v: &Option<T>, tag: u32, sizer: fn(&T) -> usize) -> usize {
    if let Some(v) = v {
        size_varint(&tag) + sizer(v)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_repeated<T>(v: &Vec<T>, tag: u32, sizer: fn(&T) -> usize) -> usize {
    v.iter().map(|v| size_varint(&tag) + sizer(v)).sum::<usize>()
}

#[inline(never)]
pub fn size_packed<T>(v: &Vec<T>, tag: u32, sizer: fn(&T) -> usize) -> usize {
    if v.len() > 0 {
        let len: usize = v.iter().map(|v| sizer(v)).sum();
        size_varint(&tag) + size_varint(&len) + len
    } else {
        0
    }
}

#[inline(never)]
pub fn size_oneof<'a, T: BinProto<'a>>(o: &Option<T>) -> usize {
    if let Some(o) = o {
        o.size()
    } else {
        0
    }
}

#[inline(never)]
pub fn size_map<K, V>(m: &BTreeMap<K, V>, tag: u32, ktag: u32, vtag: u32, ksize: fn(&K) -> usize, vsize: fn(&V) -> usize) -> usize {
    m.iter().map(|(k, v)| {
        let elem = ksize(k) + size_varint(&ktag) + vsize(v) + size_varint(&vtag);
        size_varint(&tag) + size_varint(&elem)
    }).sum()
}

pub fn decode<'buf, T: Default + BinProto<'buf>>(b: &'buf [u8]) -> Result<T> {
    let mut out = T::default();
    let mut is = InputStream::new(b);
    while is.len() > 0 {
        let tag = is._varint()?;
        out.merge_field(tag, &mut is)?;
    }
    Ok(out)
}

pub fn encode<'buf, T: BinProto<'buf>>(b: &T) -> Result<Vec<u8>> {
    let mut out = OutputStream::default();
    b.encode(&mut out);
    Ok(out.buf)
}
