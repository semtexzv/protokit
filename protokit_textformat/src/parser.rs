use lex_core::{parse_with_options, NumberFormatBuilder};
use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag, take_till, take_while, take_while_m_n};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, none_of, one_of};
use nom::combinator::{cut, map, map_res, opt, recognize, value};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::{AsChar, IResult, Offset, Slice};

use super::ast::*;

fn comment(i: Span) -> IResult<Span, Span> {
    preceded(char('#'), is_not("\n\r"))(i)
}

fn ident(i: Span) -> IResult<Span, Span> {
    recognize(tuple((alpha1, many0(alt((alphanumeric1, tag("_")))))))(i)
}

fn compound_ident(istart: Span) -> IResult<Span, Span> {
    let (i, _) = ident(istart)?;
    let (iend, _) = many0(preceded(char('.'), ident))(i)?;
    let pos = istart.offset(iend);
    Ok((iend, istart.slice(.. pos)))
}

fn dec_lit(i: Span) -> IResult<Span, Span> {
    let (i, n) = take_while(|c: char| c.is_dec_digit())(i)?;

    Ok((i, n))
}

fn oct_lit(i: Span) -> IResult<Span, Span> {
    preceded(char('0'), take_while(|c: char| c.is_oct_digit()))(i)
}

fn hex_lit(i: Span) -> IResult<Span, Span> {
    preceded(alt((tag("0x"), tag("0X"))), take_while(|c: char| c.is_hex_digit()))(i)
}

fn float_lit(i: Span) -> IResult<Span, f64> {
    fn exp(i: Span) -> IResult<Span, i32> {
        let (i, _) = one_of("eE")(i)?;
        let (i, sig) = opt(one_of("+-"))(i)?;

        let sig = sig.map(|v| if v == '-' { -1 } else { 1 }).unwrap_or(1);

        let (i, exp): (_, i32) = map_res(take_while(|c: char| c.is_dec_digit()), |v: &str| {
            lex_core::parse(v.as_bytes())
        })(i)?;

        Ok((i, exp * sig))
    }

    fn fract(i: &str) -> IResult<&str, &str> {
        let (i, _) = char('.')(i)?;
        take_while(|c: char| c.is_dec_digit())(i)
    }

    fn suffix(i: &str) -> IResult<&str, ()> {
        value((), one_of("fF"))(i)
    }

    alt((
        // Try fract first
        map(tuple((fract, exp, opt(suffix))), |(f, e, _)| {
            lex_float::parse_float(b"".iter(), f.as_bytes().iter(), e)
        }),
        map(tuple((dec_lit, fract, exp, opt(suffix))), |(d, f, e, _)| {
            lex_float::parse_float(d.as_bytes().iter(), f.as_bytes().iter(), e)
        }),
        map(tuple((dec_lit, exp, suffix)), |(d, e, _)| {
            lex_float::parse_float(d.as_bytes().iter(), b"".iter(), e)
        }),
        map(tuple((dec_lit, suffix)), |(d, _)| {
            lex_float::parse_float(d.as_bytes().iter(), b"".iter(), 1)
        }),
    ))(i)
}

#[test]
fn test_float_lit() {
    assert_eq!(float_lit("0.10e4"), Ok(("", 0.10e4)));
}

fn escape_contents(i: &str) -> IResult<&str, &str> {
    let char_escape = recognize(one_of("abfnrtv\\'\"?"));
    let oct_escape = take_while_m_n(3, 3, |c: char| c.is_oct_digit());
    let hex_escape = recognize(preceded(one_of("xX"), take_while_m_n(2, 2, |c: char| c.is_hex_digit())));
    let uni_escape = recognize(tuple((char('u'), take_while_m_n(4, 4, |c: char| c.is_hex_digit()))));
    let uni2_escape = recognize(tuple((tag("U000"), take_while_m_n(3, 3, |c: char| c.is_hex_digit()))));
    let uni3_escape = recognize(tuple((tag("U0010"), take_while_m_n(4, 4, |c: char| c.is_hex_digit()))));

    alt((
        char_escape,
        oct_escape,
        hex_escape,
        uni_escape,
        uni2_escape,
        uni3_escape,
    ))(i)
}

fn str_char(i: &str) -> IResult<&str, char> {
    none_of("\\\0\n\"")(i)
}

fn str_escaped(i: Span) -> IResult<Span, Span> {
    escaped(str_char, '\\', escape_contents)(i)
}

fn str_lit(i: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((
        delimited(char('"'), str_escaped, char('"')),
        delimited(char('\''), str_escaped, char('\'')),
    )))(i)
}

fn ws<'a, F: 'a, O>(mut inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O>,
{
    move |i: Span| {
        let (i, _) = many0(alt((comment, multispace1)))(i)?;
        inner(i)
    }
}

pub fn strdec(i: Span) -> Result<i128, lex_core::Error> {
    // dbg!(&i);
    parse_with_options::<_, { NumberFormatBuilder::decimal() }>(
        i.as_bytes(),
        &lex_core::parse_integer_options::STANDARD,
    )
}

pub fn stroct(i: Span) -> Result<i128, lex_core::Error> {
    // dbg!(&i);
    parse_with_options::<_, { NumberFormatBuilder::octal() }>(i.as_bytes(), &lex_core::parse_integer_options::STANDARD)
}

pub fn strhex(i: Span) -> Result<i128, lex_core::Error> {
    // dbg!(&i);
    parse_with_options::<_, { NumberFormatBuilder::hexadecimal() }>(
        i.as_bytes(),
        &lex_core::parse_integer_options::STANDARD,
    )
}

fn literal(i: Span) -> IResult<Span, Literal> {
    alt((
        map(ident, Literal::Identifier),
        map(str_lit, Literal::String),
        map(float_lit, Literal::Float),
        map(preceded(char('-'), ws(float_lit)), |v| Literal::Float(-v)),
        map_res(dec_lit, |v| Ok::<_, lex_core::Error>(Literal::Int(strdec(v)?))),
        map_res(oct_lit, |v| Ok::<_, lex_core::Error>(Literal::Int(stroct(v)?))),
        map_res(hex_lit, |v| Ok::<_, lex_core::Error>(Literal::Int(strhex(v)?))),
        map_res(preceded(char('-'), ws(dec_lit)), |v| {
            Ok::<_, lex_core::Error>(Literal::Int(-strdec(v)?))
        }),
        map_res(preceded(char('-'), ws(oct_lit)), |v| {
            Ok::<_, lex_core::Error>(Literal::Int(-stroct(v)?))
        }),
        map_res(preceded(char('-'), ws(hex_lit)), |v| {
            Ok::<_, lex_core::Error>(Literal::Int(-strhex(v)?))
        }),
        map(tuple((tag("-"), ws(ident))), |v| Literal::SignedIdentifier(v.0, v.1)),
    ))(i)
}

#[test]
fn test_literal() {
    assert_eq!(literal("0"), Ok(("", Literal::Int(0))));
    assert_eq!(literal("0 "), Ok((" ", Literal::Int(0))));
    assert_eq!(literal("-30 "), Ok((" ", Literal::Int(-30))));
    assert_eq!(literal("- 30 "), Ok((" ", Literal::Int(-30))));
    assert_eq!(literal("-IDENT "), Ok((" ", Literal::SignedIdentifier("-", "IDENT"))));
    assert_eq!(literal("IDENT "), Ok((" ", Literal::Identifier("IDENT"))));
}

fn ext_name(i: Span) -> IResult<Span, Span> {
    delimited(char('['), compound_ident, char(']'))(i)
}

fn any_name(i: Span) -> IResult<Span, (Span, Span)> {
    let (i, _) = ws(char('['))(i)?;
    let (i, inner) = ws(separated_pair(compound_ident, char('/'), compound_ident))(i)?;
    let (i, _) = ws(char(']'))(i)?;
    Ok((i, inner))
}

fn field_name(i: Span) -> IResult<Span, FieldName> {
    alt((
        map(ext_name, FieldName::Extended),
        map(any_name, |n| FieldName::AnyShorthand(n.0, n.1)),
        map(ident, FieldName::Normal),
    ))(i)
}

fn field(i: Span) -> IResult<Span, Field> {
    let (i, name) = ws(field_name)(i)?;
    let (i, value) = alt((scalar_field, message_field))(i)?;

    Ok((i, Field { name, value }))
}

fn scalar_value(i: Span) -> IResult<Span, Literal> {
    ws(literal)(i)
}

fn scalar_list(i: Span) -> IResult<Span, Vec<Literal>> {
    delimited(
        ws(char('[')),
        separated_list0(ws(char(',')), ws(literal)),
        ws(char(']')),
    )(i)
}

fn scalar_field(i: Span) -> IResult<Span, FieldValue> {
    let (i, _) = ws(char(':'))(i)?;
    let (i, values) = ws(alt((
        map(scalar_value, FieldValue::Scalar),
        map(scalar_list, FieldValue::ScalarList),
    )))(i)?;
    let (i, _) = ws(opt(one_of(";,")))(i)?;

    Ok((i, values))
}

fn message_field(i: Span) -> IResult<Span, FieldValue> {
    let (i, _) = ws(opt(tag(":")))(i)?;
    let (i, contents) = ws(alt((
        map(message_body, FieldValue::Message),
        map(message_list, FieldValue::MessageList),
    )))(i)?;
    let (i, _) = ws(opt(one_of(";,")))(i)?;

    Ok((i, contents))
}

pub fn message_body(i: Span) -> IResult<Span, Vec<Field>> {
    // println!("MSG BODY: {i}");
    let (i, t) = ws(one_of("{<"))(i)?;
    let (i, inner) = ws(cut(many0(field)))(i)?;
    let (i, _e) = ws(char(if t == '{' { '}' } else { '>' }))(i)?;

    Ok((i, inner))
}

#[test]
fn test_compount_ident() {
    assert_eq!(compound_ident("a.b"), Ok(("", "a.b")));
    assert_eq!(compound_ident("a"), Ok(("", "a")));
    assert_eq!(compound_ident("a/"), Ok(("/", "a")));
}

#[test]
fn test_message_body() {
    assert_eq!(message_body("{}"), Ok(("", vec![])));
    assert_eq!(
        message_body("{ enum: ENUM }"),
        Ok((
            "",
            vec![Field {
                name: FieldName::Normal("enum"),
                value: FieldValue::Scalar(Literal::Identifier("ENUM")),
            }]
        ))
    );
    assert_eq!(
        message_body(r#"{a: "1"}"#),
        Ok((
            "",
            vec![Field {
                name: FieldName::Normal("a"),
                value: FieldValue::Scalar(Literal::String(vec!["1"])),
            }]
        ))
    );
}

fn message_list(i: Span) -> IResult<Span, Vec<Vec<Field>>> {
    delimited(
        ws(char('[')),
        separated_list0(ws(char(',')), ws(message_body)),
        ws(char(']')),
    )(i)
}

pub fn _proto_file_comment(i: Span) -> IResult<Span, Span> {
    let (i, _name) = ws(tag("proto-file"))(i)?;
    let (i, _) = ws(char(':'))(i)?;
    let (i, path) = ws(take_till(|c| c == '\r' || c == '\n'))(i)?;

    Ok((i, path))
}

pub fn _proto_message_comment(i: Span) -> IResult<Span, Span> {
    let (i, _name) = ws(tag("proto-message"))(i)?;
    let (i, _) = ws(char(':'))(i)?;
    let (i, path) = ws(take_till(|c| c == '\r' || c == '\n'))(i)?;

    Ok((i, path))
}

pub fn textproto(i: Span) -> IResult<Span, TextProto> {
    let mut proto_file = None;
    let mut proto_message = None;
    // let (i, lines) = many0(multispace1)(i)?;
    let (i, comments) = many0(delimited(multispace0, comment, multispace0))(i)?;
    dbg!(&comments);
    for c in &comments {
        if let Ok((_j, pf)) = _proto_file_comment(c) {
            proto_file = Some(pf);
        }
        if let Ok((_j, pm)) = _proto_message_comment(c) {
            proto_message = Some(pm);
        }
    }
    let (i, fields) = many1(field)(i)?;
    Ok((
        i,
        TextProto {
            proto_file,
            proto_message,
            fields,
        },
    ))
}

#[test]
fn test_any_name() {
    assert_eq!(any_name("[domain.com/api.Type]"), Ok(("", ("domain.com", "api.Type"))));
    assert_eq!(any_name("[domain/api.Type]"), Ok(("", ("domain", "api.Type"))));
    assert_eq!(any_name("[com/api.Type]"), Ok(("", ("com", "api.Type"))));
    assert_eq!(any_name("[x/api.Type]"), Ok(("", ("x", "api.Type"))));
    assert!(any_name("[x/]").is_err());
    assert!(any_name("[/x]").is_err());
    assert!(any_name("[/]").is_err());
}

#[test]
fn test_field() {
    assert_eq!(
        field("field_name: 10  ;   "),
        Ok((
            "   ",
            Field {
                name: FieldName::Normal("field_name"),
                value: FieldValue::Scalar(Literal::Int(10)),
            }
        ))
    );
    assert_eq!(
        field("field_name: ENUM"),
        Ok((
            "",
            Field {
                name: FieldName::Normal("field_name"),
                value: FieldValue::Scalar(Literal::Identifier("ENUM")),
            }
        ))
    );
    assert_eq!(
        field("[com.google.a.field_name]: 10   "),
        Ok((
            "",
            Field {
                name: FieldName::Extended("com.google.a.field_name"),
                value: FieldValue::Scalar(Literal::Int(10)),
            }
        ))
    );
    assert_eq!(
        field("field_name { }"),
        Ok((
            "",
            Field {
                name: FieldName::Normal("field_name"),
                value: FieldValue::Message(vec![]),
            }
        ))
    );
    assert_eq!(
        field(r#"field_name { f: "Hello" }"#),
        Ok((
            "",
            Field {
                name: FieldName::Normal("field_name"),
                value: FieldValue::Message(vec![Field {
                    name: FieldName::Normal("f"),
                    value: FieldValue::Scalar(Literal::String(vec!["Hello"])),
                }]),
            }
        ))
    );

    assert_eq!(
        field(r#"[type.googleapis.com/com.example.SomeType] { typ: FORKING }"#),
        Ok((
            "",
            Field {
                name: FieldName::AnyShorthand("type.googleapis.com", "com.example.SomeType"),
                value: FieldValue::Message(vec![Field {
                    name: FieldName::Normal("typ"),
                    value: FieldValue::Scalar(Literal::Identifier("FORKING")),
                }]),
            }
        ))
    )
}

#[test]
fn test_textproto() {
    let tproto = r#"
    # proto-file: company/api/v1/any.proto
    # proto-message: BlaBLaBLa
    local_field: 10.0e10
    local_field: 10

# An Any value using regular fields.
any_value {
  type_url: "type.googleapis.com/com.example.SomeType"
  value: "\x0a\x05hello"  # serialized bytes of com.example.SomeType
}

# The same value using Any expansion
any_value {
  [type.googleapis.com/com.example.SomeType] {
    field1: "hello"
    type: FORKING
  }
}
    "#;

    let (left, _tp) = textproto(tproto).unwrap();
    assert_eq!(left, "")
}
