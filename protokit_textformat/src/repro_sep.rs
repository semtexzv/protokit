#[cfg(test)]
mod tests {
    use crate::reflect::Registry;
    use crate::stream::InputStream;
    use crate::Token;
    use crate::*;

    #[derive(Default)]
    struct MockRepeated {
        vals: Vec<i32>,
        bools: Vec<bool>,
    }

    impl<'buf> TextProto<'buf> for MockRepeated {
        fn merge_field(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            match stream.parse_key()?.as_ref() {
                "vals" => merge_repeated(&mut self.vals, stream),
                "bools" => merge_repeated(&mut self.bools, stream),
                "1" => {
                    // Simulate integer key 1 mapping to vals
                    merge_repeated(&mut self.vals, stream)
                }
                _ => skip_unknown(stream),
            }
        }
        fn encode(&self, _stream: &mut OutputStream) {}
        fn decode(&mut self, stream: &mut InputStream<'buf>) -> Result<()> {
            stream.message_fields(self)
        }
    }

    impl binformat::ProtoName for MockRepeated {
        fn qualified_name(&self) -> &'static str {
            "MockRepeated"
        }
    }

    #[test]
    fn test_comma_separated_fields() {
        let reg = Registry::default();
        // "field: val, field: val"
        let txt = "vals: 1, vals: 2";
        let mut stream = InputStream::new(txt, &reg);
        let mut msg = MockRepeated::default();
        stream
            .message_fields(&mut msg)
            .expect("Should parse comma separated fields");
        assert_eq!(msg.vals, vec![1, 2]);
    }

    #[test]
    fn test_comma_separated_values() {
        let reg = Registry::default();
        // "field: 1, 2"
        let txt = "vals: 3, 4";
        let mut stream = InputStream::new(txt, &reg);
        let mut msg = MockRepeated::default();
        stream
            .message_fields(&mut msg)
            .expect("Should parse comma separated values");
        assert_eq!(msg.vals, vec![3, 4]);
    }

    #[test]
    fn test_semicolon_separated_fields() {
        let reg = Registry::default();
        // "field: val; field: val"
        let txt = "vals: 5; vals: 6";
        let mut stream = InputStream::new(txt, &reg);
        let mut msg = MockRepeated::default();
        stream
            .message_fields(&mut msg)
            .expect("Should parse semicolon separated fields");
        assert_eq!(msg.vals, vec![5, 6]);
    }

    #[test]
    fn test_integer_key() {
        let reg = Registry::default();
        let txt = "1: 10";
        let mut stream = InputStream::new(txt, &reg);
        let mut msg = MockRepeated::default();
        stream.message_fields(&mut msg).expect("Should parse integer key");
        assert_eq!(msg.vals, vec![10]);
    }

    #[test]
    fn test_bool_separators() {
        let reg = Registry::default();
        let txt = "bools: true, bools: false";
        let mut stream = InputStream::new(txt, &reg);
        let mut msg = MockRepeated::default();
        stream.message_fields(&mut msg).expect("Should parse bool fields");
        assert_eq!(msg.bools, vec![true, false]);

        let txt2 = "bools: true, false";
        let mut stream2 = InputStream::new(txt2, &reg);
        let mut msg2 = MockRepeated::default();
        stream2
            .message_fields(&mut msg2)
            .expect("Should parse comma bool values");
        assert_eq!(msg2.bools, vec![true, false]);
    }
}
