use std::cmp::min;
use std::mem::{replace, size_of};
use std::ptr::read_unaligned;
use std::slice::from_raw_parts;

use crate::{BinProto, Bytes, Error, Fixed, Result, Sigint, Varint, EGRP};

pub struct InputStream<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) pos: usize,
    pub(crate) limit: usize,
}

impl<'i> InputStream<'i> {
    pub fn new(buf: &'i [u8]) -> Self {
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
    pub fn limit(&mut self, limit: usize) -> usize {
        replace(&mut self.limit, self.pos + limit)
    }

    /// Bump the current limit by offset
    pub fn bump(&mut self, off: usize) {
        self.limit += off;
    }

    pub fn skip(&mut self, tag: u32) -> Result<()> {
        match (tag & 0b111) as u8 {
            crate::VARINT => { self._varint::<u64>()?; }
            crate::BYTES => { self._bytes()?; }
            crate::FIX32 => { self._fixed::<u32>()?; }
            crate::FIX64 => { self._fixed::<u64>()?; }
            crate::SGRP => { self.skip_grp((tag & !0b111) | EGRP as u32)?; }
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
        let mut result: T = T::default();
        let mut shift = 0;

        let mut success = false;
        for b in self.buf[self.pos..].iter() {
            let msb_dropped = b & 0x7F;
            result = result | ((T::from(msb_dropped)) << shift);
            shift += 7;

            if b & 0x80 == 0 || shift > T::max_shift() {
                success = b & 0x80 == 0;
                break;
            }
        }

        if success {
            self.pos += (shift / 7) as usize;
            Ok(result)
        } else {
            Err(Error::Empty)
        }
    }

    pub fn _sigint<T: Sigint>(&mut self) -> Result<T> {
        Ok(T::decode(self._varint::<T::Varint>()?))
    }

    pub fn _fixed<T: Fixed>(&mut self) -> Result<T> {
        let tlen = size_of::<T>();
        if self.len() < tlen {
            return Err(Error::Empty);
        }
        let out = unsafe { read_unaligned(self.buf.as_ptr().offset(self.pos as _) as *const T) };
        self.pos += tlen;
        Ok(out)
    }

    pub fn _bytes(&mut self) -> Result<&[u8]> {
        let len: usize = self._varint()?;
        if self.len() < len {
            return Err(Error::Empty);
        }
        self.pos += len;
        Ok(&self.buf[self.pos - len..self.pos])
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
        *field = self._fixed()?;
        Ok(())
    }

    pub fn fixed64<T: Fixed>(&mut self, field: &mut T) -> Result<()> {
        *field = self._fixed()?;
        Ok(())
    }

    pub fn bytes<T: Bytes>(&mut self, field: &mut T) -> Result<()> {
        field.push(self._bytes()?)?;
        Ok(())
    }

    pub fn string<T: Bytes>(&mut self, field: &mut T) -> Result<()> {
        field.push(self._bytes()?)?;
        Ok(())
    }

    pub fn nested<P: BinProto>(&mut self, p: &mut P) -> Result<()> {
        self._field_nested(p)
    }

    pub fn group<P: BinProto>(&mut self, p: &mut P) -> Result<()> {
        self._field_group(0, p)
    }

    pub fn _field_nested(&mut self, proto: &mut dyn BinProto) -> Result<()> {
        let len = self._varint()?;
        if len > self.len() {
            return Err(Error::Empty);
        }
        self.limit(len);
        while self.len() > 0 {
            let tag = self._varint()?;
            proto.merge_field(tag, self)?;
        }
        // Bump consumed
        self.bump(len);
        Ok(())
    }

    pub fn _field_group(&mut self, _gtag: u32, proto: &mut dyn BinProto) -> Result<()> {
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
    pub(crate) fn _varint<V: Varint>(&mut self, mut v: V) {
        // let mut out = [0; 10];
        // let mut len = 0u8;
        // if v == 0 {
        //     self.buf.push(0);
        //     return;
        // }
        // while v > 0 && len < 10 {
        //     if v >= 128 {
        //         out[len as usize] = (v as u8 & 0x7F) | 0x80;
        //     } else {
        //         out[len as usize] = (v as u8 & 0x7F);
        //     }
        //     v >>= 7;
        //     len += 1;
        // }
        // self.buf.extend_from_slice(&out[..len]);
        // (out, len)
        //
        while v > V::default() {
            if v >= V::from(0x80u8) {
                self.buf.push(0x80 | v.low_byte() & 0x7F)
            } else {
                self.buf.push(v.low_byte() & 0x7F)
            }
            v = v >> 7
        }
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
        self.varint(0, if *b { &1 } else { &0 });
    }

    pub fn protoenum<V: Clone + Copy + Into<u32>>(&mut self, _: u32, v: &V) {
        self._varint((*v).into());
    }

    pub fn fixed<V: Fixed>(&mut self, _: u32, v: &V) {
        let wire = v.to_wire();
        let slice = unsafe { from_raw_parts(&wire as *const V::WIRE as *const u8, size_of::<V::WIRE>()) };
        self.buf.extend_from_slice(&slice);
    }

    pub fn fixed32<V: Fixed>(&mut self, _: u32, v: &V)   {
        self.fixed(0, v)
    }

    pub fn fixed64<V: Fixed>(&mut self, _: u32, v: &V) {
        self.fixed(0, v)
    }

    pub fn bytes<B: Bytes>(&mut self, _: u32, b: &B) {
        self._varint(b.bytes().len());
        self._bytes(b.bytes());
    }

    pub fn string(&mut self, _: u32, b: &String) {
        self._varint(b.bytes().len());
        self._bytes(b.bytes());
    }

    pub fn nested<P: BinProto>(&mut self, _: u32, v: &P) {
        v.encode(self)
    }

    pub fn group<P: BinProto>(&mut self, num: u32, v: &P) {
        // SGRP tag is encoded outside of this method
        v.encode(self);
        self._varint((num & !0b111) | EGRP as u32);
    }
}
