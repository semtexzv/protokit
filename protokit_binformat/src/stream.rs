use std::cmp::min;
use std::mem::{replace, size_of};
use std::ptr::read_unaligned;
use std::slice::from_raw_parts;

use crate::{BinProto, Bytes, Error, Fixed, Result, Sigint, Varint, EGRP};

pub const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

pub struct InputStream<'buf> {
    pub(crate) buf: &'buf [u8],
    pub(crate) pos: usize,
    pub(crate) limit: usize,
}

impl<'buf> InputStream<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            limit: buf.len(),
        }
    }

    /// Returns the length of currently readable subslice
    pub fn len(&self) -> usize {
        min(self.buf.len(), self.limit) - self.pos
    }

    /// Limits the currently readable subslice, and returns previous limit
    pub fn limit(&mut self, limit: usize) -> Result<usize> {
        if self.limit < limit {
            return Err(Error::InvalidLimit);
        }
        if self.limit < limit {
            panic!("Limiting back")
        }
        Ok(replace(&mut self.limit, min(self.pos + limit, self.buf.len())))
    }

    pub fn unlimit(&mut self, limit: usize) {
        assert!(self.limit <= limit);
        self.limit = limit;
    }

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
                self.skip_grp((tag & !0b111) | EGRP as u32)?;
            }
            other => return crate::unknown_wire(other),
        }

        Ok(())
    }

    pub fn skip_grp(&mut self, egrp_tag: u32) -> Result<()> {
        let tag = self._varint()?;
        if tag == egrp_tag {
            return Ok(());
        }
        self.skip(tag)
    }

    pub fn _varint<T: Varint>(&mut self) -> Result<T> {
        let mut result: u64 = 0;
        let mut shift = 0;

        let mut success = false;
        for b in self.buf[self.pos .. self.limit].iter() {
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

    pub fn _sigint<T: Sigint>(&mut self) -> Result<T> {
        Ok(T::decode(self._varint::<T::Varint>()?))
    }

    pub fn _fixed<T: Fixed>(&mut self) -> Result<T> {
        let tlen = size_of::<T>();
        if self.len() < tlen {
            return Err(Error::UnexpectedEOF);
        }
        let out = unsafe { read_unaligned(self.buf.as_ptr().offset(self.pos as _) as *const T) };
        self.pos += tlen;
        Ok(out)
    }

    pub fn _bytes(&mut self) -> Result<&'buf [u8]> {
        let len: usize = self._varint()?;
        if self.len() < len {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += len;
        Ok(&self.buf[self.pos - len .. self.pos])
    }

    pub fn _string(&mut self) -> Result<&str> {
        Ok(std::str::from_utf8(self._bytes()?)?)
    }

    pub fn varint<T: Varint>(&mut self, field: &mut T) -> Result<()> {
        *field = self._varint()?;
        Ok(())
    }
    pub fn sigint<T: Sigint + Default>(&mut self, field: &mut T) -> Result<()> {
        *field = self._sigint()?;
        Ok(())
    }
    pub fn protoenum<T: From<u32>>(&mut self, field: &mut T) -> Result<()> {
        *field = self._varint::<u32>()?.into();
        Ok(())
    }
    pub fn bool(&mut self, field: &mut bool) -> Result<()> {
        *field = self._varint::<u64>()? > 0;
        Ok(())
    }

    pub fn fixed32<T: Default + Fixed>(&mut self, field: &mut T) -> Result<()> {
        assert_eq!(size_of::<T>(), 4);
        *field = self._fixed()?;
        Ok(())
    }

    pub fn fixed64<T: Fixed>(&mut self, field: &mut T) -> Result<()> {
        assert_eq!(size_of::<T>(), 8);
        *field = self._fixed()?;
        Ok(())
    }

    pub fn bytes<'x, T: Bytes<'buf>>(&mut self, field: &mut T) -> Result<()> {
        field.clear();
        field.push(self._bytes()?)?;
        Ok(())
    }

    pub fn string<T: Bytes<'buf>>(&mut self, field: &mut T) -> Result<()> {
        field.clear();
        field.push(self._bytes()?)?;
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
        while self.len() > 0 {
            let tag = self._varint()?;
            proto.merge_field(tag, self)?;
        }
        assert_eq!(self.pos, start + len);
        // Bump consumed
        self.unlimit(olimit);
        Ok(())
    }

    pub fn _field_group(&mut self, _gtag: u32, proto: &mut dyn BinProto<'buf>) -> Result<()> {
        while self.len() > 0 {
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
pub struct OutputStream {
    pub(crate) buf: Vec<u8>,
}

impl OutputStream {
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Emits a raw vint onto the wire
    pub(crate) fn _varint<V: Varint>(&mut self, v: V) {
        let mut n = v.into_u64();

        while n >= 0x80 {
            self.buf.push(MSB | (n as u8));
            n >>= 7;
        }

        // dst[i] = n as u8;
        self.buf.push(n as u8);
    }

    pub(crate) fn _bytes(&mut self, v: &[u8]) {
        self.buf.extend_from_slice(v);
    }

    pub fn varint<V: Varint>(&mut self, _: u32, v: &V) {
        self._varint(*v)
    }

    pub fn sigint<V: Sigint>(&mut self, _: u32, v: &V) {
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
        let slice = unsafe { from_raw_parts(&wire as *const V::WIRE as *const u8, size_of::<V::WIRE>()) };
        self.buf.extend_from_slice(&slice);
    }

    pub fn fixed32<V: Fixed>(&mut self, _: u32, v: &V) {
        self.fixed(0, v)
    }

    pub fn fixed64<V: Fixed>(&mut self, _: u32, v: &V) {
        self.fixed(0, v)
    }

    pub fn bytes<'x, B: Bytes<'x>>(&mut self, _: u32, b: &B) {
        self._varint(b.bytes().len());
        self._bytes(b.bytes());
    }

    pub fn string<'out, B: Bytes<'out>>(&mut self, _: u32, b: &B) {
        self._varint(b.bytes().len());
        self._bytes(b.bytes());
    }

    pub fn nested<'buf, P: BinProto<'buf>>(&mut self, _: u32, v: &P) {
        let mut inner = OutputStream::default();
        v.encode(&mut inner);
        self._varint(inner.len());
        self._bytes(&inner.buf);
    }

    pub fn group<'buf, P: BinProto<'buf>>(&mut self, num: u32, v: &P) {
        // SGRP tag is encoded outside of this method
        v.encode(self);
        self._varint((num & !0b111) | EGRP as u32);
    }
}
