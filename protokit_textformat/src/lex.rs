use logos::Logos;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Logos)]
#[logos(subpattern idt = r"[_a-zA-Z][_0-9a-zA-Z]*")]
#[logos(subpattern dec = r"[1-9][0-9]*")]
#[logos(subpattern oct = r"0[0-9]*")]
#[logos(subpattern dig = r"[0-9]+")]
#[logos(subpattern hex = r"0x[0-9a-fA-F][0-9a-fA-F]?")]
#[logos(subpattern exp = r"[eE][+-]?[0-9]+")]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"#[^\n]+", logos::skip)]
    Comment,

    #[regex("(?&idt)")]
    Ident,

    #[regex(r"\[(?&idt)(\.(?&idt))*\]")]
    ExtIdent,

    #[regex("(?&dec)", priority = 3)]
    DecLit,
    #[regex("(?&hex)", priority = 2)]
    HexLit,
    #[regex("(?&oct)", priority = 2)]
    OctLit,

    #[regex(r"(?&dig)(.(?&dig))?([eE][+-]?(?&dig))?", priority = 1)]
    FltLit,

    #[regex(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#)]
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

    #[token(",")]
    Comma,
    #[token("-")]
    Minus,

    Error,

    StartOfFile,
    EndOfFile,
}
