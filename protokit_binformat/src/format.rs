use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use anyhow::bail;

use integer_encoding::VarInt;

use crate::{WriteBuffer, ReadBuffer,  Decodable, Encodable, Result};

pub trait ExtendFromBytes {
    fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<()>;
    fn _as_bytes(&self) -> &[u8];
    fn bytes_len(&self) -> usize;
}

impl<T> ExtendFromBytes for Option<T>
where
    T: ExtendFromBytes + Default,
{
    fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.get_or_insert_with(|| T::default()).extend_from_bytes(bytes)
    }

    fn _as_bytes(&self) -> &[u8] {
        match self {
            None => &[],
            Some(v) => v._as_bytes(),
        }
    }

    fn bytes_len(&self) -> usize {
        match self {
            None => 0,
            Some(v) => v.bytes_len(),
        }
    }
}

impl ExtendFromBytes for Box<str> {
    fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                let mut str = std::mem::replace(self, Box::from("")).into_string();
                str.push_str(s);
                *self = str.into_boxed_str();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    fn _as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn bytes_len(&self) -> usize {
        self.len()
    }
}

impl ExtendFromBytes for String {
    #[inline(always)]
    fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let s = std::str::from_utf8(bytes)?;
        self.push_str(s);
        Ok(())
    }

    fn _as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn bytes_len(&self) -> usize {
        self.len()
    }
}

impl ExtendFromBytes for Vec<u8> {
    fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    fn _as_bytes(&self) -> &[u8] {
        self
    }

    fn bytes_len(&self) -> usize {
        self.len()
    }
}

pub trait Format<M> {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode_val(&tag, buf)?;
        Self::encode_val(self, buf)
    }
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()>;
    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>>;
}

pub struct Bytes;
impl<B> Format<Bytes> for B
where
    B: ExtendFromBytes,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode(&tag, 0, buf)?;
        // Self::encode_val(buf)
        self.encode_val(buf)
    }

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode(&self.bytes_len(), 0, buf)?;
        buf.extend_from_slice(self._as_bytes());
        Ok(())
    }


    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (dlen, len) = usize::decode_var(buf)
            .ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        if buf.len() < dlen + len {
            return Err(anyhow::Error::msg("Mising data"));
        }
        self.extend_from_bytes(&buf[len .. dlen + len])?;
        Ok(&buf[len + dlen ..])
    }
}

pub struct Repeat<D>(PhantomData<D>);

impl<T, D> Format<Repeat<D>> for Vec<T>
where
    T: Format<D> + Default,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        for t in self {
            t.encode(tag, buf)?;
        }
        Ok(())
    }

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        panic!("Unexpected, can't encode value without tag for packed fields");
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut next = T::default();
        buf = T::decode(&mut next, buf)?;
        self.push(next);
        Ok(buf)
    }
}

pub struct Map<K, V>(PhantomData<(K, V)>);

impl<KF, VF, K: Format<KF> + Default + Hash + Eq, V: Format<VF> + Default> Format<Map<KF, VF>> for HashMap<K, V> {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        for (k, v) in self {
            let mut nest = WriteBuffer::new();
            Format::<KF>::encode(k, 1, &mut nest)?;
            Format::<VF>::encode(v, 2, &mut nest)?;

            Format::<RawVInt>::encode_val(&tag, buf)?;
            Format::<RawVInt>::encode_val(&nest.len(), buf)?;
            buf.extend_from_slice(&nest);
        }
        Ok(())
    }

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        panic!("Not applicable")
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut len = 0;

        buf = Format::<RawVInt>::decode(&mut len, buf)?;
        if buf.len() < len {
            bail!("Not enough data")
        }
        let (mut inner_buf, outer_buf) = buf.split_at(len);

        while !inner_buf.is_empty() {
            let mut k = K::default();
            let mut v = V::default();
            let mut tag = 0;
            inner_buf = Format::<RawVInt>::decode(&mut tag, inner_buf)?;
            match tag {
                1 => {
                    inner_buf = Format::<KF>::decode(&mut k, inner_buf)?;
                    inner_buf = Format::<RawVInt>::decode(&mut tag, inner_buf)?;
                    if tag != 2 {
                        bail!("Invalid tag in map entry: {tag}");
                    }
                    inner_buf = Format::<VF>::decode(&mut v, inner_buf)?
                },
                2 => {
                    inner_buf = Format::<VF>::decode(&mut v, inner_buf)?;
                    inner_buf = Format::<RawVInt>::decode(&mut tag, inner_buf)?;
                    if tag != 2 {
                        bail!("Invalid tag in map entry: {tag}");
                    }
                    inner_buf = Format::<KF>::decode(&mut k, inner_buf)?
                },
                other => {
                    bail!("Invalid tag in map entry: {other}");
                }
            }

            self.insert(k, v);
        }

        Ok(outer_buf)
    }
}

pub struct RawVInt;

impl Format<RawVInt> for u64 {
    fn encode(&self, _: u32, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode_val(self, buf)
    }

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let len = self.required_space();
        let olen = buf.len();
        buf.resize(buf.len() + len, 0);
        self.encode_var(&mut buf[olen ..]);
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (d, len) = u64::decode_var(buf).ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        *self = d;
        Ok(&buf[len ..])
    }
}

impl Format<RawVInt> for i64 {
    fn encode(&self, _: u32, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode_val(self, buf)
    }

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let target: &u64 = unsafe { std::mem::transmute(self) };
        let len = target.required_space();
        let olen = buf.len();
        buf.resize(buf.len() + len, 0);
        target.encode_var(&mut buf[olen ..]);
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (d, len) = u64::decode_var(buf).ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        unsafe {
            *self = std::mem::transmute(d);
        }
        Ok(&buf[len ..])
    }
}

macro_rules! impl_rawint {
    ($($t:ty:$d:ty),*) => {$(
    impl Format<RawVInt> for $t {
        fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            let mut v : $d = 0;
            buf = Format::<RawVInt>::decode(&mut v, buf)?;
            *self = v as $t;
            Ok(buf)
        }

        fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
            let v = self.clone() as $d;
            Format::<RawVInt>::encode(&v, 0, buf)
        }

        fn encode<'b>(&self , _: u32, buf: &'b mut WriteBuffer) -> Result<()> {
            let v = self.clone() as $d;
            Format::<RawVInt>::encode(&v, 0, buf)
        }
    }
    )*};
}

impl_rawint! {
    u8:u64, u16:u64, u32:u64, usize:u64,
    i8:i64, i16:i64, i32:i64, isize:i64
}

pub struct VInt;

impl<T> Format<VInt> for T
where
    T: Format<RawVInt>,
{
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        Format::<RawVInt>::encode(self, 0, buf)
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        Format::<RawVInt>::decode(self, buf)
    }
}

macro_rules! defer_opt_impl_body {
    ($t:ty) => {
        fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
            match self {
                Some(v) => Format::<$t>::encode(v, tag, buf),
                None => Ok(()),
            }
        }

        fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
            panic!("unexpected encode_val for optional fields");
        }

        fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            Format::<$t>::decode(self.get_or_insert_with(|| T::default()), buf)
        }
    };
}

macro_rules! defer_opt_impl_complete {
    ($($t:ty),*) => {$(
        impl<T> Format<$t> for Option<T>
        where
            T: Format<$t> + Default,
        {
            defer_opt_impl_body!{$t}
        }
    )*};
}

defer_opt_impl_complete! {VInt, SInt, Fix}

pub struct Enum;

pub trait ProtoEnum: From<u32> + Into<u32> {}

impl<T> Format<Enum> for T
where
    T: Clone + ProtoEnum,
{
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let t: u32 = self.clone().into();
        Format::<RawVInt>::encode(&t, 0, buf)
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut v = 0;
        let buf = Format::<RawVInt>::decode(&mut v, buf)?;
        *self = T::from(v);
        Ok(buf)
    }
}

impl<T> Format<Enum> for Option<T>
where
    T: Clone + ProtoEnum + Default,
{
    defer_opt_impl_body! {Enum}
}

pub struct SInt;

impl Format<SInt> for i32 {
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let len = self.required_space();
        let olen = buf.len();
        buf.resize(buf.len() + len, 0);
        self.encode_var(&mut buf[olen ..]);
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (d, len) = i32::decode_var(buf).ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        *self = d;
        Ok(&buf[len ..])
    }
}

impl Format<SInt> for i64 {
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let len = self.required_space();
        let olen = buf.len();
        buf.resize(buf.len() + len, 0);
        self.encode_var(&mut buf[olen ..]);
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (d, len) = i64::decode_var(buf).ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        *self = d;
        Ok(&buf[len ..])
    }
}

pub struct Fix;

impl Format<Fix> for bool {

    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        buf.push(if *self { 1 } else { 0 });
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        *self = buf[0] != 0;
        Ok(&buf[1 ..])
    }
}

macro_rules! impl_fix {
    ($($t:ty),*) => {$(
    impl<'a> Format<Fix> for $t {

        fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
            buf.extend_from_slice(&self.to_le_bytes());
            Ok(())
        }

        fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            if buf.len() < std::mem::size_of::<$t>() {
                bail!("Not enough bytes");
            }
            let v = <$t>::from_le_bytes(buf[0..std::mem::size_of::<$t>()].try_into()?);
            *self = v;
            Ok(& buf[::std::mem::size_of::<$t>()..])
        }

    }
    )*};
}

impl_fix! {
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64
}

pub struct Nest;

impl<T> Format<Nest> for T
where
    T: Decodable + Encodable + Default,
{
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let mut nested = WriteBuffer::new();
        self.encode(&mut nested)?;
        Format::<RawVInt>::encode(&nested.len(), 0, buf)?;
        buf.extend_from_slice(&nested);
        Ok(())
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
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
}

pub struct Pack<F>(F);

impl<T, F> Format<Pack<F>> for Vec<T>
where
    T: Format<F> + Default,
{
    fn encode_val(&self, buf: &mut WriteBuffer) -> Result<()> {
        let mut nested = WriteBuffer::new();
        for it in self {
            Format::<F>::encode_val(it, &mut nested)?;
        }
        Format::<RawVInt>::encode(&nested.len(), 0, buf)?;
        buf.extend_from_slice(&nested);
        Ok(())
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut len = 0;

        buf = Format::<RawVInt>::decode(&mut len, buf)?;
        if buf.len() < len {
            bail!("Not enough data")
        }
        let (mut inner_buf, outer_buf) = buf.split_at(len);
        while !inner_buf.is_empty() {
            let mut it = T::default();
            inner_buf = T::decode(&mut it, inner_buf)?;
            self.push(it);
        }

        Ok(outer_buf)
    }
}