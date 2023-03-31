use std::collections::BTreeMap;
use std::fmt::Display;
use std::num::{ParseIntError, TryFromIntError};
use std::ops::{Deref, DerefMut};
use std::str::{FromStr, Utf8Error};

use thiserror::Error;

mod escape;
mod lex;
pub mod reflect;
pub mod stream;

use lex::Token;
use lex::Token::*;
pub use stream::{InputStream, OutputStream};

use crate::escape::unescape;
use crate::reflect::Registry;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token: expected: {exp:?} got: {got:?}")]
    Unexpected { exp: Token, got: Token },

    #[error("Unknown identifier: {0}")]
    Unknown(String),

    #[error("Invalid integer: {0}")]
    InvalidInt(#[from] ParseIntError),

    #[error("Integer out of range: {0}")]
    IntRange(#[from] TryFromIntError),

    #[error("String not valid UTF8")]
    UTF8(#[from] Utf8Error),
}

#[cold]
pub fn unexpected<T>(exp: Token, got: Token) -> Result<T> {
    Err(Error::Unexpected { exp, got })
}

#[cold]
pub fn unknown<T>(name: &str) -> Result<T> {
    Err(Error::Unknown(name.to_string()))
}

pub trait TextProto<'buf> {
    /// Parse a message from a stream
    ///
    /// Start position: `Token::StartOfFile` or `Token::LBrace`
    /// End position: `Token::EndOfFile` or `Token::RBrace`
    fn decode(&mut self, stream: &mut InputStream<'buf>) -> Result<()>
        where
            Self: Sized,
    {
        stream.message_fields(self)
    }

    /// Merge a single field into this message from input stream
    ///
    /// Stream position: Identifier token
    fn merge_field(&mut self, stream: &mut InputStream<'buf>) -> Result<()>;

    /// Encode this message contents into the provided output stream
    fn encode(&self, stream: &mut OutputStream);
}

pub trait Enum: From<u32> + Into<u32> + FromStr<Err=Error> + Display {}

impl<'buf, T> TextProto<'buf> for Box<T>
    where
        T: TextProto<'buf>,
{
    fn decode(&mut self, stream: &mut InputStream<'buf>) -> Result<()>
        where
            Self: Sized,
    {
        self.deref_mut().decode(stream)
    }

    fn merge_field(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
        self.deref_mut().merge_field(stream)
    }

    fn encode(&self, stream: &mut OutputStream) {
        self.deref().encode(stream)
    }
}

/// Method implementing Field::emit (extracted for reusability)
fn _emit<'any, F: TextField<'any> + ?Sized>(f: &F, name: &str, stream: &mut OutputStream) {
    stream.ln();
    stream.push(name);
    if !F::is_message() {
        stream.colon()
    }
    stream.space();
    f.emit_value(stream);
}

pub trait TextField<'buf> {
    fn is_message() -> bool {
        false
    }

    /// Merge value from stream into the current self
    ///
    /// Stream position: Field identifier token
    fn merge_text(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
        stream.expect_consume(Ident)?;
        if !Self::is_message() {
            stream.expect_consume(Colon)?;
        }
        self.merge_value(stream)
    }
    /// Merges a value from the stream into self
    ///
    /// Stream position: Value input (scalar literal, left bracket/brace)
    fn merge_value(&mut self, stream: &mut InputStream<'buf>) -> Result<()>;

    /// Emit a complete field entry in form of
    ///
    /// ident: scalar-value
    /// or:
    /// ident message-value
    fn emit(&self, name: &str, stream: &mut OutputStream) {
        _emit(self, name, stream)
    }
    /// Emit just the value stored in this field
    fn emit_value(&self, stream: &mut OutputStream);
}


//

//

//         Ok(())
//     }
//
//     fn emit(&self, name: &str, stream: &mut OutputStream) {

//     }
//
//     fn emit_value(&self, stream: &mut OutputStream) {
//         todo!()
//     }
// }

impl<'buf> TextField<'buf> for bool {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.bool()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.push(if *self { "true" } else { "false" });
    }
}

impl<'buf> TextField<'buf> for i32 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.i32()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for i64 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.i64()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for u32 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.u32()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for u64 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.u64()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for f32 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.f64()? as _;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for f64 {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        *self = stream.f64()?;
        Ok(())
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.disp(self);
    }
}

impl<'buf> TextField<'buf> for String {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        stream.string(|s| {
            if s.contains('\\') {
                let unescaped = unescape(s.bytes()).collect::<Vec<_>>();
                self.push_str(std::str::from_utf8(&unescaped)?);
            } else {
                self.push_str(s);
            }
            Ok(())
        })
    }

    fn emit(&self, name: &str, stream: &mut OutputStream) {
        if self.len() > 0 {
            _emit(self, name, stream)
        }
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.string(self);
    }
}

impl<'buf> TextField<'buf> for &'buf str {
    fn merge_value(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
        stream.string(|s| {
            *self = s;
            if s.contains('\\') {
                panic!("Need to escape");
            }
            Ok(())
        })
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.string(self)
    }
}

impl<'buf> TextField<'buf> for Vec<u8> {
    fn merge_value(&mut self, stream: &mut InputStream) -> Result<()> {
        stream.bytes(|s| {
            if s.contains(&b'\\') {
                let unescaped = unescape(s.iter().cloned()).collect::<Vec<_>>();
                self.extend_from_slice(&unescaped);
            } else {
                self.extend_from_slice(s);
            }
            Ok(())
        })
    }

    fn emit(&self, name: &str, stream: &mut OutputStream) {
        if self.len() > 0 {
            _emit(self, name, stream)
        }
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.bytes(self);
    }
}

impl<'buf, F> TextField<'buf> for F
    where
        F: TextProto<'buf>,
{
    fn is_message() -> bool {
        true
    }

    fn merge_value(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
        self.decode(stream)
    }

    fn emit(&self, name: &str, stream: &mut OutputStream) {
        stream.ln();
        stream.push(name);
        stream.space();
        self.emit_value(stream);
        stream.ln();
    }

    fn emit_value(&self, stream: &mut OutputStream) {
        stream.lbrace();
        stream.enter();
        TextProto::encode(self, stream);
        stream.exit();
        stream.ln();
        stream.rbrace();
    }
}

pub fn merge_single<'buf, T: TextField<'buf>>(
    v: &mut T,
    stream: &mut InputStream<'buf>,
) -> Result<()> {
    v.merge_text(stream)
}


pub fn merge_oneof<'buf, T: TextProto<'buf> + Default>(
    oneof: &mut Option<T>,
    stream: &mut InputStream<'buf>,
) -> Result<()> {
    oneof.get_or_insert_with(Default::default).merge_field(stream)
}

pub fn merge_optional<'buf, T: TextField<'buf> + Default>(
    v: &mut Option<T>,
    stream: &mut InputStream<'buf>,
) -> Result<()> {
    v.get_or_insert_with(Default::default).merge_text(stream)
}

pub fn merge_repeated<'buf, T: TextField<'buf> + Default>(
    out: &mut Vec<T>,
    stream: &mut InputStream<'buf>,
) -> Result<()> {
    let is_list = stream.try_consume(LBracket);
    loop {
        out.push(T::default());
        out.last_mut().unwrap().merge_value(stream)?;
        match stream.cur {
            // End of the list
            RBracket | EndOfFile if is_list => {
                // In this case we must advance one past the rbracket
                stream.next();
                return Ok(());
            }
            // Comma as elem separator
            Comma if is_list => {
                stream.next();
                continue;
            }
            // This was the last entry in this field, return
            _ if !is_list => {
                return Ok(());
            }
            other => return unexpected(RBracket, other),
        }
    }
}

pub fn merge_map<'buf, K, V>(
    field: &mut BTreeMap<K, V>,
    stream: &mut InputStream<'buf>,
) -> Result<()>
    where K: TextField<'buf> + Default + PartialEq + Ord,
          V: TextField<'buf> + Default
{
    #[derive(Default)]
    struct Help<K, V> {
        key: Option<K>,
        value: V,
    }
    impl<'buf, K: TextField<'buf> + Default, V: TextField<'buf> + Default> TextProto<'buf> for Help<K, V> {
        fn merge_field(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            match stream.field() {
                "key" => merge_optional(&mut self.key, stream),
                "value" => merge_single(&mut self.value, stream),
                other => unknown(other),
            }
        }

        fn encode(&self, stream: &mut OutputStream) {
            emit_optional(&self.key, "key", stream);
            emit_raw(&self.value, "value", stream);
        }
    }

    let mut help = Vec::<Help<K, V>>::default();

    merge_repeated(&mut help, stream)?;

    for it in help {
        if let Some(k) = it.key {
            field.insert(k, it.value);
        }
    }

    Ok(())
}

pub fn emit_raw<'buf, F: TextField<'buf>>(o: &F, name: &'static str, stream: &mut OutputStream) {
    o.emit(name, stream)
}

pub fn emit_single<'buf, F: TextField<'buf> + Default + PartialEq>(
    field: &F,
    name: &'static str,
    stream: &mut OutputStream,
) {
    if field != &Default::default() {
        emit_raw(field, name, stream)
    }
}

pub fn emit_optional<'buf, F: TextField<'buf> + Default>(
    field: &Option<F>,
    name: &'static str,
    stream: &mut OutputStream,
) {
    if let Some(field) = field {
        emit_raw(field, name, stream)
    }
}

pub fn emit_repeated<'buf, F: TextField<'buf> + Default + PartialEq>(
    field: &Vec<F>,
    name: &'static str,
    stream: &mut OutputStream,
) {
    if field.len() > 0 {
        stream.ln();
        stream.push(name);
        if !F::is_message() {
            stream.push(": [")
        } else {
            stream.push(" [")
        }
        if F::is_message() {
            stream.enter();
            stream.ln();
        }
        for (i, it) in field.iter().enumerate() {
            it.emit_value(stream);
            if i != field.len() - 1 {
                stream.push(",");
                if F::is_message() {
                    stream.ln();
                } else {
                    stream.space();
                }
            }
        }

        if F::is_message() {
            stream.exit();
            stream.ln();
        }
        stream.rbracket();
    }
}

pub fn emit_map<'buf, K: TextField<'buf>, V: TextField<'buf>>(
    field: &BTreeMap<K, V>,
    name: &'static str,
    stream: &mut OutputStream,
) {
    if field.len() > 0 {
        stream.ln();
        stream.push(name);
        stream.space();
        stream.lbracket();
        stream.enter();
        field.iter().for_each(|(k, v)| {
            stream.ln();
            stream.lbrace();
            stream.enter();
            k.emit("key", stream);
            v.emit("value", stream);
            stream.exit();
            stream.ln();
            stream.rbrace();
        });
        stream.exit();
        stream.ln();
        stream.rbracket();
    }
}

pub fn emit_oneof<'any, P: TextProto<'any>>(o: &Option<P>, stream: &mut OutputStream) {
    if let Some(o) = o {
        o.encode(stream)
    }
}

pub fn decode<'buf, T: TextProto<'buf> + Default>(data: &'buf str, reg: &Registry) -> Result<T> {
    let mut out = T::default();
    let mut stream = InputStream::new(data);
    out.decode(&mut stream)?;
    Ok(out)
}

pub fn encode<'any, T: TextProto<'any>>(b: &T, reg: &Registry) -> Result<String> {
    let mut out = OutputStream::new();
    b.encode(&mut out);
    Ok(out.buf)
}
