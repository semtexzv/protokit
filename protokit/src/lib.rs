#![feature(allocator_api)]

use std::alloc::{Allocator, Global};
use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

pub use anyhow::Result;
pub use binformat::{BinProto, BytesLike, Fixed, Sigint, Varint};
use bumpalo::Bump;
pub use derive::{protoenum, Proto};
#[cfg(feature = "grpc")]
pub use grpc;
pub use indexmap::IndexMap;
pub use textformat::{TextField as _, TextProto};
pub use {binformat, textformat};

// /// A collection of types describing how the indirect values (nested objects, strings and bytes)
// /// should be stored.
// trait Config<'ptr, 'buf>: Debug {
//     type Ptr<T: 'ptr + Debug>: Deref<Target=T> + Debug + Sized;
//     type Seq<T: 'ptr + Debug>: Deref<Target=[T]> + Debug + Sized;
//     type Str: Deref<Target=str> + Debug;
//     type Buf: Deref<Target=[u8]> + Debug;
// }
//
// // trait ConfigCreate<'a>: Config<'a> {
// //     fn create<T: Default>(&self) -> Self::Ptr<T>;
// // }
//
// #[derive(Debug, Clone, Copy)]
// struct OfOwned;
//
// impl<'ptr, 'buf> Config<'ptr, 'buf> for OfOwned {
//     type Ptr<T: 'ptr + Debug> = Box<T>;
//     type Seq<T: 'ptr + Debug> = Vec<T>;
//     type Str = String;
//     type Buf = Vec<u8>;
// }
//
// //
// // pub struct OfRef<'buf>(PhantomData<&'buf ()>);
// //
// // impl<'buf> Config for OfRef<'buf> {
// //     type Ptr<T> = &'buf T;
// //     type Seq<T> = Vec<T>;
// //     type Str<'buf> = &'buf str;
// //     type Buf< = &'buf [u8];
// // }
// //
// // pub struct OfCow<'buf>(PhantomData<&'buf ()>);
// //
// // impl<'buf> Config<'buf> for OfCow<'buf> {
// //     type Ptr<T: 'buf> = Box<T>;
// //     type Seq<T: 'buf> = Vec<T>;
// //     type Str = Cow<'buf, str>;
// //     type Buf = Cow<'buf, [u8]>;
// // }
// //
// #[derive(Debug, Clone)]
// pub struct OfBump<'arena>(&'arena Bump);
//
// impl<'arena, 'buf> Config<'arena, 'buf> for OfBump<'arena> {
//     type Ptr<T: 'arena + Debug> = bumpalo::boxed::Box<'arena, T>;
//     type Seq<T: 'arena + Debug> = bumpalo::collections::Vec<'arena, T>;
//     type Str = String;
//     type Buf = Vec<u8>;
// }
//
// #[derive(Debug)]
// pub struct OfBumpRef<'arena, 'buf>(&'arena Bump, &'buf ());
//
// impl<'arena, 'buf> Config<'arena, 'buf> for OfBumpRef<'arena, 'buf> {
//     type Ptr<T: 'arena + Debug> = bumpalo::boxed::Box<'arena, T>;
//     type Seq<T: 'arena + Debug> = bumpalo::collections::Vec<'arena, T>;
//     type Str = bumpalo::collections::String<'arena>;
//     type Buf = bumpalo::collections::Vec<'arena, u8>;
// }

// #[derive(Debug, Clone)]
// struct Example<'ptr, 'buf, C: Config<'ptr, 'buf>> where Self: 'ptr
// {
//     recur: C::Ptr<Example<'ptr, 'buf, C>>,
//     repeat: C::Seq<Example<'ptr, 'buf, C>>,
//     str: C::Str,
//     buf: C::Buf,
// }
//
// #[derive(Debug, Clone)]
// pub struct E<'buf, A: Allocator> {
//     recur: Box<Self, A>,
//     seq: Vec<Self, A>,
//     str: &'buf str,
//     buf: &'buf [u8],
// }
//
// impl<'ptr, 'buf, C: Config<'ptr, 'buf>> Clone for Example<'ptr, 'buf, C> {
//     fn clone(&self) -> Self {
//         Self {
//             recur: self.recur.clone(),
//             repeat: self.repeat.clone(),
//             str: self.str.clone(),
//             buf: self.buf.clone(),
//         }
//     }
// }
//
// type ExampleOwned = Example<'static, 'static, OfOwned>;
// type ExampleBump<'bump> = Example<'bump, 'static, OfBump<'bump>>;
// type ExampleBumpRef<'bump, 'buf> = Example<'bump, 'buf, OfBumpRef<'bump, 'buf>>;
//
// fn x(x: E<'_, Global>) {
//     let x = x.clone();
//     panic!("{:?}", x);
//     // panic!("{:?}", x);
// }
// // type XRef<'buf> = X<'buf, OfRef<'buf>>;
