use std::collections::HashMap;
use std::marker::PhantomData;

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

pub trait Decode<M> {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()>;
    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>>;
}

pub struct Bytes;
impl<B> Decode<Bytes> for B
where
    B: ExtendFromBytes,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
        Decode::<RawVInt>::encode(&self.bytes_len(), 0, buf)?;
        buf.extend_from_slice(self._as_bytes());
        Ok(())
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let (dlen, len) = usize::decode_var(buf).ok_or_else(|| anyhow::Error::msg("Missing data"))?;
        self.extend_from_bytes(&buf[len .. dlen + len])?;
        Ok(&buf[len + dlen ..])
    }
}

pub struct Repeat<D>(PhantomData<D>);

impl<T, D> Decode<Repeat<D>> for Vec<T>
where
    T: Decode<D> + Default,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        for t in self {
            t.encode(tag, buf)?;
        }
        Ok(())
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut next = T::default();
        buf = T::decode(&mut next, buf)?;
        self.push(next);
        Ok(buf)
    }
}

pub struct Map<K, V>(PhantomData<(K, V)>);

impl<KF, VF, K: Decode<KF>, V: Decode<VF>> Decode<Map<KF, VF>> for HashMap<K, V> {
    fn encode(&self, _tag: u32, _buf: &mut WriteBuffer) -> Result<()> {
        todo!()
    }

    fn decode<'b>(&mut self, _buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        todo!()
    }
}

pub struct RawVInt;

impl Decode<RawVInt> for u64 {
    fn encode(&self, _tag: u32, buf: &mut WriteBuffer) -> Result<()> {
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

impl Decode<RawVInt> for i64 {
    fn encode(&self, _tag: u32, buf: &mut WriteBuffer) -> Result<()> {
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
    impl Decode<RawVInt> for $t {
        fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            let mut v : $d = 0;
            buf = Decode::<RawVInt>::decode(&mut v, buf)?;
            *self = v as $t;
            Ok(buf)
        }

        fn encode<'b>(&self ,_tag: u32, buf: &'b mut WriteBuffer) -> Result<()> {
            let v = self.clone() as $d;
            Decode::<RawVInt>::encode(&v, 0, buf)
        }
    }
    )*};
}

impl_rawint! {
    u8:u64, u16:u64, u32:u64, usize:u64,
    i8:i64, i16:i64, i32:i64, isize:i64
}

pub struct VInt;

impl<T> Decode<VInt> for T
where
    T: Decode<RawVInt>,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
        Decode::<RawVInt>::encode(self, 0, buf)
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        Decode::<RawVInt>::decode(self, buf)
    }
}

macro_rules! defer_opt_impl_body {
    ($t:ty) => {
        fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
            match self {
                Some(v) => Decode::<$t>::encode(v, tag, buf),
                None => Ok(()),
            }
        }

        fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            Decode::<$t>::decode(self.get_or_insert_with(|| T::default()), buf)
        }
    };
}

macro_rules! defer_opt_impl_complete {
    ($($t:ty),*) => {$(
        impl<T> Decode<$t> for Option<T>
        where
            T: Decode<$t> + Default,
        {
            defer_opt_impl_body!{$t}
        }
    )*};
}

defer_opt_impl_complete! {VInt, SInt, Fix}

pub struct Enum;

pub trait ProtoEnum: From<u32> + Into<u32> {}

impl<T> Decode<Enum> for T
where
    T: Clone + ProtoEnum,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        let t: u32 = self.clone().into();
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
        Decode::<RawVInt>::encode(&t, 0, buf)
    }

    fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut v = 0;
        let buf = Decode::<RawVInt>::decode(&mut v, buf)?;
        *self = T::from(v);
        Ok(buf)
    }
}

impl<T> Decode<Enum> for Option<T>
where
    T: Clone + ProtoEnum + Default,
{
    defer_opt_impl_body! {Enum}
}

pub struct SInt;

impl Decode<SInt> for i32 {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        let len = self.required_space();
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
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
impl Decode<SInt> for i64 {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        let len = self.required_space();
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
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

impl Decode<Fix> for bool {
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
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
    impl<'a> Decode<Fix> for $t {
        fn decode<'b>(&mut self, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
            let v = <$t>::from_le_bytes(buf.try_into()?);
            *self = v;
            Ok(& buf[::std::mem::size_of::<$t>()..])
        }
        fn encode<'b>(&self, tag: u32, buf: &'b mut WriteBuffer) -> Result<()> {
            Decode::<RawVInt>::encode(&tag, 0, buf)?;
            buf.extend_from_slice(&self.to_le_bytes());
            Ok(())
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

// impl<T: Decodable + Encodable + Default> Format<Option<T>> for Nest<T> {
//     fn decode<'b>(target: &mut Option<T>, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         Nest::decode(target.get_or_insert_with(|| Default::default()), buf)
//     }
//
//     fn encode(target: &Option<T>, tag: u32, buf: &mut Buffer) -> Result<()> {
//         match target {
//             None => Ok(()),
//             Some(v) => Nest::encode(v, tag, buf),
//         }
//     }
// }

impl<T> Decode<Nest> for T
where
    T: Decodable + Encodable + Default,
{
    fn encode(&self, tag: u32, buf: &mut WriteBuffer) -> Result<()> {
        Decode::<RawVInt>::encode(&tag, 0, buf)?;
        // println!("Encoded tag: {:?}", buf);
        let mut nested = WriteBuffer::new();
        self.encode(&mut nested)?;
        Decode::<RawVInt>::encode(&nested.len(), 0, buf)?;
        // println!("Extending {:?} into {:?}", nested, buf);
        buf.extend_from_slice(&nested);
        Ok(())
    }

    fn decode<'b>(&mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        let mut len = 0;
        let mut tag = 0xFF;

        buf = Decode::<RawVInt>::decode(&mut len, buf)?;
        assert!(buf.len() >= len);
        let (mut inner_buf, outer_buf) = buf.split_at(len);
        while !inner_buf.is_empty() {
            inner_buf = Decode::<RawVInt>::decode(&mut tag, inner_buf)?;
            inner_buf = self.merge_field(tag, inner_buf)?;
        }

        Ok(outer_buf)
    }
}

// impl<T> Decode<Repeat<T>> for Vec<T>
// where
//     T: Decode<Nest<T>> + Default,
// {
//     fn encode(&self, tag: u32, buf: &mut Buffer) -> Result<()> {
//         for t in self {
//             Decode::<Nest<T>>::encode(t, tag, buf)?;
//         }
//         Ok(())
//     }
//
//     fn decode<'a, 'b>(&'a mut self, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         let mut next = T::default();
//         buf = Decode::<Nest<T>>::decode(&mut next, buf)?;
//         self.push(next);
//         Ok(buf)
//     }
// }

// pub fn nest_decode<'b, T: Decodable + Default>(target: &mut T, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//     let mut len = 0;
//     let mut tag = 0xFF;
//
//     buf = RawVInt::decode(&mut len, buf)?;
//     assert!(buf.len() >= len);
//     let (mut inner_buf, outer_buf) = buf.split_at(len);
//     while !inner_buf.is_empty() {
//         inner_buf = RawVInt::decode(&mut tag, inner_buf)?;
//         inner_buf = target.merge_field(tag, inner_buf)?;
//     }
//
//     Ok(outer_buf)
// }
//
// impl<T: Decodable + Encodable + Default> Format<T> for Nest<T> {
//     fn decode<'b>(target: &mut T, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         let mut len = 0;
//         let mut tag = 0xFF;
//
//         buf = RawVInt::decode(&mut len, buf)?;
//         assert!(buf.len() >= len);
//         let (mut inner_buf, outer_buf) = buf.split_at(len);
//         while !inner_buf.is_empty() {
//             inner_buf = RawVInt::decode(&mut tag, inner_buf)?;
//             inner_buf = target.merge_field(tag, inner_buf)?;
//         }
//
//         Ok(outer_buf)
//     }
//     fn encode(target: &T, tag: u32, buf: &mut Buffer) -> Result<()> {
//         RawVInt::encode(&tag, 0, buf)?;
//         // println!("Encoded tag: {:?}", buf);
//         let mut nested = Vec::new();
//         target.encode(&mut nested)?;
//         RawVInt::encode(&nested.len(), 0, buf)?;
//         // println!("Extending {:?} into {:?}", nested, buf);
//         buf.extend_from_slice(&nested);
//         Ok(())
//     }
// }

// pub struct Map<K, V>(K, V);

// impl<K: Default, KF: Format<K>, V: Default, VF: Format<V>> Format<HashMap<K, V>> for Map<KF, VF> {
//     fn decode<'b>(_target: &mut HashMap<K, V>, _buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         todo!()
//     }
//     fn encode(_target: &HashMap<K, V>, _tag: u32, _buf: &mut Buffer) -> Result<()> {
//         unimplemented!()
//     }
// }

pub struct Pack<F>(F);

impl<T, F> Decode<Pack<F>> for Vec<T>
where
    T: Decode<F>,
{
    fn encode(&self, _tag: u32, _buf: &mut WriteBuffer) -> Result<()> {
        todo!()
    }

    fn decode<'b>(&mut self, _buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
        todo!()
    }
}
//
// impl<T: Default, F: Format<T>> Format<Vec<T>> for Pack<F> {
//     fn decode<'b>(target: &mut Vec<T>, buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         let mut len = 0;
//         let mut buf = Decode::<RawVInt>::decode(&mut len, buf)?;
//         assert!(buf.len() >= len);
//         while !buf.is_empty() {
//             let mut next = T::default();
//             buf = F::decode(&mut next, buf)?;
//             target.push(next);
//         }
//
//         Ok(buf)
//     }
//     fn encode(target: &Vec<T>, tag: u32, buf: &mut Buffer) -> Result<()> {
//         Decode::<RawVInt>::encode(&tag, 0, buf)?;
//         let mut nested = Vec::new();
//         for t in target {
//             F::encode(t, 0, &mut nested)?;
//         }
//         Decode::<RawVInt>::encode(&nested.len(), 0, buf)?;
//         buf.extend_from_slice(&nested);
//         Ok(())
//     }
// }

// pub struct Rep<F>(F);
//
// impl<T: Default, F: Format<T>> Format<Vec<T>> for Rep<F> {
//     fn decode<'b>(target: &mut Vec<T>, mut buf: ReadBuffer<'b>) -> Result<ReadBuffer<'b>> {
//         let mut next = T::default();
//         buf = F::decode(&mut next, buf)?;
//         target.push(next);
//         Ok(buf)
//     }
//     fn encode(target: &Vec<T>, tag: u32, buf: &mut Buffer) -> Result<()> {
//         for t in target {
//             F::encode(t, tag, buf)?;
//         }
//         Ok(())
//     }
// }
