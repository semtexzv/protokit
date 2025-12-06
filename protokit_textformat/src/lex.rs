use logos::Logos;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Logos)]
#[logos(subpattern idt = r"[_a-zA-Z][_0-9a-zA-Z]*")]
#[logos(subpattern dec = r"[1-9][0-9]*")]
#[logos(subpattern oct = r"0[0-9]*")]
#[logos(subpattern dig = r"[0-9]+")]
#[logos(subpattern hex = r"0x[0-9a-fA-F]+")]
#[logos(subpattern exp = r"[eE][+-]?[0-9]+")]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"#[^\n]*", logos::skip)]
    #[token("//", line_comment)]
    #[token("/*", block_comment)]
    Comment,

    #[regex("(?&idt)")]
    Ident,

    #[regex("(?&dec)", priority = 3)]
    DecLit,
    #[regex("(?&hex)", priority = 2)]
    HexLit,
    #[regex("(?&oct)", priority = 2)]
    OctLit,

    #[regex(r"((?&dig)(\.(?&dig)?)?|\.(?&dig))([eE][+-]?(?&dig))?[fF]?", priority = 1)]
    FltLit,

    #[regex(r#""([^"\\\n]|\\.)*"|'([^'\\\n]|\\.)*'"#)]
    StrLit,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,

    #[token(";")]
    Semi,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("/")]
    Slash,

    #[token(",")]
    Comma,
    #[token("-")]
    Minus,

    Error,

    StartOfFile,
    EndOfFile,
}

fn line_comment(lex: &mut logos::Lexer<Token>) -> logos::Skip {
    if let Some(pos) = lex.remainder().find('\n') {
        lex.bump(pos + 1);
    } else {
        lex.bump(lex.remainder().len());
    }
    logos::Skip
}

fn block_comment(lex: &mut logos::Lexer<Token>) -> logos::Skip {
    if let Some(pos) = lex.remainder().find("*/") {
        lex.bump(pos + 2);
    } else {
        lex.bump(lex.remainder().len());
    }
    logos::Skip
}

#[cfg(test)]
mod test {
    use logos::Lexer;

    use crate::lex::Token;

    #[test]
    fn test_lex_any() {
        let txt = "[a/a.a] { }";
        let mut lex = Lexer::<Token>::new(txt);
        assert_eq!(lex.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lex.slice(), "[");
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Slash)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
    }

    #[test]
    fn test_lex_comma() {
        let txt = ",true]";
        let mut lex = Lexer::<Token>::new(txt);
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.slice(), "true");
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
    }
    #[test]
    fn test_lex_list() {
        let txt = "[true, true]";
        let mut lex = Lexer::<Token>::new(txt);
        assert_eq!(lex.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident)));
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
    }
    #[test]
    fn test_lex_comment() {
        let txt = "1 /* comment */ 2";
        let mut lex = Lexer::<Token>::new(txt);
        assert_eq!(lex.next(), Some(Ok(Token::DecLit)));
        assert_eq!(lex.slice(), "1");
        // Comments are skipped
        assert_eq!(lex.next(), Some(Ok(Token::DecLit)));
        assert_eq!(lex.slice(), "2");
        assert_eq!(lex.next(), None);
    }
}
