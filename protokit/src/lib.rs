#![feature(allocator_api)]

pub use anyhow::Result;
pub use binformat::{self, BinProto, BytesLike, Fixed, Sigint, Varint};
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use indexmap::IndexMap;
#[cfg(feature = "textformat")]
pub use textformat::{self, TextField as _, TextProto};
