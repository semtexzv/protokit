use core::iter::Peekable;

pub fn escape_bytes_to(bytes: &[u8], buf: &mut String) {
    for c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'\'' => buf.push_str("\\\'"),
            b'"' => buf.push_str("\\\""),
            b'\\' => buf.push_str("\\\\"),

            7 => buf.push_str("\\a"),
            8 => buf.push_str("\\b"),

            11 => buf.push_str("\\v"),
            12 => buf.push_str("\\f"),

            63 => buf.push_str("\\?"),

            b'\x20'..=b'\x7e' => buf.push(*c as char),
            _ => {
                buf.push('\\');
                buf.push((b'0' + (c >> 6)) as char);
                buf.push((b'0' + ((c >> 3) & 7)) as char);
                buf.push((b'0' + (c & 7)) as char);
            }
        }
    }
}

pub fn unescape<I: Iterator<Item = u8>>(it: I) -> Unescape<I> {
    Unescape {
        it: it.peekable(),
        buf: [0; 4],
        buf_len: 0,
        buf_idx: 0,
        pending_hex: false,
    }
}

pub struct Unescape<I: Iterator<Item = u8>> {
    it: Peekable<I>,
    buf: [u8; 4],
    buf_len: usize,
    buf_idx: usize,
    // hleft is no longer needed for \u/\U as we handle them immediately,
    // but needed for \x (hex escape) which is variable length?
    // \x is usually 2 hex digits in C++, but protobuf might allow variable?
    // Protobuf spec says \x followed by 2 hex digits.
    // The previous implementation handled variable length hex?
    // "first @ b'0' ..= b'7'" handled octal.
    // \x handled by "self.hleft = 1; continue;" -> next loop reads 2 chars?
    // Let's keep hleft for \x if needed, or refactor.
    // Previous \x logic: hleft=1.
    // next(): if hleft > 0: read 2 chars, return byte.
    // So \x reads 2 chars.
    pending_hex: bool,
}

fn fhex(c: u8) -> u8 {
    (c & 0x0F) + 9 - ((c & 0x10) >> 1) - ((c & 0x10) >> 4)
}

#[test]
fn test_hex() {
    assert_eq!(fhex(b'a'), 0xa);
    assert_eq!(fhex(b'A'), 0xa);
}

impl<I: Iterator<Item = u8>> Iterator for Unescape<I> {
    type Item = crate::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf_idx < self.buf_len {
            let b = self.buf[self.buf_idx];
            self.buf_idx += 1;
            return Some(Ok(b));
        }

        if self.pending_hex {
            self.pending_hex = false;
            let a = self.it.next()?;
            let b = self.it.next()?;
            return Some(Ok(fhex(a) << 4 | fhex(b)));
        }

        let bslash = self.it.next()?;
        if bslash != b'\\' {
            return Some(Ok(bslash));
        }

        let next_char = match self.it.next()? {
            b'a' => 7,
            b'b' => 8,
            b't' => 9,
            b'n' => 10,
            b'v' => 11,
            b'f' => 12,
            b'r' => 13,
            b'"' => 34,
            b'\'' => 39,
            b'?' => 63,
            b'\\' => 92,
            b'x' => {
                self.pending_hex = true;
                return self.next();
            }
            b'u' => {
                // Read 4 hex digits
                let mut v = 0u32;
                for _ in 0..4 {
                    let c = match self.it.next() {
                        Some(c) => c,
                        None => return Some(Err(crate::Error::InvalidEscape)),
                    };
                    v = (v << 4) | (fhex(c) as u32);
                }
                if let Some(c) = char::from_u32(v) {
                    let s = c.encode_utf8(&mut self.buf);
                    self.buf_len = s.len();
                    self.buf_idx = 0;
                    return self.next();
                } else {
                    return Some(Err(crate::Error::InvalidEscape));
                }
            }
            b'U' => {
                // Read 8 hex digits
                let mut v = 0u32;
                for _ in 0..8 {
                    let c = match self.it.next() {
                        Some(c) => c,
                        None => return Some(Err(crate::Error::InvalidEscape)),
                    };
                    v = (v << 4) | (fhex(c) as u32);
                }
                if let Some(c) = char::from_u32(v) {
                    let s = c.encode_utf8(&mut self.buf);
                    self.buf_len = s.len();
                    self.buf_idx = 0;
                    return self.next();
                } else {
                    return Some(Err(crate::Error::InvalidEscape));
                }
            }
            first @ b'0'..=b'7' => {
                let mut value = first - b'0';
                for _ in 0..2 {
                    match self.it.peek() {
                        Some(x @ b'0'..=b'7') => {
                            value = value << 3 | (*x - b'0');
                            let _ = self.it.next();
                        }
                        _ => break,
                    }
                }
                value
            }
            _ => return Some(Err(crate::Error::InvalidEscape)),
        };
        Some(Ok(next_char))
    }
}
