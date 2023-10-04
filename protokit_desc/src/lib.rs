#![allow(unused_imports, clippy::match_like_matches_macro)]

use core::fmt::Debug;
use core::str::FromStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub use arcstr;
use arcstr::ArcStr;
pub(crate) use binformat::{BinProto, BytesLike, Fixed, Sigint, Varint};
pub(crate) use derive::{protoenum, Proto};
#[cfg(feature = "descriptors")]
pub use generated::google::protobuf::descriptor::*;
use indexmap::{indexmap, IndexMap};
pub(crate) use textformat::{TextField as _, TextProto};
pub(crate) use {binformat, textformat};

#[cfg(feature = "descriptors")]
pub mod generated;

pub type FieldNum = i32;
pub type GlobalDefId = u64;
pub type LocalDefId = u32;

pub const LOCAL_DEFID_MSG: LocalDefId = 0x80000000;
pub const LOCAL_DEFID_ENUM: LocalDefId = 0x40000000;
pub const LOCAL_DEFID_SVC: LocalDefId = 0x20000000;
pub const LOCAL_DEFID_EXT: LocalDefId = 0x10000000;

// Top byte is reserved
pub const LOCAL_ONLY_TYPE: LocalDefId = 0xFF000000;
pub const LOCAL_ONLY_ID: LocalDefId = 0x00FFFFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

impl FromStr for Syntax {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            other => Err(other.to_string()),
        }
    }
}

impl ToString for Syntax {
    fn to_string(&self) -> String {
        match *self {
            Syntax::Proto2 => "proto2".to_string(),
            Syntax::Proto3 => "proto3".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportType {
    Normal,
    Weak,
    Public,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    #[default]
    Singular,
    Optional,
    Repeated,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Bool,
    Fixed64,
    Sfixed64,
    Fixed32,
    Sfixed32,
    Double,
    Float,
    String_,
    Bytes_,
}

impl BuiltinType {
    pub fn is_scalar(&self) -> bool {
        match self {
            Self::String_ | Self::Bytes_ => false,
            _ => true,
        }
    }
}

impl FromStr for BuiltinType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BuiltinType::*;
        Ok(match s {
            "int32" => Int32,
            "int64" => Int64,
            "uint32" => Uint32,
            "uint64" => Uint64,
            "sint32" => Sint32,
            "sint64" => Sint64,
            "bool" => Bool,
            "fixed32" => Fixed32,
            "fixed64" => Fixed64,
            "sfixed32" => Sfixed32,
            "sfixed64" => Sfixed64,
            "double" => Double,
            "float" => Float,
            "string" => String_,
            "bytes" => Bytes_,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnresolvedHint {
    Message,
    Group,
    Enum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Was not yet resolved to message or an enum
    Unresolved(ArcStr, UnresolvedHint),
    /// One of the builtin types
    Builtin(BuiltinType),
    /// Message definition - LenPrefixed wire type
    Message(GlobalDefId),
    /// A message encoded with group wire format
    Group(GlobalDefId),
    /// Enum definition - Varint wire type
    Enum(GlobalDefId),
    /// Map from builtin to any other data type, Serialized as a simple message
    /// where k has a tag of 0, and v tag 1
    Map(Box<(BuiltinType, DataType)>),
}

impl Default for DataType {
    fn default() -> Self {
        Self::Unresolved(ArcStr::new(), UnresolvedHint::Message)
    }
}

impl DataType {
    pub fn is_message(&self) -> bool {
        matches!(self, DataType::Message(_))
    }
    pub fn is_scalar(&self) -> bool {
        match self {
            DataType::Builtin(b) => b.is_scalar(),
            DataType::Enum(_) => true,
            _ => false,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LenDelim = 2,
    StartGroup = 3,
    EndGroup = 4,
    Fixed32 = 5,
    Unk1 = 6,
    Unk2 = 7,
}

pub fn parser_wire_tag(field: &FieldDef) -> (WireType, Option<WireType>, u32) {
    let (normal, packed) = field.wire_types();
    (normal, packed, field.num as u32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Ident(ArcStr),
    Str(ArcStr),
    Int(i128),
    Float(f64),
    Object(HashMap<ArcStr, Value>),
}

#[cfg(feature = "descriptors")]
fn builtin_to_descriptor(bt: &BuiltinType) -> FieldDescriptorProtoType {
    match bt {
        BuiltinType::Sint32 => FieldDescriptorProtoType::TYPE_SINT32,
        BuiltinType::Int32 => FieldDescriptorProtoType::TYPE_INT32,
        BuiltinType::Int64 => FieldDescriptorProtoType::TYPE_INT64,
        BuiltinType::Uint32 => FieldDescriptorProtoType::TYPE_UINT32,
        BuiltinType::Uint64 => FieldDescriptorProtoType::TYPE_UINT64,
        BuiltinType::Sint64 => FieldDescriptorProtoType::TYPE_SINT64,
        BuiltinType::Bool => FieldDescriptorProtoType::TYPE_BOOL,
        BuiltinType::Fixed64 => FieldDescriptorProtoType::TYPE_FIXED64,
        BuiltinType::Sfixed64 => FieldDescriptorProtoType::TYPE_SFIXED64,
        BuiltinType::Fixed32 => FieldDescriptorProtoType::TYPE_FIXED32,
        BuiltinType::Sfixed32 => FieldDescriptorProtoType::TYPE_SFIXED32,
        BuiltinType::Double => FieldDescriptorProtoType::TYPE_DOUBLE,
        BuiltinType::Float => FieldDescriptorProtoType::TYPE_FLOAT,
        BuiltinType::String_ => FieldDescriptorProtoType::TYPE_STRING,
        BuiltinType::Bytes_ => FieldDescriptorProtoType::TYPE_BYTES,
    }
}

#[cfg(feature = "descriptors")]
fn type_to_descriptor(typ: &DataType) -> FieldDescriptorProtoType {
    match typ {
        DataType::Builtin(bt) => builtin_to_descriptor(bt),
        DataType::Message(_) => FieldDescriptorProtoType::TYPE_MESSAGE,
        DataType::Group(_) => FieldDescriptorProtoType::TYPE_GROUP,
        DataType::Enum(_) => FieldDescriptorProtoType::TYPE_ENUM,
        DataType::Map(_) => FieldDescriptorProtoType::TYPE_MESSAGE,
        DataType::Unresolved(u, _) => panic!("Unresolved type: {u:?}"),
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldDef {
    pub name: ArcStr,
    pub frequency: Frequency,
    pub typ: DataType,
    pub num: FieldNum,
    #[cfg(feature = "descriptors")]
    pub options: FieldOptions,
}

impl FieldDef {
    /// What wire types does this field support,
    /// the first value returned is the normal wire type,
    /// the second wire type refers to a wire type if the field supports packed repeated encoding
    pub fn wire_types(&self) -> (WireType, Option<WireType>) {
        use BuiltinType::*;
        use DataType::*;

        let normal = match &self.typ {
            Builtin(b) => match b {
                Int32 | Int64 | Sint32 | Sint64 | Uint32 | Uint64 | Bool => WireType::Varint,
                Fixed32 | Sfixed32 | Float => WireType::Fixed32,
                Fixed64 | Sfixed64 | Double => WireType::Fixed64,
                String_ | Bytes_ => WireType::LenDelim,
            },
            Map(_) => WireType::LenDelim,
            Message(_) => WireType::LenDelim,
            Group(_) => WireType::StartGroup,
            Enum(_) => WireType::Varint,
            Unresolved(_p, _) => panic!("Was not resolved"),
        };

        let second = match normal {
            WireType::Varint | WireType::Fixed32 | WireType::Fixed64 => Some(WireType::LenDelim),
            _ => None,
        };
        (normal, second)
    }

    pub fn default_wire_type(&self, force_packed: bool) -> WireType {
        match self.wire_types() {
            (_, Some(p)) if self.is_repeated() && (self.is_packed() || force_packed) => p,
            (norm, _) => norm,
        }
    }

    pub fn is_optional(&self) -> bool {
        self.frequency == Frequency::Optional
    }
    pub fn is_repeated(&self) -> bool {
        self.frequency == Frequency::Repeated
    }
    #[cfg(feature = "descriptors")]
    pub fn is_packed(&self) -> bool {
        self.is_repeated() && self.options.packed.unwrap_or(false)
    }

    #[cfg(feature = "descriptors")]
    pub fn set_packed(&mut self, packed: bool) {
        self.options.packed = Some(packed);
    }

    #[cfg(not(feature = "descriptors"))]
    pub fn is_packed(&self) -> bool {
        false
    }

    // #[cfg(not(feature = "descriptors"))]
    // pub fn set_packed(&mut self, packed: bool) {
    //     // self.options.packed = packed;
    // }

    pub fn is_message(&self) -> bool {
        match &self.typ {
            DataType::Message(_) => true,
            _ => false,
        }
    }

    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(
        set: &mut FileSetDef,
        file: &FileDescriptorProto,
        _msg_desc: &DescriptorProto,
        desc: &FieldDescriptorProto,
    ) -> Self {
        let opts = desc.options.as_deref().cloned().unwrap_or_default();

        let is_proto3 = file.syntax.as_deref() == Some("proto3");

        let mut name = set.cache(desc.name.as_ref().unwrap());

        let frequency = match &desc.label {
            Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL) if !is_proto3 => Frequency::Optional,
            Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL) if is_proto3 && desc.proto3_optional != Some(true) => {
                Frequency::Singular
            }
            Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL) if is_proto3 => Frequency::Optional,
            Some(FieldDescriptorProtoLabel::LABEL_REQUIRED) => Frequency::Required,
            Some(FieldDescriptorProtoLabel::LABEL_REPEATED) => Frequency::Repeated,
            Some(FieldDescriptorProtoLabel(label)) => {
                panic!("Unknown label: {label:?}")
            }
            None => Frequency::Singular,
        };

        let typ = match *desc.r#type.as_ref().unwrap() {
            FieldDescriptorProtoType::TYPE_DOUBLE => DataType::Builtin(BuiltinType::Double),
            FieldDescriptorProtoType::TYPE_FLOAT => DataType::Builtin(BuiltinType::Float),
            FieldDescriptorProtoType::TYPE_INT64 => DataType::Builtin(BuiltinType::Int64),
            FieldDescriptorProtoType::TYPE_UINT64 => DataType::Builtin(BuiltinType::Uint64),
            FieldDescriptorProtoType::TYPE_INT32 => DataType::Builtin(BuiltinType::Int32),
            FieldDescriptorProtoType::TYPE_FIXED64 => DataType::Builtin(BuiltinType::Fixed64),
            FieldDescriptorProtoType::TYPE_FIXED32 => DataType::Builtin(BuiltinType::Fixed32),
            FieldDescriptorProtoType::TYPE_BOOL => DataType::Builtin(BuiltinType::Bool),
            FieldDescriptorProtoType::TYPE_STRING => DataType::Builtin(BuiltinType::String_),
            FieldDescriptorProtoType::TYPE_GROUP => {
                let mut n = desc.type_name.as_deref().unwrap();
                while let Some(p) = n.find('.') {
                    n = &n[p + 1 ..]
                }
                name = set.cache(n);
                DataType::Unresolved(set.cache(desc.type_name.as_ref().unwrap()), UnresolvedHint::Group)
            }
            FieldDescriptorProtoType::TYPE_MESSAGE => {
                DataType::Unresolved(set.cache(desc.type_name.as_ref().unwrap()), UnresolvedHint::Message)
            }
            FieldDescriptorProtoType::TYPE_BYTES => DataType::Builtin(BuiltinType::Bytes_),
            FieldDescriptorProtoType::TYPE_UINT32 => DataType::Builtin(BuiltinType::Uint32),
            FieldDescriptorProtoType::TYPE_ENUM => {
                DataType::Unresolved(set.cache(desc.type_name.as_ref().unwrap()), UnresolvedHint::Enum)
            }
            FieldDescriptorProtoType::TYPE_SFIXED32 => DataType::Builtin(BuiltinType::Sfixed32),
            FieldDescriptorProtoType::TYPE_SFIXED64 => DataType::Builtin(BuiltinType::Sfixed64),
            FieldDescriptorProtoType::TYPE_SINT32 => DataType::Builtin(BuiltinType::Sint32),
            FieldDescriptorProtoType::TYPE_SINT64 => DataType::Builtin(BuiltinType::Sint64),

            FieldDescriptorProtoType(_) => panic!(),
        };
        Self {
            name,
            frequency,
            typ,
            num: desc.number.unwrap(),
            options: opts,
        }
    }

    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self, set: &FileSetDef, file: &FileDef, msg: &mut DescriptorProto) -> FieldDescriptorProto {
        let mut fout = FieldDescriptorProto {
            name: Some(self.name.to_string()),
            number: Some(self.num),
            ..Default::default()
        };

        match self.frequency {
            Frequency::Optional => fout.label = Some(FieldDescriptorProtoLabel::LABEL_OPTIONAL),
            Frequency::Repeated => fout.label = Some(FieldDescriptorProtoLabel::LABEL_REPEATED),
            Frequency::Required => fout.label = Some(FieldDescriptorProtoLabel::LABEL_REQUIRED),
            _ => {}
        }

        fout.r#type = Some(type_to_descriptor(&self.typ));

        match &self.typ {
            DataType::Message(m) | DataType::Group(m) => {
                let (message, file) = set.message_by_id(*m).unwrap();
                fout.type_name = Some(format!(".{}.{}", file.package, message.name))
            }
            DataType::Enum(e) => {
                let (en, file) = set.enum_by_id(*e).unwrap();
                fout.type_name = Some(format!(".{}.{}", file.package, en.name))
            }
            DataType::Map(map) => {
                let mut name = self.name.clone().to_string();
                unsafe {
                    name.as_bytes_mut()[.. 1].make_ascii_uppercase();
                }
                let map_entry_name = format!("{name}Entry");
                fout.type_name = Some(format!(
                    ".{}.{}.{}",
                    file.package,
                    msg.name.as_ref().unwrap(),
                    map_entry_name
                ));
                fout.label = Some(FieldDescriptorProtoLabel::LABEL_REPEATED);

                let key_field = FieldDescriptorProto {
                    name: Some("key".to_string()),
                    r#type: Some(builtin_to_descriptor(&map.0)),
                    number: Some(1),
                    ..Default::default()
                };

                let mut val_field = FieldDescriptorProto {
                    name: Some("value".to_string()),
                    r#type: Some(type_to_descriptor(&map.1)),
                    number: Some(2),
                    ..Default::default()
                };

                match &map.1 {
                    DataType::Builtin(_b) => {}
                    DataType::Message(m) => {
                        let (message, file) = set.message_by_id(*m).unwrap();
                        val_field.type_name = Some(format!(".{}.{}", file.package, message.name));
                    }
                    DataType::Enum(e) => {
                        let (en, file) = set.enum_by_id(*e).unwrap();
                        val_field.type_name = Some(format!(".{}.{}", file.package, en.name));
                    }
                    other => panic!("Unexpected map value type {other:?}"),
                }
                msg.nested_type.push(DescriptorProto {
                    name: Some(map_entry_name),
                    field: vec![key_field, val_field],
                    options: Some(Box::new(MessageOptions {
                        map_entry: Some(true),
                        ..Default::default()
                    })),
                    ..Default::default()
                });
            }
            DataType::Builtin(_) => {}
            DataType::Unresolved(u, _) => panic!("Unresolved type: {u:?}"),
        }

        fout
    }
}

#[derive(Debug, Default)]
pub struct VariantDef {
    pub name: ArcStr,
    pub num: FieldNum,
    #[cfg(feature = "descriptors")]
    pub options: FieldOptions,
}

#[derive(Debug, Default)]
pub struct EnumFields {
    pub by_name: IndexMap<ArcStr, VariantDef>,
}

impl EnumFields {
    pub fn by_name(&self, n: &str) -> Option<&VariantDef> {
        self.by_name.get(n)
    }
    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(set: &mut FileSetDef, desc: &[EnumValueDescriptorProto]) -> Self {
        let mut res = Self::default();
        for desc in desc {
            let name = set.cache(desc.name.as_ref().unwrap());
            res.by_name.insert(
                name.clone(),
                VariantDef {
                    name,
                    num: *desc.number.as_ref().unwrap(),
                    options: Default::default(),
                },
            );
        }
        res
    }
}

#[derive(Debug, Default)]
pub struct EnumDef {
    pub name: ArcStr,
    pub variants: EnumFields,
    #[cfg(feature = "descriptors")]
    pub options: EnumOptions,
}

impl EnumDef {
    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(set: &mut FileSetDef, name: &ArcStr, desc: &EnumDescriptorProto) -> Self {
        Self {
            name: name.clone(),
            variants: EnumFields::from_descriptor(set, &desc.value),
            options: Default::default(),
        }
    }

    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self) -> EnumDescriptorProto {
        let mut out = EnumDescriptorProto {
            name: Some(self.name.to_string()),
            ..Default::default()
        };

        for v in self.variants.by_name.values() {
            out.value.push(EnumValueDescriptorProto {
                name: Some(v.name.to_string()),
                number: Some(v.num as _),
                ..Default::default()
            });
        }

        out
    }
}

#[derive(Debug, Default)]
pub struct MessageFields {
    pub by_number: IndexMap<FieldNum, FieldDef>,
    pub by_name: IndexMap<ArcStr, FieldNum>,
}

impl MessageFields {
    pub fn insert(&mut self, field: FieldDef) {
        self.by_name.insert(field.name.clone(), field.num);
        self.by_number.insert(field.num, field);
    }
    pub fn by_name(&self, n: &str) -> Option<&FieldDef> {
        self.by_name.get(n).and_then(|n| self.by_number.get(n))
    }
    pub fn by_number(&self, n: FieldNum) -> Option<&FieldDef> {
        self.by_number.get(&n)
    }
    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(
        set: &mut FileSetDef,
        file: &FileDescriptorProto,
        oneof: Option<i32>,
        desc: &DescriptorProto,
        fields: &[FieldDescriptorProto],
    ) -> Self {
        let mut this: Self = Default::default();
        for f in fields {
            if f.oneof_index == oneof {
                this.by_name
                    .insert(set.cache(f.name.as_ref().unwrap()), f.number.unwrap());
                this.by_number
                    .insert(f.number.unwrap(), FieldDef::from_descriptor(set, file, desc, f));
            }
        }
        this
    }
}

#[derive(Debug, Default)]
pub struct MessageDef {
    pub name: ArcStr,
    pub fields: MessageFields,
    #[cfg(feature = "descriptors")]
    pub options: MessageOptions,
    pub oneofs: IndexMap<ArcStr, OneOfDef>,
    pub is_virtual_map: bool,
}

impl MessageDef {
    #[cfg(feature = "descriptors")]
    fn from_descriptor(
        set: &mut FileSetDef,
        file: &FileDescriptorProto,
        name: &ArcStr,
        desc: &DescriptorProto,
    ) -> Self {
        let is_virtual_map = desc.options.as_ref().and_then(|v| v.map_entry).unwrap_or(false);
        if is_virtual_map {
            eprintln!("{:?} is virtual map: {:?}", desc.name, desc);
        }
        MessageDef {
            name: name.clone(),
            fields: MessageFields::from_descriptor(set, file, None, desc, &desc.field),
            options: Default::default(),
            oneofs: desc
                .oneof_decl
                .iter()
                .enumerate()
                .map(|(id, od)| {
                    let name = set.cache(od.name.as_ref().unwrap());
                    (
                        name.clone(),
                        OneOfDef {
                            name,
                            fields: MessageFields::from_descriptor(set, file, Some(id as _), desc, &desc.field),
                            options: Default::default(),
                        },
                    )
                })
                .collect(),
            is_virtual_map,
        }
    }
    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self, set: &FileSetDef, file: &FileDef) -> DescriptorProto {
        let mut out = DescriptorProto {
            name: Some(self.name.to_string()),
            ..Default::default()
        };

        for f in self.fields.by_number.values() {
            let f: &FieldDef = f;
            let field = f.to_descriptor(set, file, &mut out);
            out.field.push(field);
        }
        out
    }
}

#[derive(Debug, Default)]
pub struct OneOfDef {
    pub name: ArcStr,
    pub fields: MessageFields,
    #[cfg(feature = "descriptors")]
    pub options: OneofOptions,
}

#[derive(Debug, Default)]
pub struct ServiceDef {
    pub name: ArcStr,
    pub rpc: IndexMap<ArcStr, RpcDef>,
    #[cfg(feature = "descriptors")]
    pub options: ServiceOptions,
}

impl ServiceDef {
    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self, set: &FileSetDef, file: &FileDef) -> ServiceDescriptorProto {
        let mut out = ServiceDescriptorProto {
            name: Some(self.name.to_string()),
            options: Some(Box::new(self.options.clone())),
            ..Default::default()
        };

        for rpc in self.rpc.values() {
            out.method.push(rpc.to_descriptor(set, file));
        }
        out
    }

    #[cfg(feature = "descriptors")]
    fn from_descriptor(
        set: &mut FileSetDef,
        file: &FileDescriptorProto,
        name: &ArcStr,
        desc: &ServiceDescriptorProto,
    ) -> Self {
        Self {
            name: name.clone(),
            rpc: desc
                .method
                .iter()
                .map(|v| {
                    let name = set.cache(v.name.as_ref().expect("Missing service name"));
                    (name.clone(), RpcDef::from_descriptor(set, &name, &v))
                })
                .collect(),
            options: desc.options.as_deref().cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RpcDef {
    pub name: ArcStr,
    pub req_stream: bool,
    pub req_typ: DataType,

    pub res_stream: bool,
    pub res_typ: DataType,
    #[cfg(feature = "descriptors")]
    pub options: MethodOptions,
}

impl RpcDef {
    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self, set: &FileSetDef, _: &FileDef) -> MethodDescriptorProto {
        let mut out = MethodDescriptorProto {
            name: Some(self.name.to_string()),
            client_streaming: Some(self.req_stream),
            server_streaming: Some(self.res_stream),
            options: Some(self.options.clone().into()),
            ..Default::default()
        };

        match &self.req_typ {
            DataType::Message(m) => {
                let (typ, file) = set.message_by_id(*m).unwrap();
                out.input_type = Some(format!(".{}.{}", file.package, typ.name));
            }
            other => panic!("Can't have {other:?} as rpc request type"),
        };

        match &self.res_typ {
            DataType::Message(m) => {
                let (typ, file) = set.message_by_id(*m).unwrap();
                out.output_type = Some(format!(".{}.{}", file.package, typ.name));
            }
            other => panic!("Can't have {other:?} as rpc response type"),
        };
        out
    }
    #[cfg(feature = "descriptors")]
    fn from_descriptor(set: &mut FileSetDef, name: &ArcStr, desc: &MethodDescriptorProto) -> Self {
        Self {
            name: name.clone(),
            req_stream: desc.client_streaming.unwrap_or_default(),
            req_typ: DataType::Unresolved(set.cache(desc.input_type.as_ref().unwrap()), UnresolvedHint::Message),
            res_stream: desc.server_streaming.unwrap_or_default(),
            res_typ: DataType::Unresolved(set.cache(desc.output_type.as_ref().unwrap()), UnresolvedHint::Message),
            options: desc.options.as_deref().cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct ExtendDef {
    pub in_message: ArcStr,
    pub name: ArcStr,
    pub fields: MessageFields,
}

#[derive(Debug)]
pub enum Definition {
    Message(MessageDef),
    Enum(EnumDef),
    Service(ServiceDef),
    Extend(ExtendDef),
}

#[derive(Debug, Default, Clone)]
pub struct ImportDef {
    pub name: ArcStr,
    pub file_idx: usize,
}

#[derive(Debug, Default)]
pub struct FileDef {
    pub name: ArcStr,
    pub imports: Vec<ImportDef>,
    pub public_imports: Vec<ImportDef>,

    pub syntax: Syntax,
    pub package: ArcStr,

    pub messages: IndexMap<ArcStr, MessageDef>,
    pub enums: IndexMap<ArcStr, EnumDef>,
    pub services: IndexMap<ArcStr, ServiceDef>,

    pub names: IndexMap<ArcStr, LocalDefId>,
    pub extensions: IndexMap<ArcStr, ExtendDef>,
    // How a given local definiton is extended by foreign global definitions
    pub extenders: IndexMap<LocalDefId, Vec<GlobalDefId>>,
}

impl FileDef {
    pub fn fill_names(&mut self) {
        self.messages.iter().enumerate().for_each(|(i, (n, _m))| {
            self.names.insert(n.clone(), (LOCAL_DEFID_MSG) | i as u32);
        });

        self.enums.iter().enumerate().for_each(|(i, (n, _m))| {
            self.names.insert(n.clone(), LOCAL_DEFID_ENUM | i as u32);
        });

        self.services.iter().enumerate().for_each(|(i, (n, _m))| {
            self.names.insert(n.clone(), LOCAL_DEFID_SVC | i as u32);
        });
    }

    pub fn resolve_types(&mut self, file_id: usize, prevs: &IndexMap<ArcStr, FileDef>) {
        self.messages.values_mut().for_each(|m: &mut MessageDef| {
            m.fields
                .by_number
                .values_mut()
                .chain(m.oneofs.values_mut().flat_map(|v| v.fields.by_number.values_mut()))
                .for_each(|f: &mut FieldDef| {
                    if !resolve_type(
                        &mut f.typ,
                        &self.names,
                        &self.imports,
                        file_id,
                        self.package.as_str(),
                        m.name.as_str(),
                        prevs,
                    ) {
                        panic!(
                            "Could not resolve {:?} for field: {} in file {}",
                            f.typ, f.name, self.name
                        );
                    }
                })
        });
        self.services.values_mut().for_each(|m: &mut ServiceDef| {
            m.rpc.values_mut().for_each(|r: &mut RpcDef| {
                if !resolve_type(
                    &mut r.req_typ,
                    &self.names,
                    &self.imports,
                    file_id,
                    self.package.as_str(),
                    m.name.as_str(),
                    prevs,
                ) {
                    panic!(
                        "Could not resolve {:?} for field: {} in file {}",
                        r.req_typ, r.name, self.name
                    );
                }
                if !resolve_type(
                    &mut r.res_typ,
                    &self.names,
                    &self.imports,
                    file_id,
                    self.package.as_str(),
                    m.name.as_str(),
                    prevs,
                ) {
                    panic!(
                        "Could not resolve {:?} for field: {} in file {}",
                        r.res_typ, r.name, self.name
                    );
                }
            })
        });
        self.extensions.values_mut().for_each(|m: &mut ExtendDef| {
            m.fields.by_number.values_mut().for_each(|f: &mut FieldDef| {
                if !resolve_type(
                    &mut f.typ,
                    &self.names,
                    &self.imports,
                    file_id,
                    self.package.as_str(),
                    m.in_message.as_str(),
                    prevs,
                ) {
                    panic!(
                        "Could not resolve {:?} for field: {} in file {}",
                        f.typ, f.name, self.name
                    );
                }
            })
        });

        let mut rewrites = BTreeMap::new();
        for (owner_idx, msg) in self.messages.values().enumerate() {
            for (field_idx, field) in msg.fields.by_number.values().enumerate() {
                if let DataType::Message(inner_id) = field.typ {
                    // Map fields can be only in same file.
                    if inner_id >> 32 != prevs.len() as _ {
                        continue;
                    }
                    let inner_id = inner_id as u32 & LOCAL_ONLY_ID;
                    if let Some((_, inner)) = &self.messages.get_index(inner_id as _) {
                        if inner.is_virtual_map {
                            let k = inner.fields.by_number(1).unwrap();
                            let v = inner.fields.by_number(2).unwrap();
                            let DataType::Builtin(k) = k.typ else { panic!() };
                            rewrites.insert((owner_idx, field_idx), DataType::Map(Box::new((k, v.typ.clone()))));
                        }
                    }
                }
            }
        }

        eprintln!("Rewriting map fields:{:?}", rewrites);
        for ((msg_id, field_id), change_to) in rewrites.into_iter() {
            let msg = self.messages.get_index_mut(msg_id).unwrap().1;
            let field = msg.fields.by_number.get_index_mut(field_id).unwrap().1;
            eprintln!(
                "Changing field {:?} {} {} in {:?} from : {:?}, to {:?}",
                field.name, msg_id, field_id, msg.name, field.typ, change_to
            );
            field.typ = change_to;
            field.frequency = Frequency::Singular;
        }
    }

    pub fn resolve_extensions(&mut self, file_id: usize, prevs: &mut IndexMap<ArcStr, FileDef>) {
        for (i, m) in self.extensions.iter() {
            let name = resolve_name(
                prevs,
                &self.names,
                &self.imports,
                file_id,
                self.package.as_str(),
                m.in_message.as_str(),
                m.name.as_str(),
            );
            if let Some(defid) = name {
                if let Some((_, file)) = prevs.get_index_mut((defid >> 32) as usize) {
                    let ext_id = self.extensions.get_index_of(i).unwrap() as u32 | LOCAL_DEFID_EXT;
                    let ext_id = local_to_global(file_id, ext_id);

                    let e = file.extenders.entry(defid as LocalDefId);
                    e.or_default().push(ext_id);
                } else if (defid >> 32) as usize == file_id {
                    let ext_id = self.extensions.get_index_of(i).unwrap() as u32 | LOCAL_DEFID_EXT;
                    let ext_id = local_to_global(file_id, ext_id);

                    let e = self.extenders.entry(defid as LocalDefId);
                    e.or_default().push(ext_id);
                } else {
                    panic!("Wrong ID")
                }
            } else {
                panic!("Unresolved extension name: {:#?}", self.names);
            }
        }
    }
    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(set: &mut FileSetDef, src: &FileDescriptorSet, desc: &FileDescriptorProto) -> Self {
        // panic!("{:#?}", src);
        let mut this = Self {
            name: set.cache(desc.name.as_ref().unwrap()),
            imports: desc
                .dependency
                .iter()
                .map(|v| {
                    let file_idx = src.file.iter().position(|f| f.name.as_deref() == Some(v.as_str()));
                    let Some(file_idx) = file_idx else {
                        panic!("Did not find {:?} in : {:#?}", v, src);
                    };
                    ImportDef {
                        name: set.cache(v),
                        file_idx,
                    }
                })
                .collect(),
            public_imports: vec![],
            syntax: desc
                .syntax
                .as_ref()
                .map(|s| Syntax::from_str(s))
                .transpose()
                .unwrap()
                .unwrap_or(Syntax::Proto2),
            package: set.cache(desc.package.as_ref().expect("Missing package name")),
            messages: Default::default(),
            enums: Default::default(),
            services: Default::default(),
            names: Default::default(),
            extensions: Default::default(),
            extenders: Default::default(),
        };

        fn parse_enum(set: &mut FileSetDef, this: &mut FileDef, parent: Option<&str>, desc: &EnumDescriptorProto) {
            let name = if let Some(parent) = parent {
                set.cache(&format!("{}.{}", parent, desc.name.as_ref().unwrap()))
            } else {
                set.cache(desc.name.as_ref().unwrap())
            };
            let def = EnumDef::from_descriptor(set, &name, desc);
            this.enums.insert(name, def);
        }

        fn parse_msg(
            set: &mut FileSetDef,
            file: &FileDescriptorProto,
            this: &mut FileDef,
            parent: Option<&str>,
            desc: &DescriptorProto,
        ) {
            let name = if let Some(parent) = parent {
                set.cache(&format!("{}.{}", parent, desc.name.as_ref().unwrap()))
            } else {
                set.cache(desc.name.as_ref().unwrap())
            };
            for desc in &desc.nested_type {
                parse_msg(set, file, this, Some(name.as_str()), desc)
            }
            for desc in &desc.enum_type {
                parse_enum(set, this, Some(name.as_str()), desc);
            }
            let def = MessageDef::from_descriptor(set, file, &name, desc);
            this.messages.insert(name, def);
        }
        fn parse_svc(
            set: &mut FileSetDef,
            file: &FileDescriptorProto,
            this: &mut FileDef,
            desc: &ServiceDescriptorProto,
        ) {
            let name = set.cache(desc.name.as_ref().unwrap());
            let def = ServiceDef::from_descriptor(set, file, &name, desc);
            this.services.insert(name, def);
        }

        for desc in &desc.enum_type {
            parse_enum(set, &mut this, None, desc);
        }

        for field_desc in &desc.message_type {
            parse_msg(set, desc, &mut this, None, field_desc);
        }
        for svc_desc in &desc.service {
            parse_svc(set, desc, &mut this, svc_desc);
        }
        this
    }
    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self, set: &FileSetDef) -> FileDescriptorProto {
        let mut out = FileDescriptorProto {
            name: Some(self.name.to_string()),
            syntax: Some(self.syntax.to_string()),
            package: Some(self.package.to_string()),
            ..Default::default()
        };

        for i in &self.imports {
            out.dependency.push(i.name.to_string());
        }

        for (i, import) in self.public_imports.iter().enumerate() {
            out.dependency.push(import.name.to_string());
            out.public_dependency.push((self.imports.len() + i) as _);
        }

        for m in self.messages.values() {
            println!("Serializing: {:?}", m.name);
            let m: &MessageDef = m;
            let mut name = m.name.as_str();
            let mut parent: Option<&mut DescriptorProto> = None;
            while name.contains('.') {
                let (parent_name, this_name) = name.split_once('.').unwrap();
                parent = if let Some(parent) = parent {
                    parent
                        .nested_type
                        .iter_mut()
                        .find(|m| m.name.as_deref() == Some(parent_name))
                } else {
                    out.message_type
                        .iter_mut()
                        .find(|m| m.name.as_deref() == Some(parent_name))
                };
                name = this_name;
            }
            if let Some(parent) = parent {
                let mut desc = m.to_descriptor(set, self);
                desc.name = name.to_string().into();
                parent.nested_type.push(desc);
            } else {
                out.message_type.push(m.to_descriptor(set, self));
            }
        }

        for en in self.enums.values() {
            println!("Serializing: {:?}", en.name);
            let en: &EnumDef = en;
            let mut name = en.name.as_str();
            let mut parent: Option<&mut DescriptorProto> = None;
            while name.contains('.') {
                let (parent_name, this_name) = name.split_once('.').unwrap();
                parent = if let Some(parent) = parent {
                    parent
                        .nested_type
                        .iter_mut()
                        .find(|m| m.name.as_deref() == Some(parent_name))
                } else {
                    out.message_type
                        .iter_mut()
                        .find(|m| m.name.as_deref() == Some(parent_name))
                };
                name = this_name;
            }
            if let Some(parent) = parent {
                let mut en = en.to_descriptor();
                en.name = name.to_string().into();
                parent.enum_type.push(en);
            } else {
                out.enum_type.push(en.to_descriptor());
            }
        }

        for svc in self.services.values() {
            out.service.push(svc.to_descriptor(set, self));
        }

        out
    }
}

fn local_to_global(file_id: usize, local_id: LocalDefId) -> GlobalDefId {
    (file_id as u64) << 32 | (local_id as u64)
}

fn global_to_type(def_id: GlobalDefId, hint: &UnresolvedHint) -> DataType {
    if def_id & LOCAL_DEFID_MSG as u64 != 0 {
        return if let UnresolvedHint::Group = hint {
            DataType::Group(def_id)
        } else {
            DataType::Message(def_id)
        };
    }
    if def_id & LOCAL_DEFID_ENUM as u64 != 0 {
        return DataType::Enum(def_id);
    }
    panic!("Invalid: {def_id:x?}")
}

fn resolve_type(
    to_resolve: &mut DataType,
    local_names: &IndexMap<ArcStr, LocalDefId>,
    local_imports: &[ImportDef],
    file_id: usize,
    local_package: &str,
    local_parent: &str,
    files: &IndexMap<ArcStr, FileDef>,
) -> bool {
    match to_resolve {
        DataType::Unresolved(u, hint) => {
            let resolved = resolve_name(
                files,
                local_names,
                local_imports,
                file_id,
                local_package,
                local_parent,
                u,
            );

            if let Some(sym) = resolved {
                *to_resolve = global_to_type(sym, hint);
                true
            } else {
                false
            }
        }
        DataType::Map(maptype) => {
            if let DataType::Unresolved(_u, _) = &mut maptype.1 {
                return resolve_type(
                    &mut maptype.1,
                    local_names,
                    local_imports,
                    file_id,
                    local_package,
                    local_parent,
                    files,
                );
            }
            true
        }
        _ => true,
    }
}

fn resolve_name(
    files: &IndexMap<ArcStr, FileDef>,
    names: &IndexMap<ArcStr, LocalDefId>,
    imports: &[ImportDef],
    file_idx: usize,
    package: &str,
    scope: &str,
    sym: &str,
) -> Option<GlobalDefId> {
    let resolved = try_resolve_symbol(names, package, scope, sym);
    if let Some(local_id) = resolved {
        return Some(local_to_global(file_idx, local_id));
    }

    for i in imports {
        let import_idx = files.get_index_of(&i.name).unwrap();
        let file = files.get_index(import_idx).unwrap().1;

        let file: &FileDef = file;
        let resolved = resolve_name(files, &file.names, &file.imports, import_idx, &file.package, "", sym);
        if let Some(resolved) = resolved {
            return Some(resolved);
        }
    }
    None
}

fn try_resolve_within_scopes(
    names: &IndexMap<ArcStr, LocalDefId>,
    mut scope: &str,
    symbol: &str,
) -> Option<LocalDefId> {
    // We don't accept global symbols. This is the inner resolution method
    assert!(!symbol.starts_with('.'));
    // Given a scope of First.Second.Third and name of Item
    // This method tries: First.Second.Third.Item => First.Second.Item => First.Item => Item
    loop {
        let scope_dot = if !scope.is_empty() { "." } else { "" };
        let qualified = format!("{scope}{scope_dot}{symbol}");
        match (names.get(qualified.as_str()), scope.rfind('.')) {
            (Some(v), _) => return Some(*v),
            (None, Some(p)) => scope = &scope[.. p],
            // Resolve globally without the prefix
            (None, None) => return names.get(symbol).copied(),
        }
    }
}

fn try_resolve_symbol(
    names: &IndexMap<ArcStr, LocalDefId>,
    mut file_package: &str,
    scope: &str,
    symbol: &str,
) -> Option<LocalDefId> {
    if let Some(without_dot) = symbol.strip_prefix('.') {
        // We're searching for global symbol. If package prefix matches, we can search for the inner part of the symbol
        if let Some(without_package) = without_dot.strip_prefix(file_package) {
            let localized_symbol = &without_package[1 ..];
            return names.get(localized_symbol).cloned();
        } else {
            eprintln!("Package mismatch: {} within: {}", without_dot, &file_package);
            return None;
        }
    } else if let Some(localized) = symbol.strip_prefix(file_package) {
        let localized_symbol = &localized[1 ..];
        return names.get(localized_symbol).cloned();
    }

    if let Some(v) = try_resolve_within_scopes(names, scope, symbol) {
        return Some(v);
    }
    let orig_package = file_package;
    loop {
        // Create a virtual global symbol, trying shorter package prefixes
        // If a package prefix is a match, We find it, and remove it in the inner function
        let namespaced = format!(".{file_package}.{symbol}");
        match (
            try_resolve_symbol(names, orig_package, "", namespaced.as_str()),
            file_package.rfind('.'),
        ) {
            (Some(v), _) => return Some(v),
            // We need to remove subpackages, because name sections might be of nested messages, not package names
            (None, Some(v)) => file_package = &file_package[.. v],
            (None, None) => {
                return try_resolve_within_scopes(names, "", symbol);
            }
        }
    }
}

pub enum DefType {
    Enum,
    Message,
    Service,
}

#[derive(Debug, Default)]
pub struct FileSetDef {
    /// Memoization table for identifiers
    pub symcache: HashSet<ArcStr>,
    /// Actual files that we're describing
    pub files: IndexMap<ArcStr, FileDef>,
}

impl FileSetDef {
    /// Intern the string, and return a reference counted version
    pub fn cache(&mut self, s: &str) -> ArcStr {
        if let Some(s) = self.symcache.get(s) {
            s.clone()
        } else {
            let s: ArcStr = ArcStr::from(s);
            self.symcache.insert(s.clone());
            s
        }
    }
}

impl FileSetDef {
    pub fn ext_by_id(&self, def: GlobalDefId) -> Option<(&ExtendDef, &FileDef)> {
        if def & LOCAL_DEFID_EXT as u64 == 0 {
            return None;
        }

        let file = self.files.values().nth((def as usize) >> 32);

        // eprintln!("MSG BY ID {} {} {:?},", (def as usize) >> 32, (def as u32 & util) as usize, file);
        file.and_then(|d| {
            d.extensions
                .values()
                .nth((def as u32 & LOCAL_ONLY_ID) as usize)
                .map(|v| (v, d))
        })
    }

    pub fn message_by_id(&self, def: GlobalDefId) -> Option<(&MessageDef, &FileDef)> {
        if def & LOCAL_DEFID_MSG as u64 == 0 {
            return None;
        }

        let file = self.files.values().nth((def as usize) >> 32);

        // eprintln!("MSG BY ID {} {} {:?},", (def as usize) >> 32, (def as u32 & util) as usize, file);
        file.and_then(|d| {
            d.messages
                .values()
                .nth((def as u32 & LOCAL_ONLY_ID) as usize)
                .map(|v| (v, d))
        })
    }

    pub fn enum_by_id(&self, def: GlobalDefId) -> Option<(&EnumDef, &FileDef)> {
        if def & LOCAL_DEFID_ENUM as u64 == 0 {
            return None;
        }

        let file = self.files.values().nth((def as usize) >> 32);

        file.and_then(|d| {
            d.enums
                .values()
                .nth((def as u32 & LOCAL_ONLY_ID) as usize)
                .map(|v| (v, d))
        })
    }

    #[cfg(feature = "descriptors")]
    pub fn from_descriptor(desc: FileDescriptorSet) -> Self {
        let mut this = Self::default();
        for f in desc.file.iter() {
            let mut file = FileDef::from_descriptor(&mut this, &desc, f);

            file.fill_names();
            file.resolve_types(this.files.len(), &this.files);
            file.resolve_extensions(this.files.len(), &mut this.files);
            this.files.insert(file.name.clone(), file);
        }
        this
    }
    #[cfg(feature = "descriptors")]
    pub fn from_bytes(data: &[u8]) -> FileSetDef {
        let desc: FileDescriptorSet = crate::binformat::decode(data).unwrap();
        Self::from_descriptor(desc)
    }
    #[cfg(feature = "descriptors")]
    pub fn to_descriptor(&self) -> FileDescriptorSet {
        let mut desc = FileDescriptorSet::default();
        for f in self.files.values() {
            desc.file.push(f.to_descriptor(self));
            // desc = desc.with_file(f.to_descriptor(self));
        }
        desc
    }
}

#[test]
fn test_resolution_scoping() {
    let names = indexmap! {
        ArcStr::from("Book.Chapter.Section.Line") => 1,
        ArcStr::from("Book.Chapter.Section") => 2,
        ArcStr::from("Book.Chapter") => 3,
        ArcStr::from("Book") => 4
    };
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section.Line", "Line"),
        Some(1)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section.Line", "Section"),
        Some(2)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section.Line", "Chapter"),
        Some(3)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section.Line", "Book"),
        Some(4)
    );

    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section", "Line"),
        Some(1)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section", "Section"),
        Some(2)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section", "Chapter"),
        Some(3)
    );
    assert_eq!(
        try_resolve_within_scopes(&names, "Book.Chapter.Section", "Book"),
        Some(4)
    );

    assert_eq!(try_resolve_within_scopes(&names, "Book.Chapter", "Line"), None);
    assert_eq!(try_resolve_within_scopes(&names, "Book.Chapter", "Section"), Some(2));
    assert_eq!(try_resolve_within_scopes(&names, "Book.Chapter", "Chapter"), Some(3));
    assert_eq!(try_resolve_within_scopes(&names, "Book.Chapter", "Book"), Some(4));

    assert_eq!(try_resolve_within_scopes(&names, "Book", "Line"), None);
    assert_eq!(try_resolve_within_scopes(&names, "Book", "Section"), None);
    assert_eq!(try_resolve_within_scopes(&names, "Book", "Chapter"), Some(3));
    assert_eq!(try_resolve_within_scopes(&names, "Book", "Book"), Some(4));

    assert_eq!(try_resolve_within_scopes(&names, "", "Line"), None);
    assert_eq!(try_resolve_within_scopes(&names, "", "Section"), None);
    assert_eq!(try_resolve_within_scopes(&names, "", "Chapter"), None);
    assert_eq!(try_resolve_within_scopes(&names, "", "Book"), Some(4));

    assert_eq!(
        try_resolve_symbol(
            &names,
            "com.company.books",
            "Book.Chapter.Section.Line",
            "company.books.Book",
        ),
        Some(4)
    );
}
