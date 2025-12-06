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
    pub reg: &'buf Registry,
    // Whether we are parsing root message, or we're expecting more braces
    root: bool,
    override_slice: Option<&'buf str>,
    depth: usize,
}

impl<'buf> InputStream<'buf> {
    pub fn new(data: &'buf str, reg: &'buf Registry) -> Self {
        let lex = Lexer::new(data);
        Self {
            lex,
            cur: StartOfFile,
            reg,
            root: true,
            override_slice: None,
            depth: 0,
        }
    }

    #[must_use]
    pub fn next_token(&mut self) -> Token {
        self.advance();
        self.cur
    }

    pub fn token(&self) -> Token {
        self.cur
    }

    pub fn advance(&mut self) {
        self.override_slice = None;
        let n = self.lex.next();
        self.cur = n.unwrap_or(Ok(EndOfFile)).unwrap_or(Error);
    }

    pub fn buf(&self) -> &'buf str {
        self.lex.slice()
    }

    pub fn field(&self) -> &'buf str {
        if let Some(s) = self.override_slice {
            s
        } else {
            self.lex.slice()
        }
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

    pub fn try_consume(&mut self, kind: Token) -> bool {
        if self.cur == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Parses a field key, which can be a simple identifier or an extension name in brackets.
    /// Returns the string representation of the key.
    pub fn parse_key(&mut self) -> Result<std::borrow::Cow<'buf, str>> {
        match self.cur {
            Ident | DecLit => {
                let s = std::borrow::Cow::Borrowed(self.lex.slice());
                self.advance();
                Ok(s)
            }
            LBracket => {
                self.advance();
                let start = self.lex.span().start;
                // We need to handle [ type.name ]
                // It seems the old lexer allowed [Ident . Ident . Ident]
                // Let's replicate this loop.
                // First ident
                if self.cur != Ident {
                    return unexpected(Ident, self.cur, self.lex.remainder());
                }
                let mut end = self.lex.span().end;
                self.advance();

                loop {
                    match self.cur {
                        Dot | Slash => {
                            self.advance();
                            if self.cur != Ident {
                                return unexpected(Ident, self.cur, self.lex.remainder());
                            }
                            end = self.lex.span().end;
                            self.advance();
                        }
                        RBracket => {
                            self.advance();
                            break;
                        }
                        other => return unexpected(RBracket, other, self.lex.remainder()),
                    }
                }

                // We construct the string from the source slice.
                // Note: The lexer state might have advanced past multiple tokens, so self.lex.slice() currently points to RBracket or one past last Ident.
                // But we need the range [start..end].
                // However, `self.lex` in `advance()` moves forward. We can't easily get the full span from `start` to `end` via `self.lex.source()`.
                // Fortunately, `InputStream` has `lex`, and logos lexer gives access to `source()`.
                Ok(std::borrow::Cow::Borrowed(&self.lex.source()[start..end]))
            }
            other => unexpected(Ident, other, self.lex.remainder()),
        }
    }

    pub fn peek_key(&self) -> Result<std::borrow::Cow<'buf, str>> {
        let probe = self.lex.clone();
        let cur = self.cur;

        // We need a temporary stream to run parse_key
        let mut temp = Self {
            lex: probe,
            cur,
            reg: self.reg,
            root: self.root,
            override_slice: self.override_slice,
            depth: self.depth,
        };

        temp.parse_key()
    }

    pub fn lookahead_is_field(&self) -> bool {
        let mut probe = self.lex.clone();
        let next = probe.next().unwrap_or(Ok(EndOfFile)).unwrap_or(Error);
        match next {
            Ident | DecLit => {
                let after = probe.next().unwrap_or(Ok(EndOfFile)).unwrap_or(Error);
                matches!(after, Colon | LBrace | LAngle)
            }
            LBracket => {
                // Heuristic: check if it looks like an extension?
                // Or simply assume if it starts with [, it's a field in this context?
                // But [ could be start of value array.
                // If we are looking for a field key, [ usually means extension.
                // But if we are in a list, [ means value.
                // This function is used in `merge_repeated` to check if we hit a new field.
                // If we see `[`, and it's followed by `Ident`, it *could* be extension.
                // If it is followed by value chars, it's value.
                // Let's assume consistent formatting:
                // [extension_name]: value
                // [value, value]

                // If next is LBracket.
                // Check if followed by Ident then Dot/Slash/RBracket?
                if let Some(Ok(Ident)) = probe.next() {
                    // It's [Ident ...
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn string<F: FnMut(&'buf str) -> Result<()>>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf();
            f(&buf[1..buf.len() - 1])?;
            self.advance();
        }
        Ok(())
    }

    pub fn bytes<F: FnMut(&'buf [u8]) -> Result<()>>(&mut self, mut f: F) -> Result<()> {
        self.expect_curr(StrLit)?;
        while self.cur == StrLit {
            let buf = self.buf().as_bytes();
            f(&buf[1..buf.len() - 1])?;
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
            (HexLit, h) => u64::from_str_radix(&h[2..], 16)?,
            (DecLit, h) => u64::from_str_radix(h, 10)?,
            (OctLit, h) => u64::from_str_radix(h, 8)?,
            (tok, _) => return unexpected(DecLit, tok, self.lex.remainder()),
        };
        let _ = self.next_token();
        Ok(out)
    }

    pub fn u32(&mut self) -> Result<u32> {
        let out = match self.token_and_span() {
            (HexLit, h) => u32::from_str_radix(&h[2..], 16)?,
            (DecLit, h) => u32::from_str_radix(h, 10)?,
            (OctLit, h) => u32::from_str_radix(h, 8)?,
            (tok, _) => return unexpected(DecLit, tok, self.lex.remainder()),
        };
        let _ = self.next_token();
        Ok(out)
    }

    pub fn i64(&mut self) -> Result<i64> {
        let neg = self.try_consume(Minus);
        let v = self.u64()?;
        if neg {
            if v > i64::MAX as u64 + 1 {
                return Err(i64::try_from(v).unwrap_err().into());
            }
            Ok((v as i64).wrapping_neg())
        } else {
            Ok(TryInto::<i64>::try_into(v)?)
        }
    }

    pub fn i32(&mut self) -> Result<i32> {
        let neg = self.try_consume(Minus);
        let v = self.u32()?;
        if neg {
            if v > i32::MAX as u32 + 1 {
                return Err(i32::try_from(v).unwrap_err().into());
            }
            Ok((v as i32).wrapping_neg())
        } else {
            Ok(TryInto::<i32>::try_into(v)?)
        }
    }

    pub fn f64(&mut self) -> Result<f64> {
        let neg = if self.try_consume(Minus) { -1.0 } else { 1.0 };
        let res = neg
            * match self.token_and_span() {
                (FltLit | DecLit, s) => {
                    let s = s.trim_end_matches(['f', 'F']);
                    f64::from_str(s).unwrap()
                }
                (OctLit, "0") => 0.0,
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
        let v = self.f64()?;
        Ok(v)
    }

    pub fn message_fields(&mut self, p: &mut dyn TextProto<'buf>) -> Result<()> {
        if self.depth >= 100 {
            return Err(crate::Error::RecursionLimitExceeded);
        }
        self.depth += 1;
        let res = self.message_fields_inner(p);
        self.depth -= 1;
        res
    }

    fn message_fields_inner(&mut self, p: &mut dyn TextProto<'buf>) -> Result<()> {
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
                Ident | DecLit | LBracket => {
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
    pub buf: String,
    pad: usize,
    pub print_unknown_fields: bool,
}

impl<'r> OutputStream<'r> {
    pub fn new(reg: &'r Registry) -> Self {
        Self {
            reg,
            buf: "".to_string(),
            pad: 0,
            print_unknown_fields: true,
        }
    }

    pub fn ln(&mut self) {
        self.buf.push('\n');
        for _ in 0..self.pad {
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

pub struct RecursionGuard<'a, 'buf> {
    stream: &'a mut InputStream<'buf>,
}

impl<'a, 'buf> Drop for RecursionGuard<'a, 'buf> {
    fn drop(&mut self) {
        self.stream.depth -= 1;
    }
}
