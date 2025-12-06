#[cfg(test)]
mod tests {
    use crate::reflect::Registry;
    use crate::*;
    use std::fmt::Display;
    use std::str::FromStr;

    // Simulate a derived Enum
    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MyEnum(i32);

    impl Default for MyEnum {
        fn default() -> Self {
            Self(0)
        }
    }

    impl Display for MyEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for MyEnum {
        type Err = Error;
        fn from_str(_s: &str) -> Result<Self> {
            Ok(Self(0)) // Dummy
        }
    }

    impl From<u32> for MyEnum {
        fn from(v: u32) -> Self {
            Self(v as i32)
        }
    }
    impl From<MyEnum> for u32 {
        fn from(v: MyEnum) -> Self {
            v.0 as u32
        }
    }

    impl Enum for MyEnum {}

    impl<'buf> TextField<'buf> for MyEnum {
        fn merge_value(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            // Logic copied from protokit_derive
            match stream.field() {
                "FOO" => {
                    *self = Self(0);
                    stream.advance();
                }
                "BAR" => {
                    *self = Self(1);
                    stream.advance();
                }
                name => {
                    if let Ok(v) = name.parse::<i32>() {
                        *self = Self(v);
                        stream.advance();
                    } else {
                        return crate::unknown(name);
                    }
                }
            }
            Ok(())
        }

        fn emit_value(&self, stream: &mut OutputStream) {
            match self.0 {
                0 => stream.ident("FOO"),
                1 => stream.ident("BAR"),
                v => stream.disp(&v),
            }
        }
    }

    #[test]
    fn test_enum_number_parsing() {
        let reg = Registry::default();
        let txt = "1";
        let mut stream = InputStream::new(txt, &reg);
        stream.advance(); // Prime first token
        let mut e = MyEnum::default();
        e.merge_value(&mut stream).expect("Should parse '1'");
        assert_eq!(e.0, 1);
    }

    #[test]
    fn test_float_zero_parsing() {
        let reg = Registry::default();
        let txt = "0";
        let mut stream = InputStream::new(txt, &reg);
        stream.advance();
        let v = stream.f64().expect("Should parse '0' as float");
        assert_eq!(v, 0.0);
    }

    #[test]
    fn test_float_neg_zero_parsing() {
        let reg = Registry::default();
        let txt = "-0";
        let mut stream = InputStream::new(txt, &reg);
        stream.advance();
        let v = stream.f64().expect("Should parse '-0' as float");
        assert_eq!(v, 0.0);
        assert!(v.is_sign_negative());
    }

    // Simulate Message with enum field to test full merge_field flow
    #[derive(Default)]
    struct MyMsg {
        e: MyEnum,
    }

    impl<'buf> TextProto<'buf> for MyMsg {
        fn merge_field(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            match stream.field() {
                "e" => {
                    // merge_single
                    self.e.merge_text(stream)
                }
                _ => crate::skip_unknown(stream),
            }
        }
        fn encode(&self, _stream: &mut OutputStream) {}
        fn decode(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            stream.message_fields(self)
        }
    }
    impl binformat::ProtoName for MyMsg {
        fn qualified_name(&self) -> &'static str {
            "MyMsg"
        }
    }

    #[test]
    fn test_msg_enum_parsing() {
        let reg = Registry::default();
        let txt = "e: 1";
        let mut stream = InputStream::new(txt, &reg);
        // message_fields advances internally?
        // No, message_fields expects stream to be at start of message?
        // InputStream::new sets root=true.
        // stream.message_fields -> if root=true, advances.
        let mut msg = MyMsg::default();
        stream.message_fields(&mut msg).expect("Should parse msg");
        assert_eq!(msg.e.0, 1);
    }
}
