#![feature(entry_insert)]

extern crate core;

pub mod ast;
#[doc(hidden)]
mod deps;
pub mod parser;
pub mod translate;
pub mod validate;


pub enum ErrorKind {

}