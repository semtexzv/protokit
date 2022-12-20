pub type Span<'i> = &'i str;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'i> {
    String(Vec<Span<'i>>),
    Float(f64),
    Int(i128),
    Identifier(Span<'i>),
    SignedIdentifier(Span<'i>, Span<'i>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldName<'i> {
    Normal(Span<'i>),
    Extended(Span<'i>),
    Any(Span<'i>, Span<'i>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'i> {
    pub name: FieldName<'i>,
    pub value: FieldValue<'i>,
}

#[cfg(feature = "map_syntax")]
#[derive(Debug, Clone, PartialEq)]
pub struct MapField<'i> {
    pub key: Literal<'i>,
    pub value: FieldValue<'i>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue<'i> {
    Scalar(Literal<'i>),
    ScalarList(Vec<Literal<'i>>),
    Message(Vec<Field<'i>>),
    MessageList(Vec<Vec<Field<'i>>>),
    #[cfg(feature = "map_syntax")]
    Map(Vec<MapField<'i>>),
    #[cfg(feature = "map_syntax")]
    MapList(Vec<Vec<MapField<'i>>>),
}

impl FieldValue<'_> {
    pub fn as_scalar(&self) -> Option<&Literal<'_>> {
        match self {
            FieldValue::Scalar(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_scalar_list(&self) -> Option<&[Literal]> {
        match self {
            FieldValue::ScalarList(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TextProto<'i> {
    pub proto_file: Option<Span<'i>>,
    pub proto_message: Option<Span<'i>>,

    pub fields: Vec<Field<'i>>,
}
