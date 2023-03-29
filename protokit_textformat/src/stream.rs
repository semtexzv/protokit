use std::fmt::{Display, Write};
use std::str::FromStr;

use logos::Lexer;

use crate::escape::escape_bytes_to;
use crate::Token::*;
use crate::{unexpected, unknown, Enum, Result, TextField, TextProto, Token};

pub struct InputStream<'buf> {
    pub(crate) lex: Lexer<'buf, Token>,
    pub(crate) cur: Token,
    // Whether we are parsing root message, or we're expecting more braces
    root: bool,
}

impl<'buf> InputStream<'buf> {
    pub fn new(data: &'buf str) -> Self {
        let lex = Lexer::new(data);
        Self {
            lex,
            cur: StartOfFile,
            root: true,
        }
    }

    pub fn next(&mut self) -> Token {
        self.advance();
        println!("Move to {:?}: {}", self.cur, self.lex.slice());
        self.cur
    }

    pub fn advance(&mut self) {
        self.cur = self.lex.next().unwrap_or(EndOfFile);
    }

    pub fn buf(&self) -> &'buf str {
        self.lex.slice()
    }

    pub fn field(&self) -> &'buf str {
        self.lex.slice()
    }

    fn token_and_span(&self) -> (Token, &'buf str) {
        (self.cur, self.lex.slice())
    }

    /// Asserts that we're on a specified token, and advances one position
    pub fn expect_consume(&mut self, kind: Token) -> Result<Token> {
        if self.cur != kind {
            return unexpected(kind, self.cur);
        }
        Ok(self.next())
    }

    fn expect_curr(&self, kind: Token) -> Result<()> {
        if self.cur != kind {
            return unexpected(kind, self.cur);
        }
        Ok(())
    }

    /// If current token == kind, then advance and return true, otherwise return false
    pub fn try_consume(&mut self, kind: Token) -> bool {
        if self.cur == kind {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn string<F: FnMut(&'buf str)>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf();
            // TODO: Escape + Trim quotes
            f(&buf[1 .. buf.len() - 1]);
            let _ = self.next();
        }
        Ok(())
    }

    pub fn bytes<F: FnMut(&'buf [u8])>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf().as_bytes();
            // TODO: Escape + Trim quotes
            f(&buf[1 .. buf.len() - 1]);
            let _ = self.next();
        }
        Ok(())
    }

    fn ident(&mut self) -> Result<&'buf str> {
        if self.cur == Ident {
            return Ok(self.buf());
        } else {
            return unexpected(Ident, self.cur);
        }
    }

    pub fn bool(&mut self) -> Result<bool> {
        let out = match self.token_and_span() {
            (Ident, "t" | "T" | "True" | "true") => Ok(true),
            (Ident, "f" | "F" | "False" | "false") => Ok(false),
            (DecLit, x) => Ok(u64::from_str(x).unwrap_or(0) != 0),
            (kind, _) => unexpected(Ident, kind),
        }?;
        let _ = self.next();
        Ok(out)
    }

    pub fn u64(&mut self) -> Result<u64> {
        let out = match self.token_and_span() {
            (HexLit, h) => u64::from_str_radix(&h[2 ..], 16)?,
            (DecLit, h) => u64::from_str_radix(&h, 10)?,
            (OctLit, h) => u64::from_str_radix(&h, 8)?,
            (tok, _) => return unexpected(DecLit, tok),
        };
        let _ = self.next();
        Ok(out)
    }

    pub fn u32(&mut self) -> Result<u32> {
        let out = match self.token_and_span() {
            (HexLit, h) => u32::from_str_radix(&h[2 ..], 16)?,
            (DecLit, h) => u32::from_str_radix(&h, 10)?,
            (OctLit, h) => u32::from_str_radix(&h, 8)?,
            (tok, _) => return unexpected(DecLit, tok),
        };
        let _ = self.next();
        Ok(out)
    }

    pub fn i64(&mut self) -> Result<i64> {
        let neg = if self.try_consume(Minus) { -1 } else { 1 };

        Ok(neg * TryInto::<i64>::try_into(self.u64()?)?)
    }

    pub fn i32(&mut self) -> Result<i32> {
        let neg = if self.try_consume(Minus) { -1 } else { 1 };

        Ok(neg * TryInto::<i32>::try_into(self.u32()?)?)
    }

    pub fn f64(&mut self) -> Result<f64> {
        let neg = if self.try_consume(Minus) { -1.0 } else { 1.0 };
        Ok(neg
            * match self.token_and_span() {
                (FltLit | DecLit, s) => f64::from_str(s).unwrap(),
                (Ident, txt) => {
                    if txt.eq_ignore_ascii_case("infinity") | txt.eq_ignore_ascii_case("inf") {
                        f64::INFINITY
                    } else if txt.eq_ignore_ascii_case("nan") {
                        f64::NAN
                    } else {
                        return unknown(txt);
                    }
                }
                (k, _) => return unexpected(FltLit, k),
            })
    }

    fn f32(&mut self) -> Result<f64> {
        Ok(self.f64()? as _)
    }

    pub(crate) fn message_fields(&mut self, p: &mut dyn TextProto<'buf>) -> Result<()> {
        let allow_eof = self.root;
        self.root = false;
        self.next();
        loop {
            match self.cur {
                Ident | ExtIdent => {
                    p.merge_field(self)?;
                    if self.cur == Comma || self.cur == Semi {
                        self.next();
                    }
                }
                RBrace => {
                    self.next();
                    return Ok(());
                }
                EndOfFile if allow_eof => return Ok(()),
                other => return unexpected(Ident, other),
            }
        }
    }
}

#[derive(Debug)]
pub struct OutputStream {
    pub(crate) buf: String,
    pad: usize,
}

impl OutputStream {
    pub fn new() -> Self {
        Self {
            buf: "".to_string(),
            pad: 0,
        }
    }
    pub fn ln(&mut self) {
        self.buf.push('\n');
        for p in 0 .. self.pad {
            self.buf.push(' ');
        }
    }

    pub fn enter(&mut self) {
        self.pad += 4;
    }
    pub fn exit(&mut self) {
        self.pad -= 4;
    }
    pub fn push(&mut self, i: &str) {
        self.buf.push_str(i);
    }

    pub fn space(&mut self) {
        self.buf.push(' ');
    }
    pub fn disp(&mut self, d: &dyn Display) {
        write!(self.buf, "{d}").unwrap();
    }
    pub fn ident(&mut self, id: &str) {
        self.buf.push_str(id);
    }

    pub fn lbracket(&mut self) {
        self.buf.push_str("[");
    }
    pub fn rbracket(&mut self) {
        self.buf.push_str("]");
    }

    pub fn lbrace(&mut self) {
        self.buf.push_str("{");
    }
    pub fn rbrace(&mut self) {
        self.buf.push_str("}");
    }

    pub fn colon(&mut self) {
        self.buf.push(':');
    }
    pub fn bytes(&mut self, b: &[u8]) {
        self.buf.push('"');
        // TODO: escape
        escape_bytes_to(b, &mut self.buf);
        self.buf.push('"');
    }
    pub fn string(&mut self, s: &str) {
        self.buf.push('"');
        // TODO: escape properly, this won't
        escape_bytes_to(s.as_bytes(), &mut self.buf);
        self.buf.push('"');
    }

    pub fn emit_field<'any, F: TextField<'any>>(&mut self, name: &str, f: &F) {
        f.emit(name, self)
    }

    pub fn emit_oneof<'any, P: TextProto<'any>>(&mut self, o: &Option<P>) {
        if let Some(o) = o {
            o.encode(self)
        }
    }
}
