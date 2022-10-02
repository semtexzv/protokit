use std::marker::PhantomData;

pub use async_trait::async_trait;
use binformat::WriteBuffer;
use bytes::Buf;
pub use futures::future::LocalBoxFuture;
pub use futures::stream::Stream;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
pub use tonic::codegen::*;
pub use tonic::{Code, Status};
pub use {futures, tonic};

/// A [`Codec`] that implements `application/grpc+proto` via the protokit library..
#[derive(Debug, Clone)]
pub struct TonicCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<E, D> Default for TonicCodec<E, D> {
    fn default() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<E, D> Codec for TonicCodec<E, D>
where
    E: binformat::Encodable + Send + 'static,
    D: binformat::Decodable + Default + Send + 'static,
{
    type Encode = E;
    type Decode = D;

    type Encoder = TonicEncoder<E>;
    type Decoder = TonicDecoder<D>;

    fn encoder(&mut self) -> Self::Encoder {
        TonicEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        TonicDecoder(PhantomData)
    }
}

/// A [`Encoder`] that knows how to encode `T`.
#[derive(Debug, Clone, Default)]
pub struct TonicEncoder<T>(PhantomData<T>);

impl<T: binformat::Encodable> Encoder for TonicEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        use bytes::BufMut;

        let mut tmp = WriteBuffer::new();
        item.encode(&mut tmp).expect("Message only errors if not enough space");
        buf.put_slice(&tmp);

        Ok(())
    }
}

/// A [`Decoder`] that knows how to decode `U`.
#[derive(Debug, Clone, Default)]
pub struct TonicDecoder<D>(PhantomData<D>);

impl<D: binformat::Decodable + Default> Decoder for TonicDecoder<D> {
    type Item = D;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let b = buf.chunk();
        let mut item = D::default();
        let left = binformat::decode_into(b, &mut item).map_err(from_decode_error)?;

        buf.advance(b.len() - left.len());
        Ok(Some(item))
    }
}

fn from_decode_error(error: anyhow::Error) -> tonic::Status {
    // Map Protobuf parse errors to an INTERNAL status code, as per
    // https://github.com/grpc/grpc/blob/master/doc/statuscodes.md
    Status::new(Code::Internal, error.to_string())
}
