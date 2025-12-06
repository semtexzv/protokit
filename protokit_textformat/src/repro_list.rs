use crate::decode;

#[derive(Debug, Default, PartialEq)]
struct TestStringList {
    s: Vec<String>,
}

impl binformat::ProtoName for TestStringList {
    fn qualified_name(&self) -> &'static str {
        "TestStringList"
    }
}

impl<'buf> binformat::BinProto<'buf> for TestStringList {
    fn merge_field(&mut self, _tag: u32, _stream: &mut binformat::InputStream<'buf>) -> binformat::Result<()> {
        Ok(())
    }
    fn size(&self, _stack: &mut binformat::SizeStack) -> usize {
        0
    }
    fn encode(&self, _stream: &mut binformat::OutputStream) {}
}

impl<'buf> crate::TextProto<'buf> for TestStringList {
    fn decode(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        while stream.token() != crate::Token::EndOfFile {
            self.merge_field(stream)?;
        }
        Ok(())
    }

    fn merge_field(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        if stream.token() == crate::Token::Ident && stream.buf() == "s" {
            stream.advance();
            if stream.try_consume(crate::Token::Colon) {
                // ok
            }
            crate::merge_repeated(&mut self.s, stream)?;
        } else {
            // unknown field or end
            if stream.token() != crate::Token::EndOfFile {
                stream.advance();
            }
        }
        Ok(())
    }
    fn encode(&self, _stream: &mut crate::OutputStream) {}
}

#[test]
fn test_string_list_space_separated() {
    let input = "s: [\"a\" \"b\"]";
    let reg = crate::reflect::Registry::default();
    let msg: TestStringList = decode(input, &reg).unwrap();
    assert_eq!(msg.s, vec!["ab".to_string()]);
}

#[test]
fn test_string_concatenation() {
    let input = "s: [\"a\" \"b\", \"c\"]";
    let reg = crate::reflect::Registry::default();
    // Expect: ["ab", "c"]
    let msg: TestStringList = decode(input, &reg).unwrap();
    assert_eq!(msg.s, vec!["ab".to_string(), "c".to_string()]);
}

#[derive(Debug, Default, PartialEq)]
struct TestIntList {
    i: Vec<i32>,
}

impl binformat::ProtoName for TestIntList {
    fn qualified_name(&self) -> &'static str {
        "TestIntList"
    }
}

impl<'buf> binformat::BinProto<'buf> for TestIntList {
    fn merge_field(&mut self, _tag: u32, _stream: &mut binformat::InputStream<'buf>) -> binformat::Result<()> {
        Ok(())
    }
    fn size(&self, _stack: &mut binformat::SizeStack) -> usize {
        0
    }
    fn encode(&self, _stream: &mut binformat::OutputStream) {}
}

impl<'buf> crate::TextProto<'buf> for TestIntList {
    fn decode(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        while stream.token() != crate::Token::EndOfFile {
            self.merge_field(stream)?;
        }
        Ok(())
    }
    fn merge_field(&mut self, stream: &mut crate::InputStream<'buf>) -> crate::Result<()> {
        if stream.token() == crate::Token::Ident && stream.buf() == "i" {
            stream.advance();
            if stream.try_consume(crate::Token::Colon) {}
            crate::merge_repeated(&mut self.i, stream)?;
        } else {
            if stream.token() != crate::Token::EndOfFile {
                stream.advance();
            }
        }
        Ok(())
    }
    fn encode(&self, _stream: &mut crate::OutputStream) {}
}

#[test]
fn test_int_list_space_separated() {
    let input = "i: [1 2]";
    let reg = crate::reflect::Registry::default();
    let msg: TestIntList = decode(input, &reg).unwrap();
    assert_eq!(msg.i, vec![1, 2]);
}

#[test]
fn test_int_list_comma() {
    let input = "i: [1, 2]";
    let reg = crate::reflect::Registry::default();
    let msg: TestIntList = decode(input, &reg).unwrap();
    assert_eq!(msg.i, vec![1, 2]);
}

#[test]
fn test_int_list_comment() {
    let input = "i: [1 /* comment */ 2]";
    let reg = crate::reflect::Registry::default();
    let msg: TestIntList = decode(input, &reg).unwrap();
    assert_eq!(msg.i, vec![1, 2]);
}

#[test]
fn test_int_list_mixed() {
    let input = "i: [1, 2; 3 4]";
    let reg = crate::reflect::Registry::default();
    let msg: TestIntList = decode(input, &reg).unwrap();
    assert_eq!(msg.i, vec![1, 2, 3, 4]);
}
