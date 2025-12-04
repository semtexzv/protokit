use lex_core::{parse_with_options, NumberFormatBuilder};
use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, tag_no_case, take_until, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace1, none_of, one_of};
use nom::combinator::{cut, map, map_res, opt, recognize, value};
use nom::error::{context, FromExternalError};
use nom::multi::{many0, many0_count, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::Slice;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::tag::TagError;

use crate::ast::*;
use crate::deps::*;
use crate::{IResult, MyParseError, Parse};

pub const TAG_MAX: FieldNum = 536_870_911;

fn is_eol(c: char) -> bool {
    c == '\r' || c == '\n'
}

fn semicolon(i: Span) -> IResult<()> {
    match ws(tag(";"))(i) {
        Ok((i, _)) => Ok((i, ())),
        Err(_e) => IResult::Err(nom::Err::Failure(MyParseError::from_tag(i, "Trailing semicolon"))),
    }
}

pub fn eol_comment(i: Span) -> IResult<Span> {
    let (i, _) = tag("//")(i)?;
    let (i, inside) = take_while(|c| !is_eol(c))(i)?;
    let (i, _) = take_while(is_eol)(i)?;
    Ok((i, inside))
}

pub fn inline_comment(i: Span) -> IResult<Span> {
    let (i, _) = tag("/*")(i)?;
    let (i, inside) = take_until("*/")(i)?;
    let (i, _) = tag("*/")(i)?;
    Ok((i, inside))
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

fn ws<'a, F: 'a, O>(mut inner: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: FnMut(Span<'a>) -> IResult<O>,
{
    move |i: Span| {
        let (i, _) = many0(alt((eol_comment, inline_comment, multispace1)))(i)?;
        inner(i)
    }
}

/// Combinator for cutting errors in parsing.
/// If first one suceeeds, and second one fails, it means that there is a fatal syntax error
fn determined<'i, I, O, II, OI>(mut first: I, mut second: O) -> impl FnMut(Span<'i>) -> IResult<OI>
where
    I: FnMut(Span<'i>) -> IResult<II>,
    O: FnMut(Span<'i>) -> IResult<OI>,
{
    move |mut i| {
        i = first(i)?.0;
        cut(&mut second)(i)
    }
}

/// A parser that searches for string prefix. After matching the prefix, the
/// inner parser must suceeed
fn prefixed<'i, P, R>(s: &'static str, parser: P) -> impl FnMut(Span<'i>) -> IResult<R>
where
    P: FnMut(Span<'i>) -> IResult<R>,
{
    determined(ws(tag(s)), context(s, parser))
}

fn ident(i: Span) -> IResult<Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(i)
}

fn full_ident(i: Span) -> IResult<Span> {
    recognize(tuple((opt(char('.')), separated_list1(tag("."), ident))))(i)
}

fn msg_or_enum_type(i: Span) -> IResult<Type> {
    map(recognize(tuple((opt(char('.')), full_ident))), |v| Type::Named(*v))(i)
}

fn oct_digit(i: Span) -> IResult<char> {
    one_of("01234567")(i)
}

fn hex_digit(i: Span) -> IResult<char> {
    one_of("0123456789abcdefABCDEF")(i)
}

fn dec_digit(i: Span) -> IResult<char> {
    one_of("0123456789")(i)
}

fn hex_lit(i: Span) -> IResult<Span> {
    preceded(alt((tag("0x"), tag("0X"))), recognize(many1(hex_digit)))(i)
}

fn octal_lit(i: Span) -> IResult<Span> {
    preceded(alt((tag("0"), tag("0"))), recognize(many1(oct_digit)))(i)
}

fn decimal_lit(i: Span) -> IResult<Span> {
    recognize(many1(dec_digit))(i)
}

/// i128 allows us to work with u64 and i64 in single code path
fn int_lit(i: Span) -> IResult<i128> {
    alt((
        map_res(hex_lit, strhex),
        map_res(octal_lit, stroct),
        map_res(decimal_lit, strdec),
    ))(i)
}

fn field_num(i: Span) -> IResult<FieldNum> {
    map(int_lit, |f| f.try_into().expect("Field number too big {}"))(i)
}

fn exponent(i: Span) -> IResult<Span> {
    recognize(tuple((one_of("eE"), opt(one_of("+-")), decimal_lit)))(i)
}

fn float_lit(i: Span) -> IResult<f64> {
    let a = recognize(tuple((decimal_lit, tag("."), opt(exponent), decimal_lit)));

    let b = recognize(tuple((decimal_lit, exponent)));

    let c = recognize(tuple((tag("."), decimal_lit, opt(exponent))));

    let d = recognize(alt((tag("inf"), tag("nan"))));

    map_res(alt((a, b, c, d)), |v| v.parse())(i)
}

fn bool_lit(i: Span) -> IResult<bool> {
    map(alt((tag_no_case("true"), tag_no_case("false"))), |v: Span| *v == "true")(i)
}

fn escape_contents(i: Span) -> IResult<Span> {
    let char_escape = recognize(one_of("abfnrtv\\'\"0?"));
    let oct_escape = recognize(tuple((oct_digit, oct_digit, oct_digit)));
    let hex_escape = recognize(tuple((one_of("xX"), hex_digit, hex_digit)));

    alt((oct_escape, hex_escape, char_escape))(i)
}

fn str_lit(i: Span) -> IResult<Span> {
    let normal = none_of("\\\0\n\"");
    let contents = recognize(opt(escaped(normal, '\\', escape_contents)));
    delimited(char('"'), contents, char('"'))(i)
}

fn opt_sign(i: Span) -> IResult<Option<char>> {
    opt(one_of("+-"))(i)
}

fn compound_constant<'a>(_i: Span<'a>) -> IResult<'a, Const<'a>> {
    unimplemented!()
    // let (i, res) = protokit_textformat::parser::message_body(i)?;
    // Ok((i, Const::Compound(res)))
}

fn constant<'a>(i: Span<'a>) -> IResult<'a, Const<'a>> {
    let ilit = map(
        tuple((opt_sign, int_lit)),
        |(sign, val)| {
            if sign == Some('-') {
                -val
            } else {
                val
            }
        },
    );
    let flit = map(
        tuple((opt_sign, float_lit)),
        |(sign, val)| {
            if sign == Some('-') {
                -val
            } else {
                val
            }
        },
    );

    alt((
        map(str_lit, |v| Const::Str(*v)),
        map(bool_lit, Const::Bool),
        map(recognize(full_ident), |v| Const::Ident(*v)),
        map(flit, Const::Float),
        map(ilit, Const::Int),
        compound_constant,
    ))(i)
}

// Elements with whitespace start from here:
fn syntax(i: Span) -> IResult<Syntax> {
    prefixed("syntax", |i| {
        let (i, _) = ws(tag("="))(i)?;
        let (i, syntax) = ws(map_res(alt((tag("\"proto2\""), tag("\"proto3\""))), |s: Span| {
            Syntax::from_str(s.trim_matches('\"'))
        }))(i)?;
        let (i, _) = ws(tag(";"))(i)?;
        Ok((i, syntax))
    })(i)
}

fn import_type(i: Span) -> IResult<ImportType> {
    map(
        opt(alt((
            value(ImportType::Weak, tag("weak")),
            value(ImportType::Public, tag("public")),
        ))),
        |v| v.unwrap_or(ImportType::Normal),
    )(i)
}

fn import<'a>(i: Span<'a>) -> IResult<'a, Import<'a>> {
    prefixed("import", |i| {
        let (i, typ) = ws(import_type)(i)?;
        let (i, path) = ws(str_lit)(i)?;
        let (i, _) = semicolon(i)?;
        Ok((i, Import { typ, path }))
    })(i)
}

fn package<'a>(i: Span<'a>) -> IResult<'a, Package<'a>> {
    prefixed("package", |i| {
        let (i, path) = ws(recognize(full_ident))(i)?;
        let (i, _) = semicolon(i)?;

        Ok((i, Package { path }))
    })(i)
}

fn option_name(i: Span) -> IResult<OptName> {
    let simple = full_ident;
    let complex_paren = delimited(tag("("), full_ident, tag(")"));
    let complex_field = opt(preceded(ws(tag(".")), ws(ident)));
    alt((
        map(simple, |s| OptName {
            name: s,
            field_name: None,
        }),
        map(tuple((complex_paren, complex_field)), |(p, f)| OptName {
            name: p,
            field_name: f,
        }),
    ))(i)
    // recognize(tuple((alt((simple, complex_paren)), recognize(many0(complex_field)))))(i)
}

fn option<'a>(i: Span<'a>) -> IResult<'a, super::ast::Opt<'a>> {
    let (i, _) = ws(tag("option"))(i)?;
    let (i, name) = ws(option_name)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, value) = ws(constant)(i)?;
    let (i, _) = semicolon(i)?;

    Ok((i, super::ast::Opt { name, value }))
}

fn builtin(i: Span) -> IResult<BuiltinType> {
    let types = [
        "double", "float", "int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32", "fixed64", "sfixed32",
        "sfixed64", "bool", "string", "bytes",
    ];
    for t in types {
        if i.len() >= t.len() && i.starts_with(t) {
            return Ok((i.slice(t.len()..), BuiltinType::from_str(&i[..t.len()]).unwrap()));
        }
    }
    Err(nom::Err::Error(GenericErrorTree::from_external_error(
        i,
        nom::error::ErrorKind::Alpha,
        "builtin type",
    )))
}

fn ftype(i: Span) -> IResult<Type> {
    alt((map(ws(builtin), Type::Builtin), ws(msg_or_enum_type)))(i)
}

fn field_option<'a>(i: Span<'a>) -> IResult<'a, Opt<'a>> {
    let (i, name) = ws(option_name)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, value) = ws(constant)(i)?;

    Ok((i, Opt { name, value }))
}

fn field_options_brackets<'a>(i: Span<'a>) -> IResult<'a, Vec<Opt<'a>>> {
    let opts = separated_list1(tag(","), field_option);
    delimited(ws(char('[')), opts, ws(char(']')))(i)
}

fn frequency(i: Span) -> IResult<Frequency> {
    let (i, freq) = opt(alt((ws(tag("optional")), ws(tag("repeated")), ws(tag("required")))))(i)?;
    let freq = match freq.map(|v| *v) {
        Some("optional") => Frequency::Optional,
        Some("repeated") => Frequency::Repeated,
        Some("required") => Frequency::Required,
        Some(_) => panic!("Unexpected frequency"),
        None => Frequency::Singular,
    };
    Ok((i, freq))
}

fn field<'a>(i: Span<'a>) -> IResult<'a, Field<'a>> {
    let (i, frequency) = ws(frequency)(i)?;
    let (i, ftype) = ws(ftype)(i)?;
    let (i, name) = ws(ident)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, num) = ws(field_num)(i)?;
    let (i, options) = ws(opt(field_options_brackets))(i)?;

    let (i, _) = semicolon(i)?;

    Ok((
        i,
        Field {
            frequency,
            typ: ftype,
            name,
            number: num,
            opts: options.unwrap_or_default(),
        },
    ))
}

fn oneof_field<'a>(i: Span<'a>) -> IResult<'a, Field<'a>> {
    let (i, ftype) = ws(ftype)(i)?;
    let (i, name) = ws(ident)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, num) = ws(field_num)(i)?;
    let (i, options) = ws(opt(field_options_brackets))(i)?;
    let (i, _) = semicolon(i)?;
    Ok((
        i,
        Field {
            frequency: Frequency::Singular,

            typ: ftype,
            name,
            number: num,
            opts: options.unwrap_or_default(),
        },
    ))
}

fn oneof_item<'a>(i: Span<'a>) -> IResult<'a, OneOfItem<'a>> {
    alt((
        ws(map(option, OneOfItem::Option)),
        ws(map(group, OneOfItem::Group)),
        ws(map(oneof_field, OneOfItem::Field)),
    ))(i)
}

fn oneof_body<'a>(i: Span<'a>) -> IResult<'a, Vec<OneOfItem<'a>>> {
    delimited(ws(tag("{")), many0(ws(oneof_item)), ws(tag("}")))(i)
}

fn oneof<'a>(i: Span<'a>) -> IResult<'a, OneOf<'a>> {
    prefixed("oneof", |i| {
        let (i, name) = ws(ident)(i)?;
        let (i, items) = oneof_body(i)?;
        Ok((i, OneOf { name, items }))
    })(i)
}

fn key_type(i: Span) -> IResult<BuiltinType> {
    builtin(i)
}

fn map_field<'a>(i: Span<'a>) -> IResult<'a, MapField<'a>> {
    prefixed("map", |i| {
        let (i, _) = ws(tag("<"))(i)?;
        let (i, key_type) = ws(key_type)(i)?;
        let (i, _) = ws(tag(","))(i)?;
        let (i, val_type) = ws(ftype)(i)?;
        let (i, _) = ws(tag(">"))(i)?;
        let (i, name) = ws(ident)(i)?;
        let (i, _) = ws(tag("="))(i)?;
        let (i, number) = ws(field_num)(i)?;
        let (i, options) = ws(opt(field_options_brackets))(i)?;
        let (i, _) = semicolon(i)?;

        Ok((
            i,
            MapField {
                key_type,
                val_type,
                name,
                number,
                options: options.unwrap_or_default(),
            },
        ))
    })(i)
}

fn res_range(i: Span) -> IResult<ReservedRange> {
    let (i, from) = ws(field_num)(i)?;

    let max = ws(value(TAG_MAX, tag("max")));
    let end = ws(alt((field_num, max)));
    let to = opt(preceded(ws(tag("to")), end));

    let (i, to) = ws(to)(i)?;

    Ok((
        i,
        ReservedRange {
            from,
            to: to.unwrap_or(from),
        },
    ))
}

fn res_str_item(i: Span) -> IResult<Span> {
    alt((
        delimited(char('\''), ident, char('\'')),
        delimited(char('\"'), ident, char('\"')),
    ))(i)
}

fn reserved<'a>(i: Span<'a>) -> IResult<'a, Reserved<'a>> {
    prefixed("reserved", |i| {
        let ranges = separated_list1(ws(char(',')), ws(res_range));
        let str_names = separated_list1(ws(char(',')), ws(res_str_item));

        let (i, items) = ws(alt((map(str_names, Reserved::Names), map(ranges, Reserved::Ranges))))(i)?;
        let (i, _) = ws(tag(";"))(i)?;

        Ok((i, items))
    })(i)
}

fn extensions(i: Span) -> IResult<Extensions> {
    prefixed("extensions", |i| {
        let ranges = separated_list1(ws(char(',')), ws(res_range));

        let (i, ranges) = ws(ranges)(i)?;
        let (i, _) = ws(tag(";"))(i)?;

        Ok((i, Extensions { ranges }))
    })(i)
}

fn enum_lit(i: Span) -> IResult<i32> {
    let (i, (sign, mut val)) = tuple((opt(char('-')), int_lit))(i)?;
    if let Some('-') = sign {
        val = -val;
    };
    Ok((i, val.try_into().expect("Field number too big")))
}

fn enum_field<'a>(i: Span<'a>) -> IResult<'a, EnumField<'a>> {
    let (i, name) = ws(ident)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, value) = ws(enum_lit)(i)?;
    let (i, options) = ws(opt(field_options_brackets))(i)?;
    let (i, _) = ws(tag(";"))(i)?;

    Ok((
        i,
        EnumField {
            name,
            value,
            opts: options.unwrap_or_default(),
        },
    ))
}

fn enum_item<'a>(i: Span<'a>) -> IResult<'a, EnumItem<'a>> {
    alt((map(option, EnumItem::Option), map(enum_field, EnumItem::Field)))(i)
}

fn enum_body<'a>(i: Span<'a>) -> IResult<'a, Vec<EnumItem<'a>>> {
    let (i, body) = delimited(ws(tag("{")), many0(ws(enum_item)), ws(tag("}")))(i)?;
    let (i, _) = ws(opt(char(';')))(i)?;
    Ok((i, body))
}

fn _enum(i: Span) -> IResult<Enum> {
    prefixed("enum", |i| {
        let (i, name) = ws(ident)(i)?;
        let (i, items) = ws(enum_body)(i)?;
        Ok((i, Enum { name, items }))
    })(i)
}

fn message_item<'a>(i: Span<'a>) -> IResult<'a, MessageItem<'a>> {
    alt((
        map(group, MessageItem::Group),
        map(field, MessageItem::Field),
        map(_enum, MessageItem::Enum),
        map(message, MessageItem::Message),
        map(option, MessageItem::Option),
        map(oneof, MessageItem::OneOf),
        map(map_field, MessageItem::MapField),
        map(reserved, MessageItem::Reserved),
        map(extensions, MessageItem::Extensions),
        map(extend, MessageItem::Extend),
    ))(i)
}

fn message_body<'a>(i: Span<'a>) -> IResult<'a, Vec<MessageItem<'a>>> {
    let (i, body) = delimited(ws(tag("{")), many0(ws(message_item)), ws(tag("}")))(i)?;
    let (i, _) = ws(opt(char(';')))(i)?;

    Ok((i, body))
}

fn message<'a>(i: Span<'a>) -> IResult<'a, Message<'a>> {
    determined(ws(tag("message")), |i| {
        let (i, name) = ws(ident)(i)?;
        let (i, items) = ws(message_body)(i)?;
        Ok((i, Message { name, items }))
    })(i)
}

fn rpc_arg(i: Span) -> IResult<(bool, Type)> {
    delimited(
        ws(tag("(")),
        tuple((ws(map(opt(tag("stream")), |v| v.is_some())), ws(msg_or_enum_type))),
        ws(tag(")")),
    )(i)
}

fn rpc_options<'a>(i: Span<'a>) -> IResult<'a, Vec<Opt<'a>>> {
    delimited(ws(char('{')), many0(option), ws(tag("}")))(i)
}

fn rpc(i: Span) -> IResult<Rpc> {
    prefixed("rpc", |i| {
        let _opts = alt((ws(rpc_options), ws(map(tag(";"), |_| vec![]))));
        let (i, name) = ws(ident)(i)?;
        let (i, arg) = ws(rpc_arg)(i)?;
        let (i, _) = ws(tag("returns"))(i)?;
        let (i, ret) = ws(rpc_arg)(i)?;

        let (i, options) = ws(_opts)(i)?;

        Ok((
            i,
            Rpc {
                name,
                msg_type: arg.1,
                msg_stream: arg.0,
                ret_type: ret.1,
                ret_stream: ret.0,
                options,
            },
        ))
    })(i)
}

fn service_body<'a>(i: Span<'a>) -> IResult<'a, Vec<ServiceItem<'a>>> {
    delimited(
        ws(tag("{")),
        many0(ws(alt((
            map(option, ServiceItem::Option),
            map(rpc, ServiceItem::Rpc),
            // TODO: Add empty stmt
        )))),
        ws(tag("}")),
    )(i)
}

fn service<'a>(i: Span<'a>) -> IResult<'a, Service<'a>> {
    prefixed("service", |i| {
        let (i, name) = ws(ident)(i)?;
        let (i, items) = ws(service_body)(i)?;

        Ok((i, Service { name, items }))
    })(i)
}

fn extend_item(i: Span) -> IResult<ExtensionItem> {
    alt((map(field, ExtensionItem::Field), map(group, ExtensionItem::Group)))(i)
}

fn extend_body(i: Span) -> IResult<Vec<ExtensionItem>> {
    delimited(ws(char('{')), ws(many0(extend_item)), ws(char('}')))(i)
}

fn extend<'a>(i: Span<'a>) -> IResult<'a, Extension<'a>> {
    prefixed("extend", |i| {
        let (i, mtype) = ws(recognize(full_ident))(i)?;
        let (i, items) = ws(extend_body)(i)?;
        Ok((i, Extension { name: mtype, items }))
    })(i)
}

// TODO: Add requirement to start with capital letter
fn group(i: Span) -> IResult<Group> {
    let (i, freq) = ws(frequency)(i)?;
    prefixed("group", move |i| {
        let (i, name) = ws(ident)(i)?;
        let (i, _) = ws(tag("="))(i)?;
        let (i, number) = ws(field_num)(i)?;
        let (i, items) = ws(message_body)(i)?;

        Ok((
            i,
            Group {
                frequency: freq,
                name,
                number,
                items,
            },
        ))
    })(i)
}

fn def<'a>(i: Span<'a>) -> IResult<'a, Def<'a>> {
    alt((
        map(message, Def::Message),
        map(_enum, Def::Enum),
        map(service, Def::Service),
        map(extend, Def::Extend),
    ))(i)
}

fn file_item(i: Span) -> IResult<ProtoItem> {
    alt((
        map(import, ProtoItem::Import),
        map(package, ProtoItem::Package),
        map(option, ProtoItem::Option),
        map(def, ProtoItem::Def),
    ))(i)
}

impl<'i> Parse<'i> for Proto<'i> {
    fn parse(i: Span<'i>) -> IResult<'i, Self> {
        let (i, syntax) = syntax(i)?;
        let (i, items) = many0(file_item)(i)?;

        // Eat unused whitespace
        let (i, _) = ws(tag(""))(i)?;
        Ok((i, Proto { syntax, items }))
    }
}
// #[cfg(test)]
// mod tests {
//     use protokit_desc::BuiltinType::{Int32, Int64, String_};
//     use protokit_textformat::ast::{FieldName, FieldValue, Literal};
//
//     use super::*;
//
//     #[test]
//     fn test_ident() {
//         assert_eq!(ident("abcd efg"), Ok((" efg", "abcd")));
//         // assert_eq!(ident("_"), Err(nom::Err::Error(Error { input: "", code: Tag })));
//     }
//
//     #[test]
//     fn test_import() {
//         assert_eq!(
//             import(r#"import public "hello.proto";"#),
//             Ok((
//                 "",
//                 Import {
//                     typ: ImportType::Public,
//                     path: "hello.proto",
//                 }
//             ))
//         );
//     }
//
//     #[test]
//     fn test_str_lit() {
//         assert_eq!(str_lit(r#""""#), Ok(("", "")));
//         assert_eq!(str_lit(r#""a""#), Ok(("", "a")));
//         assert_eq!(str_lit(r#""a\n""#), Ok(("", r#"a\n"#)));
//         assert_eq!(str_lit(r#""a\\""#), Ok(("", r#"a\\"#)));
//         assert_eq!(str_lit(r#""a\"""#), Ok(("", r#"a\""#)));
//         assert_eq!(
//             str_lit(r#""\0\001\a\b\f\n\r\t\v\\\'\"\xfe""#),
//             Ok(("", r#"\0\001\a\b\f\n\r\t\v\\\'\"\xfe"#))
//         );
//         assert_eq!(str_lit(r#""com.example.foo""#), Ok(("", r#"com.example.foo"#)));
//     }
//
//     #[test]
//     fn test_constant() {
//         assert_eq!(
//             constant(r#""com.example.foo""#),
//             Ok(("", Const::Str("com.example.foo")))
//         );
//         assert_eq!(int_lit("0xFFFFFFFF"), Ok(("", 0xFFFFFFFF)));
//
//         assert_eq!(
//             compound_constant("{a: true}"),
//             Ok((
//                 "",
//                 Const::Compound(vec![protokit_textformat::ast::Field {
//                     name: FieldName::Normal("a"),
//                     value: FieldValue::Scalar(Literal::Identifier("true")),
//                 }])
//             ))
//         )
//     }
//
//     #[test]
//     fn test_syntax() {
//         assert_eq!(syntax(r#"syntax = "proto3";"#), Ok(("", Syntax::Proto3)))
//     }
//
//     #[test]
//     fn test_enum_field() {
//         assert_eq!(
//             enum_field(r#"RUNNING = 2 [(custom_option) = "hello world"];"#),
//             Ok((
//                 "",
//                 EnumField {
//                     name: "RUNNING",
//                     value: 2,
//                     opts: vec![Opt {
//                         name: OptName {
//                             name: "custom_option",
//                             field_name: None,
//                         },
//                         value: Const::Str("hello world"),
//                     }],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_enum() {
//         assert_eq!(
//             _enum("enum Test{}"),
//             Ok((
//                 "",
//                 Enum {
//                     name: "Test",
//                     items: vec![],
//                 }
//             ))
//         );
//         assert_eq!(
//             _enum(
//                 r#"
// enum EnumAllowingAlias {
//   option allow_alias = true;
//   UNKNOWN = 0;
//   STARTED = 1;
//   RUNNING = 2 [(custom_option) = "hello world"];
// }"#
//             ),
//             Ok((
//                 "",
//                 Enum {
//                     name: "EnumAllowingAlias",
//                     items: vec![
//                         EnumItem::Option(Opt {
//                             name: OptName {
//                                 name: "allow_alias",
//                                 field_name: None,
//                             },
//                             value: Const::Bool(true),
//                         }),
//                         EnumItem::Field(EnumField {
//                             name: "UNKNOWN",
//                             value: 0,
//                             opts: vec![],
//                         }),
//                         EnumItem::Field(EnumField {
//                             name: "STARTED",
//                             value: 1,
//                             opts: vec![],
//                         }),
//                         EnumItem::Field(EnumField {
//                             name: "RUNNING",
//                             value: 2,
//                             opts: vec![Opt {
//                                 name: OptName {
//                                     name: "custom_option",
//                                     field_name: None,
//                                 },
//                                 value: Const::Str("hello world"),
//                             }],
//                         }),
//                     ],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_reserved() {
//         assert_eq!(
//             reserved(r#"reserved "foo", "bar";"#),
//             Ok(("", Reserved::Names(vec!["foo", "bar"])))
//         );
//     }
//
//     #[test]
//     fn test_option_name() {
//         assert_eq!(
//             option_name("(custom_option)"),
//             Ok((
//                 "",
//                 OptName {
//                     name: "custom_option",
//                     field_name: None,
//                 }
//             ))
//         );
//         assert_eq!(
//             option_name("(.protobuf_unittest.complex_opt1).foo"),
//             Ok((
//                 "",
//                 OptName {
//                     name: ".protobuf_unittest.complex_opt1",
//                     field_name: Some("foo"),
//                 }
//             ))
//         );
//     }
//
//     #[test]
//     fn test_option() {
//         assert_eq!(
//             option(r#"option java_package = "com.example.foo";"#),
//             Ok((
//                 "",
//                 Opt {
//                     name: OptName {
//                         name: "java_package",
//                         field_name: None,
//                     },
//                     value: Const::Str("com.example.foo"),
//                 }
//             ))
//         );
//     }
//
//     #[test]
//     fn test_type() {
//         assert_eq!(ftype("int32 abcd"), Ok((" abcd", Type::Builtin(Int32))));
//         assert_eq!(ftype("Message32"), Ok(("", Type::Unresolved("Message32"))));
//     }
//
//     #[test]
//     fn test_field_option() {
//         assert_eq!(
//             field_option("(packed)=true"),
//             Ok((
//                 "",
//                 Opt {
//                     name: OptName {
//                         name: "packed",
//                         field_name: None,
//                     },
//                     value: Const::Bool(true),
//                 }
//             ))
//         );
//         assert_eq!(
//             field_options_brackets("[packed=true]"),
//             Ok((
//                 "",
//                 vec![Opt {
//                     name: OptName {
//                         name: "packed",
//                         field_name: None,
//                     },
//                     value: Const::Bool(true),
//                 }]
//             ))
//         );
//         assert_eq!(
//             field_options_brackets(r#"[default = "\0\001\a\b\f\n\r\t\v\\\'\"\xfe"]"#),
//             Ok((
//                 "",
//                 vec![Opt {
//                     name: OptName {
//                         name: "default",
//                         field_name: None,
//                     },
//                     value: Const::Str(r#"\0\001\a\b\f\n\r\t\v\\\'\"\xfe"#),
//                 }]
//             ))
//         );
//         assert_eq!(
//             field_options_brackets(r#"[default = 0xFFFFFFFF]"#),
//             Ok((
//                 "",
//                 vec![Opt {
//                     name: OptName {
//                         name: "default",
//                         field_name: None,
//                     },
//                     value: Const::Int(0xFFFFFFFF),
//                 }]
//             ))
//         );
//         assert_eq!(
//             field_options_brackets(r#"[(custom_option) = "hello world"]"#),
//             Ok((
//                 "",
//                 vec![Opt {
//                     name: OptName {
//                         name: "custom_option",
//                         field_name: None,
//                     },
//                     value: Const::Str("hello world"),
//                 }]
//             ))
//         );
//         assert_eq!(
//             field_options_brackets(
//                 r#"[
//                 (google.api.http) = {
//                 post: "/v3/kv/put"
//                 body: "*"
//             }]"#
//             ),
//             Ok((
//                 "",
//                 vec![Opt {
//                     name: OptName {
//                         name: "google.api.http",
//                         field_name: None,
//                     },
//                     value: Const::Compound(vec![
//                         protokit_textformat::ast::Field {
//                             name: FieldName::Normal("post"),
//                             value: FieldValue::Scalar(Literal::String(vec!["/v3/kv/put"])),
//                         },
//                         protokit_textformat::ast::Field {
//                             name: FieldName::Normal("body"),
//                             value: FieldValue::Scalar(Literal::String(vec!["*"])),
//                         },
//                     ]),
//                 }]
//             ))
//         );
//     }
//
//     #[test]
//     fn test_enum_lit() {
//         assert_eq!(enum_lit("2"), Ok(("", 2)));
//         assert_eq!(enum_lit("-2"), Ok(("", -2)));
//     }
//
//     #[test]
//     fn test_group() {
//         assert_eq!(
//             group(
//                 r#"optional group OptionalGroup_extension_lite = 16 {
//                     optional int32 a = 17;
//                   }"#
//             ),
//             Ok((
//                 "",
//                 Group {
//                     frequency: Frequency::Optional,
//                     name: "OptionalGroup_extension_lite",
//                     number: 16,
//                     items: vec![MessageItem::Field(Field {
//                         frequency: Frequency::Optional,
//                         typ: Type::Builtin(BuiltinType::Int32),
//                         name: "a",
//                         number: 17,
//                         opts: vec![],
//                     })],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_message() {
//         let input = r#"message Outer {
//   int64 ival = 1;
// }"#;
//         message(input).unwrap();
//     }
//
//     #[test]
//     fn test_message_item() {
//         assert_eq!(
//             message_item("int64 ival = 1;"),
//             Ok((
//                 "",
//                 MessageItem::Field(Field {
//                     frequency: Frequency::Singular,
//                     typ: Type::Builtin(Int64),
//                     name: "ival",
//                     number: 1,
//                     opts: vec![],
//                 })
//             ))
//         )
//     }
//
//     #[test]
//     fn test_fields() {
//         assert_eq!(
//             field("repeated int32 samples = 4 [packed=true];"),
//             Ok((
//                 "",
//                 Field {
//                     frequency: Frequency::Repeated,
//                     typ: Type::Builtin(Int32),
//                     name: "samples",
//                     number: 4,
//                     opts: vec![Opt {
//                         name: OptName {
//                             name: "packed",
//                             field_name: None,
//                         },
//                         value: Const::Bool(true),
//                     }],
//                 }
//             ))
//         );
//
//         assert_eq!(
//             field("optional  int32 default_int32    = 61 [default =  41    ];"),
//             Ok((
//                 "",
//                 Field {
//                     frequency: Frequency::Optional,
//                     typ: Type::Builtin(Int32),
//                     name: "default_int32",
//                     number: 61,
//                     opts: vec![Opt {
//                         name: OptName {
//                             name: "default",
//                             field_name: None,
//                         },
//                         value: Const::Int(41),
//                     }],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_oneof_field() {
//         assert_eq!(
//             oneof_field("SubMessage sub_message = 9;"),
//             Ok((
//                 "",
//                 Field {
//                     frequency: Frequency::Singular,
//                     typ: Type::Unresolved("SubMessage"),
//                     name: "sub_message",
//                     number: 9,
//                     opts: vec![],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_oneof() {
//         assert_eq!(
//             oneof_item("string id = 32;"),
//             Ok((
//                 "",
//                 OneOfItem::Field(Field {
//                     frequency: Frequency::Singular,
//                     typ: Type::Builtin(BuiltinType::String_),
//                     name: "id",
//                     number: 32,
//                     opts: vec![],
//                 })
//             ))
//         );
//         assert_eq!(
//             oneof("oneof foo {}"),
//             Ok((
//                 "",
//                 OneOf {
//                     name: "foo",
//                     items: vec![],
//                 }
//             ))
//         );
//     }
//
//     #[test]
//     fn test_map_field() {
//         assert_eq!(
//             map_field("map<string, Project> projects = 3;"),
//             Ok((
//                 "",
//                 MapField {
//                     key_type: String_,
//                     val_type: Type::Unresolved("Project"),
//                     name: "projects",
//                     number: 3,
//                     options: vec![],
//                 }
//             ))
//         );
//
//         let i = "map<Project, Project> projects = 3;";
//         let p = map_field(i);
//         match p {
//             Ok(_) => {}
//             Err(nom::Err::Failure(e)) => {
//                 panic!("{}", nom::error::convert_error(i, e));
//             }
//             _ => {}
//         }
//         // assert_eq!(
//         //     map_field(),
//         //     Ok((
//         //         "",
//         //         MapField {
//         //             key_type: String_,
//         //             val_type: Type::Unresolved("Project"),
//         //             name: "projects",
//         //             number: 3,
//         //             options: vec![],
//         //         }
//         //     ))
//         // );
//     }
//
//     #[test]
//     fn test_res_range() {
//         assert_eq!(res_range(r#"1"#), Ok(("", ReservedRange { from: 1, to: 1 })));
//         assert_eq!(res_range(r#"0"#), Ok(("", ReservedRange { from: 0, to: 0 })));
//         assert_eq!(
//             res_range(r#"0 to max"#),
//             Ok(("", ReservedRange { from: 0, to: TAG_MAX }))
//         );
//
//         assert_eq!(res_str_item(r#""a", "b""#), Ok((r#", "b""#, r#"a"#)));
//     }
//
//     #[test]
//     fn test_extend() {
//         let i = r#"extend Foo {
//   optional int32 bar = 126;
// }"#;
//         assert_eq!(
//             extend(i),
//             Ok((
//                 "",
//                 Extension {
//                     name: "Foo",
//                     items: vec![ExtensionItem::Field(Field {
//                         frequency: Frequency::Optional,
//                         typ: Type::Builtin(Int32),
//                         name: "bar",
//                         number: 126,
//                         opts: vec![],
//                     })],
//                 }
//             ))
//         );
//         assert_eq!(
//             extend(
//                 r#"extend TestAllExtensionsLite {
//                             optional TestRequiredLite single = 1000;
//                           }"#
//             ),
//             Ok((
//                 "",
//                 Extension {
//                     name: "TestAllExtensionsLite",
//                     items: vec![ExtensionItem::Field(Field {
//                         frequency: Frequency::Optional,
//                         typ: Type::Unresolved("TestRequiredLite"),
//                         name: "single",
//                         number: 1000,
//                         opts: vec![],
//                     })],
//                 }
//             ))
//         )
//     }
//
//     #[test]
//     fn test_rpc() {
//         assert_eq!(
//             rpc("rpc Search (SearchRequest) returns (SearchResponse);"),
//             Ok((
//                 "",
//                 Rpc {
//                     name: "Search",
//                     msg_type: Type::Unresolved("SearchRequest"),
//                     msg_stream: false,
//                     ret_type: Type::Unresolved("SearchResponse"),
//                     ret_stream: false,
//                     options: vec![],
//                 }
//             ))
//         );
//
//         assert_eq!(
//             rpc(r#"rpc Search (SearchRequest) returns (SearchResponse) { option name = "int"; }"#),
//             Ok((
//                 "",
//                 Rpc {
//                     name: "Search",
//                     msg_type: Type::Unresolved("SearchRequest"),
//                     msg_stream: false,
//                     ret_type: Type::Unresolved("SearchResponse"),
//                     ret_stream: false,
//                     options: vec![Opt {
//                         name: OptName {
//                             name: "name",
//                             field_name: None,
//                         },
//                         value: Const::Str("int"),
//                     }],
//                 }
//             ))
//         );
//     }
//
#[test]
fn test_proto_file() {
    let input = r#"
syntax = "proto3";
message Outer {
Strict ival = 11;
}"#;
    match Proto::parse_format_error(input) {
        Ok(_) => {}
        Err(e) => {
            let mut s = String::new();
            miette::GraphicalReportHandler::new().render_report(&mut s, &e).unwrap();
            println!("{s}");
        }
    };
}
