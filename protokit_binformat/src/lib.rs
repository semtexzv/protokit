#![allow(clippy::ptr_arg)]
#![deny(unconditional_recursion)]

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use core::str::Utf8Error;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::string::FromUtf8Error;

use bytes::Bytes;
use indexmap::IndexMap;
pub use stream::{InputStream, OutputStream};
use thiserror::Error;
pub use value::{Field, UnknownFieldsBorrow, UnknownFieldsOwned, Value};

pub mod stream;
pub mod value;

pub const MASK_WIRE: u8 = 0b111;

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
    InvalidBytesLimit,

    #[error("String is not UTF8: {0}")]
    InvalidUtf8(#[from] Utf8Error),

    #[error("String is not UTF8: {0}")]
    InvalidFromUtf8(#[from] FromUtf8Error),

    #[error("Unterminated group")]
    UnterminatedGroup,

    #[error("Unknown tag: {0}")]
    UnknownTag(u32),

    #[error("Unknown wire type: {0}")]
    UnknownWire(u8),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cold]
pub fn unknown_tag<T>(tag: u32) -> Result<T> {
    Err(Error::UnknownTag(tag))
}

#[cold]
pub fn unknown_wire<T>(w: u8) -> Result<T> {
    Err(Error::UnknownWire(w))
}

pub trait BinProto<'buf> {
    fn qualified_name(&self) -> &'static str;
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()>;
    fn size(&self, stack: &mut SizeStack) -> usize;
    fn encode<'out>(&self, stream: &mut OutputStream<'out>);
}

impl<'buf, T> BinProto<'buf> for Box<T>
where
    T: BinProto<'buf>,
{
    #[inline(always)]
    fn qualified_name(&self) -> &'static str {
        self.deref().qualified_name()
    }

    #[inline(always)]
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()> {
        self.deref_mut().merge_field(tag_wire, stream)
    }

    #[inline(always)]
    fn size(&self, stack: &mut SizeStack) -> usize {
        self.deref().size(stack)
    }

    #[inline(always)]
    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

#[cfg(feature = "arena")]
impl<'buf, 'arena, T> BinProto<'buf> for bumpalo::boxed::Box<'arena, T>
where
    T: BinProto<'buf>,
{
    fn qualified_name(&self) -> &'static str {
        self.deref().qualified_name()
    }

    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()> {
        self.deref_mut().merge_field(tag_wire, stream)
    }

    fn size(&self, stack: &mut SizeStack) -> usize {
        self.deref().size(stack)
    }

    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

pub trait Varint: Default + Debug + Clone + Copy {
    fn from_u64(v: u64) -> Self;
    fn into_u64(self) -> u64;
}

macro_rules! impl_varint {
    ($($ty:ty),*) => {$(
        impl Varint for $ty {
            #[inline(always)]
            fn from_u64(v: u64) -> Self {
                v as _
            }
            #[inline(always)]
            fn into_u64(self) -> u64 {
                self as _
            }
        })*
    };
}

impl_varint!(i32, u32, i64, u64, usize);

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
    type Wire: Sized;
    fn from_wire(v: Self::Wire) -> Self;
    fn to_wire(self) -> Self::Wire;
}

impl Fixed for i32 {
    type Wire = u32;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_le(v as _)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_le() as _
    }
}

impl Fixed for i64 {
    type Wire = u64;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_le(v as _)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_le() as _
    }
}

impl Fixed for u32 {
    type Wire = u32;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_le()
    }
}

impl Fixed for u64 {
    type Wire = Self;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_le(v)
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_le()
    }
}

impl Fixed for f32 {
    type Wire = u32;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_bits(Self::Wire::from_le(v))
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_bits().to_le()
    }
}

impl Fixed for f64 {
    type Wire = u64;

    #[inline(always)]
    fn from_wire(v: Self::Wire) -> Self {
        Self::from_bits(Self::Wire::from_le(v))
    }

    #[inline(always)]
    fn to_wire(self) -> Self::Wire {
        self.to_bits().to_le()
    }
}

pub trait BytesLike<'a>: Debug + Default {
    /// Length of this field
    fn len(&self) -> usize;
    /// Reference to underlying byte byffer
    fn buf(&self) -> &[u8];
    /// Set this byte buffer to new value
    fn set_slice(&mut self, b: &'a [u8]) -> Result<()>;
    fn set_bytes(&mut self, b: Bytes) -> Result<()>;
}

impl<'buf> BytesLike<'buf> for &'buf str {
    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        self.as_bytes()
    }

    #[inline(always)]
    fn set_slice(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = core::str::from_utf8(b)?;
        Ok(())
    }

    fn set_bytes(&mut self, _: Bytes) -> Result<()> {
        panic!("Can't set from bytes")
    }
}

impl<'buf> BytesLike<'buf> for String {
    #[inline(always)]
    fn len(&self) -> usize {
        String::len(self)
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        self.as_bytes()
    }

    #[inline(always)]
    fn set_slice(&mut self, b: &[u8]) -> Result<()> {
        self.clear();
        self.push_str(core::str::from_utf8(b)?);
        Ok(())
    }

    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        self.clear();
        self.push_str(core::str::from_utf8(&b)?);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for Cow<'buf, str> {
    fn len(&self) -> usize {
        str::len(self)
    }

    fn buf(&self) -> &[u8] {
        self.as_bytes()
    }

    fn set_slice(&mut self, b: &'buf [u8]) -> Result<()> {
        let b = std::str::from_utf8(b)?;
        *self = Cow::Borrowed(b);
        Ok(())
    }

    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        *self = Cow::Owned(String::from_utf8(b.to_vec())?);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for &'buf [u8] {
    #[inline(always)]
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        self
    }

    #[inline(always)]
    fn set_slice(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = b;
        Ok(())
    }

    fn set_bytes(&mut self, _: Bytes) -> Result<()> {
        panic!("Can't borrow from bytes")
    }
}

impl<'buf> BytesLike<'buf> for Vec<u8> {
    #[inline(always)]
    fn len(&self) -> usize {
        Vec::len(self)
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        self.as_slice()
    }

    #[inline(always)]
    fn set_slice(&mut self, b: &[u8]) -> Result<()> {
        self.clear();
        self.extend_from_slice(b);
        Ok(())
    }

    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        self.clear();
        self.extend_from_slice(&b);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for Cow<'buf, [u8]> {
    #[inline(always)]
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        self
    }

    #[inline(always)]
    fn set_slice(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = Cow::Borrowed(b);
        Ok(())
    }

    #[inline(always)]
    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        *self = Cow::Owned(b.to_vec());
        Ok(())
    }
}

impl<'a, const N: usize> BytesLike<'a> for [u8; N]
where
    [u8; N]: Default,
{
    fn len(&self) -> usize {
        N
    }

    fn buf(&self) -> &[u8] {
        self
    }

    fn set_slice(&mut self, b: &'a [u8]) -> Result<()> {
        *self = b.try_into().map_err(|_| Error::InvalidBytesLimit)?;
        Ok(())
    }

    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        self.set_slice(&b)
    }
}

impl<'a> BytesLike<'a> for Bytes {
    fn len(&self) -> usize {
        Bytes::len(self)
    }

    fn buf(&self) -> &[u8] {
        Bytes::deref(self)
    }

    fn set_slice(&mut self, b: &'a [u8]) -> Result<()> {
        *self = Bytes::from(b.to_vec());
        Ok(())
    }

    fn set_bytes(&mut self, b: Bytes) -> Result<()> {
        *self = b;
        Ok(())
    }
}

pub trait Map<K, V> {
    fn mlen(&self) -> usize;
    fn insert(&mut self, k: K, v: V);
    fn for_each<F: FnMut((&K, &V))>(&self, fun: F);
    fn rev_for_each<F: FnMut((&K, &V))>(&self, fun: F);
}

impl<K: PartialOrd + Ord + PartialEq, V> Map<K, V> for BTreeMap<K, V> {
    fn mlen(&self) -> usize {
        self.len()
    }

    fn insert(&mut self, k: K, v: V) {
        let _ = self.insert(k, v);
    }

    fn for_each<F: FnMut((&K, &V))>(&self, fun: F) {
        self.iter().for_each(fun)
    }

    fn rev_for_each<F: FnMut((&K, &V))>(&self, fun: F) {
        self.iter().rev().for_each(fun)
    }
}

impl<K: Hash + PartialEq + Eq, V> Map<K, V> for IndexMap<K, V> {
    fn mlen(&self) -> usize {
        self.len()
    }

    fn insert(&mut self, k: K, v: V) {
        let _ = IndexMap::insert(self, k, v);
    }

    fn for_each<F: FnMut((&K, &V))>(&self, fun: F) {
        self.iter().for_each(fun)
    }

    fn rev_for_each<F: FnMut((&K, &V))>(&self, fun: F) {
        self.iter().rev().for_each(fun)
    }
}

#[inline(never)]
pub fn merge_single<'buf, T, F>(this: &mut T, stream: &mut InputStream<'buf>, mapper: F) -> Result<()>
where
    F: Fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
{
    mapper(stream, this)
}

#[inline(never)]
pub fn merge_optional<'buf, T, F>(this: &mut Option<T>, stream: &mut InputStream<'buf>, mapper: F) -> Result<()>
where
    T: Default,
    F: Fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
{
    mapper(stream, this.get_or_insert_with(Default::default))
}

#[inline(never)]
pub fn merge_repeated<'buf, T, F>(this: &mut Vec<T>, stream: &mut InputStream<'buf>, mapper: F) -> Result<()>
where
    T: Default,
    F: Fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
{
    this.push(T::default());
    unsafe { mapper(stream, this.last_mut().unwrap_unchecked()) }
}

#[inline(never)]
pub fn merge_packed<'buf, T, F>(this: &mut Vec<T>, stream: &mut InputStream<'buf>, mapper: F) -> Result<()>
where
    T: Default,
    F: Fn(&mut InputStream<'buf>, &mut T) -> Result<()>,
{
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
    while !is.is_empty() {
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
pub fn merge_map<'buf, K, V, M: Map<K, V>>(
    this: &mut M,
    stream: &mut InputStream<'buf>,
    kmapper: fn(&mut InputStream<'buf>, &mut K) -> Result<()>,
    vmapper: fn(&mut InputStream<'buf>, &mut V) -> Result<()>,
) -> Result<()>
where
    K: Default,
    V: Default,
{
    let len = stream._varint::<usize>()?;
    let olimit = stream.limit(len)?;

    let mut key = K::default();
    let mut val = V::default();
    while !stream.is_empty() {
        // TODO: verify wire types
        match stream._varint::<u32>()? >> 3 {
            1 => merge_single(&mut key, stream, kmapper)?,
            2 => merge_single(&mut val, stream, vmapper)?,
            tag => unknown_tag(tag)?,
        }
    }
    this.insert(key, val);
    stream.unlimit(olimit);
    Ok(())
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
pub fn emit_raw<'out, T>(
    this: &T,
    tag: u32,
    stream: &mut OutputStream<'out>,
    mapper: fn(&mut OutputStream<'out>, u32, &T),
) {
    stream._tag(tag);
    mapper(stream, tag, this);
}

#[inline(never)]
pub fn emit_single<'out, T: Debug + Default + PartialEq>(
    this: &T,
    tag: u32,
    stream: &mut OutputStream<'out>,
    mapper: fn(&mut OutputStream<'out>, u32, &T),
) {
    if this != &T::default() {
        emit_raw(this, tag, stream, mapper)
    }
}

#[inline(never)]
pub fn emit_optional<'out, T>(
    this: &Option<T>,
    tag: u32,
    stream: &mut OutputStream<'out>,
    mapper: fn(&mut OutputStream<'out>, u32, &T),
) {
    if let Some(v) = this {
        stream._tag(tag);
        mapper(stream, tag, v);
    }
}

#[inline(never)]
pub fn emit_repeated<'out, T>(
    this: &Vec<T>,
    tag: u32,
    stream: &mut OutputStream<'out>,
    mapper: fn(&mut OutputStream<'out>, u32, &T),
) {
    for v in this {
        stream._tag(tag);
        mapper(stream, tag, v)
    }
}

#[inline(never)]
pub fn emit_packed<'out, T>(
    this: &Vec<T>,
    tag: u32,
    stream: &mut OutputStream<'out>,
    mapper: fn(&mut OutputStream<'out>, u32, &T),
) {
    if !this.is_empty() {
        let (ptr, len) = stream.stack.pop();
        debug_assert_eq!(this as *const Vec<T> as *const u8, ptr);

        stream._tag(tag);
        stream._varint(len);

        for v in this {
            mapper(stream, tag, v);
        }
    }
}

#[inline(never)]
pub fn emit_map<'out, K, V, M: Map<K, V>>(
    this: &M,
    tag: u32,
    ktag: u32,
    vtag: u32,
    stream: &mut OutputStream<'out>,
    kmapper: fn(&mut OutputStream<'out>, u32, &K),
    vmapper: fn(&mut OutputStream<'out>, u32, &V),
) {
    this.for_each(|(k, v)| {
        let (ptr, len) = stream.stack.pop();
        debug_assert_eq!(k as *const K as *const u8, ptr);

        stream._tag(tag);
        stream._varint(len);
        emit_raw(k, ktag, stream, kmapper);
        emit_raw(v, vtag, stream, vmapper);
    });
}

#[inline(never)]
pub fn emit_oneof<'buf, T: BinProto<'buf>>(o: &Option<T>, stream: &mut OutputStream) {
    if let Some(o) = o {
        o.encode(stream)
    }
}

pub fn _size_varint<T: Varint>(value: T) -> usize {
    const VINT_LENS: [u8; 65] = [
        10, 9, 9, 9, 9, 9, 9, 9, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 4,
        4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    VINT_LENS[value.into_u64().leading_zeros() as usize] as _
}

pub fn size_varint<T: Varint>(value: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(*value)
}

pub fn size_sigint<T: Sigint>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.encode().into_u64())
}

pub fn size_bytes<'x, T: BytesLike<'x>>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.len()) + v.len()
}

pub fn size_string<'x, T: BytesLike<'x>>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.len()) + v.len()
}

#[inline(always)]
pub fn size_protoenum<T: Copy + Into<u32>>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(Into::<u32>::into(*v))
}

#[inline(always)]
pub fn size_bool(_: &bool, _: u32, _: &mut SizeStack) -> usize {
    1
}

#[inline(always)]
pub fn size_fixed32<T>(_: &T, _: u32, _: &mut SizeStack) -> usize {
    4
}

#[inline(always)]
pub fn size_fixed64<T>(_: &T, _: u32, _: &mut SizeStack) -> usize {
    8
}

pub fn size_nested<'a, T: BinProto<'a>>(v: &T, _: u32, stack: &mut SizeStack) -> usize {
    let inner = stack.memo(v, |v, stack| v.size(stack));
    _size_varint(inner) + inner
}

pub fn size_group<'a, T: BinProto<'a>>(v: &T, tag: u32, stack: &mut SizeStack) -> usize {
    // We have endgrp tag to worry about
    v.size(stack) + _size_varint(tag)
}

#[derive(Debug, Default)]
pub struct SizeStack {
    stack: Vec<(*const u8, usize)>,
}

impl SizeStack {
    fn memo<T, F: FnOnce(&T, &mut Self) -> usize>(&mut self, p: &T, calc: F) -> usize {
        let out = calc(p, self);
        self.stack.push((p as *const T as *const u8, out));
        out
    }
    fn pop(&mut self) -> (*const u8, usize) {
        self.stack.pop().unwrap()
    }
}

#[inline(always)]
pub fn size_raw<T, F>(v: &T, tag: u32, stack: &mut SizeStack, measure: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    _size_varint(tag) + measure(v, tag, stack)
}

#[inline(never)]
pub fn size_singular<T, F>(v: &T, tag: u32, stack: &mut SizeStack, measure: F) -> usize
where
    T: PartialEq + Default,
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if v != &Default::default() {
        _size_varint(tag) + measure(v, tag, stack)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_optional<T, F>(v: &Option<T>, tag: u32, stack: &mut SizeStack, measure: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if let Some(v) = v {
        _size_varint(tag) + measure(v, tag, stack)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_repeated<T, F>(v: &Vec<T>, tag: u32, stack: &mut SizeStack, measure: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    v.iter().map(|v| _size_varint(tag) + measure(v, tag, stack)).sum()
}

#[inline(never)]
pub fn size_packed<T, F>(v: &Vec<T>, tag: u32, stack: &mut SizeStack, measure: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if !v.is_empty() {
        let len = stack.memo(v, |v, stack| v.iter().map(|v| measure(v, tag, stack)).sum());
        _size_varint(tag) + _size_varint(len) + len
    } else {
        0
    }
}

#[inline(never)]
pub fn size_oneof<'a, T: BinProto<'a>>(o: &Option<T>, stack: &mut SizeStack) -> usize {
    if let Some(o) = o {
        o.size(stack)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_map<K, V, M: Map<K, V>>(
    m: &M,
    tag: u32,
    ktag: u32,
    vtag: u32,
    stack: &mut SizeStack,
    measure_key: fn(&K, u32, &mut SizeStack) -> usize,
    measure_val: fn(&V, u32, &mut SizeStack) -> usize,
) -> usize {
    let mut sum = 0;
    // Use forward iteration to serialize in insertion order
    m.for_each(|(k, v)| {
        let elem = stack.memo(k, |_, stack| {
            let v = measure_val(v, vtag, stack);
            let k = measure_key(k, ktag, stack);
            _size_varint(ktag) + _size_varint(vtag) + k + v
        });
        sum += _size_varint(tag) + _size_varint(elem) + elem
    });
    sum
}

pub fn decode<'buf, T: Default + BinProto<'buf>>(b: &'buf [u8]) -> Result<T> {
    let mut out = T::default();
    let mut is = InputStream::new(b);
    while !is.is_empty() {
        let tag = is._varint()?;
        out.merge_field(tag, &mut is)?;
    }
    Ok(out)
}

pub fn encode<'buf, T: BinProto<'buf>>(b: &T) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut stack = SizeStack::default();
    let size = b.size(&mut stack);

    out.resize(size, 0);
    let mut ostream = OutputStream::new(stack, &mut out);
    b.encode(&mut ostream);
    assert_eq!(ostream.stack.stack.len(), 0);
    Ok(out)
}

pub fn encode_to<'buf, T: BinProto<'buf>>(b: &T, mut out: Vec<u8>) -> Result<Vec<u8>> {
    let mut stack = SizeStack::default();
    let size = b.size(&mut stack);
    out.resize(out.len() + size, 0);
    let start = out.len() - size;
    let mut ostream = OutputStream::new(stack, &mut out[start..]);
    b.encode(&mut ostream);
    assert_eq!(ostream.stack.stack.len(), 0);
    Ok(out)
}
