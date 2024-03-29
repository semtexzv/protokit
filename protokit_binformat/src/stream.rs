use core::cmp::min;
use core::fmt::Debug;
use core::mem::{replace, size_of};
use core::ptr::read_unaligned;
use core::slice::from_raw_parts;

use crate::{BinProto, BytesLike, Error, Fixed, Result, Sigint, SizeStack, Varint, _size_varint, EGRP};

const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

#[derive(Debug)]
pub struct InputStream<'buf> {
    pub(crate) buf: &'buf [u8],
    pub(crate) pos: usize,
    pub(crate) limit: usize,
}

impl<'buf> InputStream<'buf> {
    #[inline(always)]
    pub fn new(buf: &'buf [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            limit: buf.len(),
        }
    }

    /// Returns the length of currently readable subslice
    #[inline(always)]
    pub fn len(&self) -> usize {
        min(self.buf.len(), self.limit) - self.pos
    }

    /// Whether the currently readable subslice is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Limits the currently readable subslice, and returns previous limit
    pub fn limit(&mut self, limit: usize) -> Result<usize> {
        if self.limit < limit {
            return Err(Error::InvalidBytesLimit);
        }
        Ok(replace(&mut self.limit, min(self.pos + limit, self.buf.len())))
    }

    /// Reverts previous slice limit
    #[inline(always)]
    pub fn unlimit(&mut self, limit: usize) {
        debug_assert!(self.limit <= limit);
        self.limit = min(self.buf.len(), limit);
    }

    /// Skips next field with given tag
    pub fn skip(&mut self, tag: u32) -> Result<()> {
        if tag >> 3 == 0 {
            return crate::unknown_tag(0);
        }
        match (tag & 0b111) as u8 {
            crate::VARINT => {
                self._varint::<u64>()?;
            }
            crate::BYTES => {
                self._bytes()?;
            }
            crate::FIX32 => {
                self._fixed::<u32>()?;
            }
            crate::FIX64 => {
                self._fixed::<u64>()?;
            }
            crate::SGRP => {
                self.skip_until_tag((tag & !0b111) | EGRP as u32)?;
            }
            other => return crate::unknown_wire(other),
        }

        Ok(())
    }

    /// Skip next set of values, until we find the provided tag.
    /// Used for traversing through groups
    pub fn skip_until_tag(&mut self, next_tag: u32) -> Result<()> {
        let tag = self._varint()?;
        if tag == next_tag {
            return Ok(());
        }
        self.skip(tag)
    }

    /// Decode a single varint from the stream.
    /// Currently we read it as u64, and then truncate.
    pub fn _varint<T: Varint>(&mut self) -> Result<T> {
        let mut result: u64 = 0;
        let mut shift = 0;

        let mut success = false;
        // Safety: We must maintain correct limit in order for this to work
        for b in unsafe { self.buf.get_unchecked(self.pos .. self.limit).iter() } {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 || shift > (9 * 7) {
                success = b & MSB == 0;
                break;
            }
        }

        if success {
            self.pos += (shift / 7) as usize;
            Ok(T::from_u64(result))
        } else {
            Err(Error::UnexpectedEOF)
        }
    }

    /// Skip a single zigzag encoded varint from the stream
    pub fn _sigint<T: Sigint>(&mut self) -> Result<T> {
        Ok(T::decode(self._varint::<T::Varint>()?))
    }

    /// Read fixed values from the stream
    pub fn _fixed<T: Fixed>(&mut self) -> Result<T> {
        let tlen = size_of::<T>();
        if self.len() < tlen {
            return Err(Error::UnexpectedEOF);
        }
        let out = unsafe { read_unaligned(self.buf.as_ptr().add(self.pos) as *const T) };
        self.pos += tlen;
        Ok(out)
    }

    /// Read a length-prefixed slice from the stream
    pub fn _bytes(&mut self) -> Result<&'buf [u8]> {
        let len: usize = self._varint()?;
        if self.len() < len {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += len;
        Ok(&self.buf[self.pos - len .. self.pos])
    }

    /// Read length-prefixed slice from the stream and decode it a string
    pub fn _string(&mut self) -> Result<&'buf str> {
        Ok(core::str::from_utf8(self._bytes()?)?)
    }

    pub fn varint<T: Varint>(&mut self, field: &mut T) -> Result<()> {
        *field = self._varint()?;
        Ok(())
    }

    #[inline(always)]
    pub fn sigint<T: Sigint + Default>(&mut self, field: &mut T) -> Result<()> {
        *field = self._sigint()?;
        Ok(())
    }

    #[inline(always)]
    pub fn protoenum<T: From<u32>>(&mut self, field: &mut T) -> Result<()> {
        *field = self._varint::<u32>()?.into();
        Ok(())
    }

    #[inline(always)]
    pub fn bool(&mut self, field: &mut bool) -> Result<()> {
        *field = self._varint::<u64>()? > 0;
        Ok(())
    }

    #[inline(always)]
    pub fn fixed32<T: Default + Fixed>(&mut self, field: &mut T) -> Result<()> {
        debug_assert_eq!(size_of::<T>(), 4);
        *field = self._fixed()?;
        Ok(())
    }

    #[inline(always)]
    pub fn fixed64<T: Fixed>(&mut self, field: &mut T) -> Result<()> {
        debug_assert_eq!(size_of::<T>(), 8);
        *field = self._fixed()?;
        Ok(())
    }

    pub fn bytes<'x, T: BytesLike<'buf>>(&mut self, field: &mut T) -> Result<()> {
        field.set_slice(self._bytes()?)?;
        Ok(())
    }

    pub fn string<T: BytesLike<'buf>>(&mut self, field: &mut T) -> Result<()> {
        field.set_slice(self._bytes()?)?;
        Ok(())
    }

    pub fn nested<P: BinProto<'buf>>(&mut self, p: &mut P) -> Result<()> {
        self._field_nested(p)
    }

    pub fn group<P: BinProto<'buf>>(&mut self, p: &mut P) -> Result<()> {
        self._field_group(0, p)
    }

    pub fn _field_nested(&mut self, proto: &mut dyn BinProto<'buf>) -> Result<()> {
        let len = self._varint()?;
        if len > self.len() {
            return Err(Error::UnexpectedEOF);
        }
        let start = self.pos;
        let olimit = self.limit(len)?;
        while !self.is_empty() {
            let tag = self._varint()?;
            proto.merge_field(tag, self)?;
        }
        assert_eq!(self.pos, start + len);
        // Bump consumed
        self.unlimit(olimit);
        Ok(())
    }

    pub fn _field_group(&mut self, _gtag: u32, proto: &mut dyn BinProto<'buf>) -> Result<()> {
        while !self.is_empty() {
            let tag = self._varint()?;
            // If this is the end group tag, we're done with current item.
            if tag & 7 == EGRP as _ {
                break;
            }
            proto.merge_field(tag, self)?
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct OutputStream<'o> {
    pub(crate) stack: SizeStack,
    pub(crate) buf: &'o mut [u8],
    pub(crate) pos: usize
}

impl<'o> OutputStream<'o> {
    pub fn new(stack: SizeStack, buf: &'o mut [u8]) -> Self {
        Self {
            stack,
            buf,
            pos: 0
        }
    }
    // pub fn len(&self) -> usize {
    //     self.buf.len() - self.pos
    // }

    pub fn is_empty(&self) -> bool {
        self.buf.len() == self.pos - 1
    }

    pub(crate) fn _tag(&mut self, t: u32) {
        self._varint(t)
    }

    /// Emits a raw vint onto the wire
    pub(crate) fn _varint<V: Varint + Debug>(&mut self, v: V) {
        let mut n = v.into_u64();

        let len = _size_varint(n);
        // assert!(self.len() >= len);

        for _ in 0 .. len - 1 {
            self.buf[self.pos] = MSB | (n as u8 & DROP_MSB);
            self.pos += 1;
            n >>= 7;
        }
        self.buf[self.pos] = n as u8;
        self.pos += 1;
    }

    pub(crate) fn raw_bytes(&mut self, v: &[u8]) {
        assert!(self.buf.len() - self.pos >= v.len());
        self.buf[self.pos.. self.pos + v.len()].copy_from_slice(v);
        self.pos += v.len();
    }

    pub fn varint<V: Varint + Debug>(&mut self, _: u32, v: &V) {
        self._varint(*v)
    }

    pub fn sigint<V: Sigint + Debug>(&mut self, _: u32, v: &V) {
        self._varint(v.encode())
    }

    pub fn bool(&mut self, _: u32, b: &bool) {
        self._varint(if *b { 1 } else { 0 });
    }

    pub fn protoenum<V: Clone + Copy + Into<u32>>(&mut self, _: u32, v: &V) {
        self._varint((*v).into());
    }

    pub fn fixed<V: Fixed>(&mut self, _: u32, v: &V) {
        let wire = v.to_wire();
        let slice = unsafe { from_raw_parts(&wire as *const V::Wire as *const u8, size_of::<V::Wire>()) };
        self.raw_bytes(slice);
    }

    pub fn fixed32<V: Fixed>(&mut self, _: u32, v: &V) {
        self.fixed(0, v)
    }

    pub fn fixed64<V: Fixed>(&mut self, _: u32, v: &V) {
        self.fixed(0, v)
    }

    pub fn bytes<'x, B: BytesLike<'x>>(&mut self, _: u32, b: &B) {
        self._varint(b.buf().len());
        self.raw_bytes(b.buf());
    }

    pub fn string<'out, B: BytesLike<'out>>(&mut self, _: u32, b: &B) {
        self._varint(b.buf().len());
        self.raw_bytes(b.buf());
    }

    #[inline(never)]
    fn _nested(&mut self, len: usize, v: &dyn BinProto<'_>) {
        self._varint(len);
        v.encode(self)
    }

    pub fn nested<'buf, P: BinProto<'buf>>(&mut self, _: u32, v: &P) {
        let (ptr, len) = self.stack.pop();
        debug_assert_eq!(v as *const P as *const u8, ptr);

        self._nested(len, v);
    }

    pub fn group<'buf, P: BinProto<'buf>>(&mut self, num: u32, v: &P) {
        // SGRP tag is encoded outside of this method
        v.encode(self);
        self._varint((num & !0b111) | EGRP as u32);
    }
}
