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

pub fn unescape<I: Iterator<Item=u8>>(it: I) -> Unescape<I> {
    Unescape {
        it: it.peekable(),
        hleft: 0,
    }
}

pub struct Unescape<I: Iterator<Item=u8>> {
    it: Peekable<I>,
    hleft: u8,
}

fn fhex(c: u8) -> u8 {
    (c & 0x0F) + 9 - ((c & 0x10) >> 1) - ((c & 0x10) >> 4)
}

#[test]
fn test_hex() {
    assert_eq!(fhex(b'a'), 0xa);
    assert_eq!(fhex(b'A'), 0xa);
}

impl<I: Iterator<Item=u8>> Iterator for Unescape<I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.hleft > 0 {
                let a = self.it.next()?;
                let b = self.it.next()?;
                self.hleft -= 1;
                return Some(fhex(a) << 4 | fhex(b));
            }

            let bslash = self.it.next()?;
            if bslash != b'\\' {
                return Some(bslash);
            }
            let next = match self.it.next()? {
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
                    self.hleft = 1;
                    continue;
                }
                b'u' => {
                    self.hleft = 2;
                    continue;
                }
                b'U' => {
                    let _ = self.it.next();
                    let _ = self.it.next();
                    self.hleft = 3;
                    continue;
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
                _ => panic!("Invalid escape"),
            };
            return Some(next);
        }
    }
}
