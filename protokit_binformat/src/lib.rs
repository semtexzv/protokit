#![allow(clippy::ptr_arg)]

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use core::str::Utf8Error;
use std::borrow::Cow;
use std::collections::BTreeMap;

use indexmap::IndexMap;
pub use stream::{InputStream, OutputStream};
use thiserror::Error;
pub use value::{Field, UnknownFieldsBorrow, UnknownFieldsOwned, Value};

pub mod stream;
pub mod value;
pub mod fields;

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

pub trait BinProto<'buf>: Debug {
    fn merge_field(&mut self, tag_wire: u32, stream: &mut InputStream<'buf>) -> Result<()>;
    fn size(&self, stack: &mut SizeStack) -> usize;
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
    fn size(&self, stack: &mut SizeStack) -> usize {
        self.deref().size(stack)
    }

    #[inline(always)]
    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

pub trait DefaultIn<C> {
    fn default_in(ctx: &C) -> Self;
}

pub trait Varint: Default + Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord {
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

impl Fixed for i64 {
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
    fn blen(&self) -> usize;
    fn bytes(&self) -> &[u8];
    fn clear(&mut self);
    fn merge(&mut self, b: &'a [u8]) -> Result<()>;
}

impl<'buf> BytesLike<'buf> for &'buf str {
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
    fn merge(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = core::str::from_utf8(b)?;
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for String {
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
    fn merge(&mut self, b: &[u8]) -> Result<()> {
        self.push_str(core::str::from_utf8(b)?);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for Cow<'buf, str> {
    fn blen(&self) -> usize {
        self.len()
    }

    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn clear(&mut self) {
        *self = Cow::Borrowed("");
    }

    fn merge(&mut self, b: &'buf [u8]) -> Result<()> {
        let b = std::str::from_utf8(b)?;
        *self = Cow::Borrowed(b);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for &'buf [u8] {
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
    fn merge(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = b;
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for Vec<u8> {
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
    fn merge(&mut self, b: &[u8]) -> Result<()> {
        self.extend_from_slice(b);
        Ok(())
    }
}

impl<'buf> BytesLike<'buf> for Cow<'buf, [u8]> {
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
        *self = Cow::Borrowed(&[]);
    }

    #[inline(always)]
    fn merge(&mut self, b: &'buf [u8]) -> Result<()> {
        *self = Cow::Borrowed(b);
        Ok(())
    }
}

impl<'a, const N: usize> BytesLike<'a> for [u8; N]
where
    [u8; N]: Default,
{
    fn blen(&self) -> usize {
        N
    }

    fn bytes(&self) -> &[u8] {
        self
    }

    fn clear(&mut self) {}

    fn merge(&mut self, b: &'a [u8]) -> Result<()> {
        *self = b.try_into().map_err(|_| Error::InvalidBytesLimit)?;
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
        buf: &stream.buf[stream.pos .. stream.pos + len],
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
    if !this.is_empty() {
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
pub fn emit_map<K, V, M: Map<K, V>>(
    this: &M,
    tag: u32,
    ktag: u32,
    vtag: u32,
    stream: &mut OutputStream,
    kmapper: fn(&mut OutputStream, u32, &K),
    vmapper: fn(&mut OutputStream, u32, &V),
) {
    this.for_each(|(k, v)| {
        stream._varint(tag);
        assert_eq!(k as *const K as *const u8, stream.stack.top().0);
        let len = stream.stack.pop().1;
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

pub fn size_bytes<'x, T: BytesLike<'x>>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.blen()) + v.blen()
}

pub fn size_string<'x, T: BytesLike<'x>>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.blen()) + v.blen()
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

const VINT_LENS: [u8; 65] = [
    10, 9, 9, 9, 9, 9, 9, 9, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 4, 4,
    4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1,
];

pub fn _size_varint<T: Varint>(value: T) -> usize {
    VINT_LENS[value.into_u64().leading_zeros() as usize] as _
}

pub fn size_varint<T: Varint>(value: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(*value)
}

pub fn size_sigint<T: Sigint>(v: &T, _: u32, _: &mut SizeStack) -> usize {
    _size_varint(v.encode().into_u64())
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
    fn top(&self) -> (*const u8, usize) {
        *self.stack.last().unwrap()
    }
    fn pop(&mut self) -> (*const u8, usize) {
        self.stack.pop().unwrap()
    }
}

#[inline(always)]
pub fn size_raw<T, F>(v: &T, tag: u32, stack: &mut SizeStack, sizer: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    _size_varint(tag) + sizer(v, tag, stack)
}

#[inline(never)]
pub fn size_singular<T, F>(v: &T, tag: u32, stack: &mut SizeStack, sizer: F) -> usize
where
    T: PartialEq + Default,
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if v != &Default::default() {
        _size_varint(tag) + sizer(v, tag, stack)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_optional<T, F>(v: &Option<T>, tag: u32, stack: &mut SizeStack, sizer: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if let Some(v) = v {
        _size_varint(tag) + sizer(v, tag, stack)
    } else {
        0
    }
}

#[inline(never)]
pub fn size_repeated<T, F>(v: &Vec<T>, tag: u32, stack: &mut SizeStack, sizer: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    v.iter().rev().map(|v| _size_varint(tag) + sizer(v, tag, stack)).sum()
}

#[inline(never)]
pub fn size_packed<T, F>(v: &Vec<T>, tag: u32, stack: &mut SizeStack, sizer: F) -> usize
where
    F: Fn(&T, u32, &mut SizeStack) -> usize,
{
    if !v.is_empty() {
        let len: usize = v.iter().rev().map(|v| sizer(v, tag, stack)).sum();
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
    ksize: fn(&K, u32, &mut SizeStack) -> usize,
    vsize: fn(&V, u32, &mut SizeStack) -> usize,
) -> usize {
    let mut sum = 0;
    m.rev_for_each(|(k, v)| {
        let elem = stack.memo(k, |_, stack| {
            let v = vsize(v, vtag, stack);
            let k = ksize(k, ktag, stack);
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
    let mut out = OutputStream::default();
    b.size(&mut out.stack);
    b.encode(&mut out);
    Ok(out.buf)
}
