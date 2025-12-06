#[cfg(test)]
mod tests {
    use crate::reflect::Registry;
    use crate::stream::InputStream;
    use crate::Token;

    #[test]
    fn test_extension_key_parsing() {
        let reg = Registry::default();
        let txt = "[some.ext.field]: 123";
        let mut stream = InputStream::new(txt, &reg);

        // InputStream::new doesn't advance automatically to first token?
        // Let's check constructor. It calls Lexer::new, sets cur=StartOfFile.
        // next_token() or advance() needed?
        // merge_field loop usually calls advance.

        // Manually drive pars_key
        stream.advance(); // Cur should be LBracket
        assert_eq!(stream.token(), Token::LBracket);

        // But parse_key expects to start AT the key tokens?
        // parse_key implementation:
        // match self.cur { LBracket => ... }
        // So yes, we need to be at LBracket.

        let key = stream.parse_key().expect("Should parse extension key");
        assert_eq!(key, "some.ext.field");

        // Should be at Colon now
        assert_eq!(stream.token(), Token::Colon);
    }

    #[test]
    fn test_simple_key_parsing() {
        let reg = Registry::default();
        let txt = "simple_field: 123";
        let mut stream = InputStream::new(txt, &reg);
        stream.advance();
        assert_eq!(stream.token(), Token::Ident);

        let key = stream.parse_key().expect("Should parse simple key");
        assert_eq!(key, "simple_field");
        assert_eq!(stream.token(), Token::Colon);
    }
}
