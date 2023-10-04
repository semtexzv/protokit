#![allow(clippy::from_str_radix_10)]

use core::fmt::{Display, Write};
use core::str::FromStr;

use logos::Lexer;

use crate::escape::escape_bytes_to;
use crate::reflect::Registry;
use crate::Token::*;
use crate::{unexpected, unknown, Result, TextProto, Token};

pub struct InputStream<'buf> {
    pub(crate) lex: Lexer<'buf, Token>,
    pub(crate) cur: Token,
    // TODO: will be required for any support
    pub(crate) _reg: &'buf Registry,
    // Whether we are parsing root message, or we're expecting more braces
    root: bool,
}

impl<'buf> InputStream<'buf> {
    pub fn new(data: &'buf str, reg: &'buf Registry) -> Self {
        let lex = Lexer::new(data);
        Self {
            lex,
            cur: StartOfFile,
            _reg: reg,
            root: true,
        }
    }

    #[must_use]
    pub fn next_token(&mut self) -> Token {
        self.advance();
        self.cur
    }

    pub fn advance(&mut self) {
        self.cur = self.lex.next().unwrap_or(Ok(EndOfFile)).unwrap_or(Error);
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
            return unexpected(kind, self.cur, self.lex.remainder());
        }
        Ok(self.next_token())
    }

    fn expect_curr(&self, kind: Token) -> Result<()> {
        if self.cur != kind {
            return unexpected(kind, self.cur, self.lex.remainder());
        }
        Ok(())
    }

    /// If current token == kind, then advance and return true, otherwise return false
    pub fn try_consume(&mut self, kind: Token) -> bool {
        if self.cur == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn string<F: FnMut(&'buf str) -> Result<()>>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf();
            f(&buf[1 .. buf.len() - 1])?;
            self.advance();
        }
        Ok(())
    }

    pub fn bytes<F: FnMut(&'buf [u8]) -> Result<()>>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf().as_bytes();
            f(&buf[1 .. buf.len() - 1])?;
            self.advance();
        }
        Ok(())
    }

    pub fn bool(&mut self) -> Result<bool> {
        let out = match self.token_and_span() {
            (Ident, "t" | "T" | "True" | "true") => Ok(true),
            (Ident, "f" | "F" | "False" | "false") => Ok(false),
            (DecLit, x) => Ok(u64::from_str(x).unwrap_or(0) != 0),
            (kind, _) => unexpected(Ident, kind, self.lex.remainder()),
        }?;
        let _ = self.next_token();
        Ok(out)
    }

    pub fn u64(&mut self) -> Result<u64> {
        let out = match self.token_and_span() {
            (HexLit, h) => u64::from_str_radix(&h[2 ..], 16)?,
            (DecLit, h) => u64::from_str_radix(h, 10)?,
            (OctLit, h) => u64::from_str_radix(h, 8)?,
            (tok, _) => return unexpected(DecLit, tok, self.lex.remainder()),
        };
        let _ = self.next_token();
        Ok(out)
    }

    pub fn u32(&mut self) -> Result<u32> {
        let out = match self.token_and_span() {
            (HexLit, h) => u32::from_str_radix(&h[2 ..], 16)?,
            (DecLit, h) => u32::from_str_radix(h, 10)?,
            (OctLit, h) => u32::from_str_radix(h, 8)?,
            (tok, _) => return unexpected(DecLit, tok, self.lex.remainder()),
        };
        let _ = self.next_token();
        Ok(out)
    }

    pub fn i64(&mut self) -> Result<i64> {
        // TODO: There is a bug here, fix
        let neg = self.try_consume(Minus);
        let v = TryInto::<i64>::try_into(self.u64()?)?;
        Ok(if neg { -v } else { v })
    }

    pub fn i32(&mut self) -> Result<i32> {
        // TODO: There is a bug here. Fix
        let neg = self.try_consume(Minus);
        let v = TryInto::<i32>::try_into(self.u32()?)?;
        Ok(if neg { -v } else { v })
    }

    pub fn f64(&mut self) -> Result<f64> {
        let neg = if self.try_consume(Minus) { -1.0 } else { 1.0 };
        let res = neg
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
                (k, _) => return unexpected(FltLit, k, self.lex.remainder()),
            };

        self.advance();
        Ok(res)
    }

    pub fn f32(&mut self) -> Result<f64> {
        Ok(self.f64()? as _)
    }

    pub(crate) fn message_fields(&mut self, p: &mut dyn TextProto<'buf>) -> Result<()> {
        let allow_eof = self.root;
        self.root = false;
        if self.cur == Colon {
            self.advance();
        }

        let is_angle = match self.cur {
            LBrace => false,
            LAngle => true,
            StartOfFile if allow_eof => true,
            tok => return unexpected(LBrace, tok, self.lex.remainder()),
        };

        self.advance();

        loop {
            match self.cur {
                Ident | ExtIdent => {
                    p.merge_field(self)?;
                    if self.cur == Comma || self.cur == Semi {
                        self.advance();
                    }
                }
                RBrace => {
                    return if !is_angle {
                        self.advance();
                        Ok(())
                    } else {
                        unexpected(RAngle, RBrace, self.lex.remainder())
                    };
                }
                RAngle => {
                    return if is_angle {
                        self.advance();
                        Ok(())
                    } else {
                        unexpected(RBrace, RAngle, self.lex.remainder())
                    };
                }
                EndOfFile if allow_eof => return Ok(()),
                other => return unexpected(Ident, other, self.lex.remainder()),
            }
        }
    }
}

#[derive(Debug)]
pub struct OutputStream<'r> {
    pub reg: &'r Registry,
    pub(crate) buf: String,
    pad: usize,
}

impl<'r> OutputStream<'r> {
    pub fn new(reg: &'r Registry) -> Self {
        Self {
            reg,
            buf: "".to_string(),
            pad: 0,
        }
    }

    pub fn ln(&mut self) {
        self.buf.push('\n');
        for _ in 0 .. self.pad {
            self.buf.push(' ');
        }
    }

    #[inline(always)]
    pub fn enter(&mut self) {
        self.pad += 4;
    }
    #[inline(always)]
    pub fn exit(&mut self) {
        self.pad -= 4;
    }
    #[inline(always)]
    pub fn push(&mut self, i: &str) {
        self.buf.push_str(i);
    }

    #[inline(always)]
    pub fn space(&mut self) {
        self.buf.push(' ');
    }
    #[inline(always)]
    pub fn disp(&mut self, d: &dyn Display) {
        write!(self.buf, "{d}").unwrap();
    }
    #[inline(always)]
    pub fn ident(&mut self, id: &str) {
        self.buf.push_str(id);
    }
    #[inline(always)]
    pub fn lbracket(&mut self) {
        self.buf.push('[');
    }
    #[inline(always)]
    pub fn rbracket(&mut self) {
        self.buf.push(']');
    }
    #[inline(always)]
    pub fn lbrace(&mut self) {
        self.buf.push('{');
    }
    #[inline(always)]
    pub fn rbrace(&mut self) {
        self.buf.push('}');
    }
    #[inline(always)]
    pub fn colon(&mut self) {
        self.buf.push(':');
    }

    pub fn bytes(&mut self, b: &[u8]) {
        self.buf.push('"');
        escape_bytes_to(b, &mut self.buf);
        self.buf.push('"');
    }

    pub fn string(&mut self, s: &str) {
        self.buf.push('"');
        escape_bytes_to(s.as_bytes(), &mut self.buf);
        self.buf.push('"');
    }
}
