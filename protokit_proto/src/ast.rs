#![allow(dead_code)]

use protokit_desc::BuiltinType;
pub use protokit_desc::{FieldNum, Frequency, ImportType, Syntax};

pub type Span<'i> = &'i str;

#[derive(Debug, PartialEq, Eq)]
pub enum Type<'i> {
    Builtin(BuiltinType),
    Map(BuiltinType, Span<'i>),
    Unresolved(Span<'i>),
}

#[derive(Debug, PartialEq)]
pub enum Const<'i> {
    Bool(bool),
    Ident(Span<'i>),
    Str(Span<'i>),
    Int(i128),
    Float(f64),
    Compound(Vec<protokit_textformat::ast::Field<'i>>),
}

impl AstNode for Const<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            Const::Ident(i) => v.visit_ident_ref(i),
            _ => {}
        }
    }
}

pub trait AstNode {
    fn accept<V: Visitor>(&mut self, v: &mut V);
}

#[derive(Debug, PartialEq)]
pub struct Field<'i> {
    pub frequency: Frequency,
    pub typ: Type<'i>,
    pub name: Span<'i>,
    pub number: FieldNum,
    pub opts: Vec<Opt<'i>>,
}

impl AstNode for Field<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_frequency(&mut self.frequency);
        v.visit_type(&mut self.typ);
        v.visit_ident(&mut self.name);
        v.visit_field_num(self.number);
        for o in &mut self.opts {
            v.visit_opt(o);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MapField<'i> {
    pub key_type: BuiltinType,
    pub val_type: Type<'i>,
    pub name: Span<'i>,
    pub number: FieldNum,
    pub options: Vec<Opt<'i>>,
}

impl AstNode for MapField<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        // v.visit_type(&mut self.key_type);
        v.visit_type(&mut self.val_type);
        v.visit_ident(&mut self.name);
        v.visit_field_num(self.number);
        for o in &mut self.options {
            v.visit_opt(o);
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct OptName<'i> {
    pub name: Span<'i>,
    pub field_name: Option<Span<'i>>,
}

#[derive(Debug, PartialEq)]
pub struct Opt<'i> {
    pub name: OptName<'i>,
    pub value: Const<'i>,
}

impl AstNode for Opt<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident_ref(&mut self.name.name);
        // if let Some(f)
        // v.visit_ident_ref(&mut self.name.field_name);
        v.visit_const(&mut self.value);
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumField<'i> {
    pub name: Span<'i>,
    pub value: i32,
    pub opts: Vec<Opt<'i>>,
}

impl AstNode for EnumField<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        v.visit_field_num(self.value);
        for o in &mut self.opts {
            v.visit_opt(o);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EnumItem<'i> {
    Field(EnumField<'i>),
    Option(Opt<'i>),
}

impl AstNode for EnumItem<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            EnumItem::Field(f) => f.accept(v),
            EnumItem::Option(o) => o.accept(v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Enum<'i> {
    pub name: Span<'i>,
    pub items: Vec<EnumItem<'i>>,
}

impl AstNode for Enum<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        for i in &mut self.items {
            match i {
                EnumItem::Field(f) => v.visit_enum_field(f),
                EnumItem::Option(o) => v.visit_opt(o),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OneOfItem<'i> {
    Option(Opt<'i>),
    Field(Field<'i>),
    Group(Group<'i>),
}

impl AstNode for OneOfItem<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            OneOfItem::Option(o) => v.visit_opt(o),
            OneOfItem::Field(f) => v.visit_field(f),
            OneOfItem::Group(g) => v.visit_group(g),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OneOf<'i> {
    pub name: Span<'i>,
    pub items: Vec<OneOfItem<'i>>,
}

impl AstNode for OneOf<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        for i in &mut self.items {
            v.visit_oneof_item(i);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReservedRange {
    pub from: FieldNum,
    pub to: FieldNum,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reserved<'i> {
    Names(Vec<Span<'i>>),
    Ranges(Vec<ReservedRange>),
}

impl AstNode for Reserved<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            Reserved::Names(n) => {
                for i in n {
                    v.visit_ident(i);
                }
            }
            Reserved::Ranges(r) => {
                for r in r {
                    v.visit_range(r)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Extensions {
    pub ranges: Vec<ReservedRange>,
}

impl AstNode for Extensions {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        for r in &mut self.ranges {
            v.visit_range(r)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageItem<'i> {
    Field(Field<'i>),
    Enum(Enum<'i>),
    Message(Message<'i>),
    Option(Opt<'i>),
    OneOf(OneOf<'i>),
    MapField(MapField<'i>),
    Group(Group<'i>),
    Reserved(Reserved<'i>),
    Extensions(Extensions),
    Extend(Extension<'i>),
}

impl AstNode for MessageItem<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            MessageItem::Field(f) => v.visit_field(f),
            MessageItem::Enum(e) => v.visit_enum(e),
            MessageItem::Message(m) => v.visit_message(m),
            MessageItem::Option(o) => v.visit_opt(o),
            MessageItem::OneOf(o) => v.visit_oneof(o),
            MessageItem::MapField(m) => v.visit_map_field(m),
            MessageItem::Group(g) => v.visit_group(g),
            MessageItem::Reserved(r) => v.visit_reserved(r),
            MessageItem::Extensions(e) => v.visit_extensions(e),
            MessageItem::Extend(e) => v.visit_extend(e),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Message<'i> {
    pub name: Span<'i>,
    pub items: Vec<MessageItem<'i>>,
}

impl AstNode for Message<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        for i in &mut self.items {
            v.visit_message_item(i)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rpc<'i> {
    pub name: Span<'i>,
    pub msg_type: Type<'i>,
    pub msg_stream: bool,

    pub ret_type: Type<'i>,
    pub ret_stream: bool,

    pub options: Vec<Opt<'i>>,
}

impl AstNode for Rpc<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        v.visit_type(&mut self.msg_type);
        v.visit_type(&mut self.ret_type);
        for o in &mut self.options {
            v.visit_opt(o);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ServiceItem<'i> {
    Option(Opt<'i>),
    Rpc(Rpc<'i>),
}

impl AstNode for ServiceItem<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            ServiceItem::Option(o) => v.visit_opt(o),
            ServiceItem::Rpc(r) => v.visit_rpc(r),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Service<'i> {
    pub name: Span<'i>,
    pub items: Vec<ServiceItem<'i>>,
}

impl AstNode for Service<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident(&mut self.name);
        for it in &mut self.items {
            v.visit_service_item(it)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Group<'i> {
    pub frequency: Frequency,
    pub name: Span<'i>,
    pub number: FieldNum,
    pub items: Vec<MessageItem<'i>>,
}

impl AstNode for Group<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_frequency(&mut self.frequency);
        v.visit_ident(&mut self.name);
        v.visit_field_num(self.number);
        for m in &mut self.items {
            v.visit_message_item(m);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExtensionItem<'i> {
    Field(Field<'i>),
    Group(Group<'i>),
}

#[derive(Debug, PartialEq)]
pub struct Extension<'i> {
    pub name: Span<'i>,
    pub items: Vec<ExtensionItem<'i>>,
}

impl AstNode for Extension<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_ident_ref(&mut self.name);
        for it in &mut self.items {
            match it {
                ExtensionItem::Field(f) => v.visit_field(f),
                ExtensionItem::Group(g) => v.visit_group(g),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Def<'i> {
    Message(Message<'i>),
    Enum(Enum<'i>),
    Service(Service<'i>),
    Extend(Extension<'i>),
}

impl AstNode for Def<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            Def::Message(m) => v.visit_message(m),
            Def::Enum(e) => v.visit_enum(e),
            Def::Service(s) => v.visit_service(s),
            Def::Extend(e) => v.visit_extend(e),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import<'i> {
    pub typ: ImportType,
    pub path: Span<'i>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Package<'i> {
    pub path: Span<'i>,
}

#[derive(Debug, PartialEq)]
pub enum ProtoItem<'i> {
    Import(Import<'i>),
    Package(Package<'i>),
    Option(Opt<'i>),
    Def(Def<'i>),
}

impl AstNode for ProtoItem<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        match self {
            ProtoItem::Import(i) => v.visit_import(i),
            ProtoItem::Package(p) => v.visit_package(p),
            ProtoItem::Option(o) => v.visit_opt(o),
            ProtoItem::Def(d) => v.visit_def(d),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Proto<'i> {
    pub syntax: Syntax,
    pub items: Vec<ProtoItem<'i>>,
}

impl AstNode for Proto<'_> {
    fn accept<V: Visitor>(&mut self, v: &mut V) {
        v.visit_syntax(&mut self.syntax);
        for i in &mut self.items {
            v.visit_proto_item(i);
        }
    }
}

#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_proto(&mut self, item: &mut Proto) {
        item.accept(self);
    }
    fn visit_syntax(&mut self, syntax: &mut Syntax) {}
    fn visit_proto_item(&mut self, item: &mut ProtoItem) {
        item.accept(self)
    }
    fn visit_import(&mut self, item: &mut Import) {}
    fn visit_package(&mut self, item: &mut Package) {}
    fn visit_opts(&mut self, items: &mut [Opt]) {
        for o in items {
            self.visit_opt(o);
        }
    }
    fn visit_opt(&mut self, item: &mut Opt) {
        item.accept(self)
    }
    fn visit_def(&mut self, item: &mut Def) {
        item.accept(self)
    }
    fn visit_message(&mut self, item: &mut Message) {
        item.accept(self)
    }
    fn visit_message_item(&mut self, item: &mut MessageItem) {
        item.accept(self)
    }
    fn visit_enum(&mut self, item: &mut Enum) {
        item.accept(self)
    }
    fn visit_enum_item(&mut self, item: &mut EnumItem) {
        item.accept(self)
    }
    fn visit_enum_field(&mut self, item: &mut EnumField) {
        item.accept(self)
    }
    fn visit_oneof(&mut self, item: &mut OneOf) {
        item.accept(self)
    }
    fn visit_oneof_item(&mut self, item: &mut OneOfItem) {
        item.accept(self)
    }
    fn visit_field(&mut self, item: &mut Field) {
        item.accept(self)
    }
    fn visit_map_field(&mut self, item: &mut MapField) {
        item.accept(self)
    }
    fn visit_service(&mut self, item: &mut Service) {
        item.accept(self)
    }
    fn visit_service_item(&mut self, item: &mut ServiceItem) {
        item.accept(self)
    }
    fn visit_rpc(&mut self, rpc: &mut Rpc) {
        rpc.accept(self)
    }
    fn visit_extend(&mut self, item: &mut Extension) {
        item.accept(self)
    }
    fn visit_group(&mut self, item: &mut Group) {
        item.accept(self)
    }
    fn visit_reserved(&mut self, item: &mut Reserved) {
        item.accept(self)
    }
    fn visit_range(&mut self, item: &mut ReservedRange) {}
    fn visit_extensions(&mut self, item: &mut Extensions) {
        item.accept(self)
    }
    fn visit_frequency(&mut self, freq: &mut Frequency) {}
    fn visit_field_num(&mut self, num: FieldNum) {}
    /// Reference to a type
    fn visit_type(&mut self, typ: &mut Type) {}
    /// An idenditier, that is being declared
    fn visit_ident(&mut self, ident: &mut Span) {}
    /// Reference to existing identifier
    fn visit_ident_ref(&mut self, item: &mut Span) {}
    fn visit_const(&mut self, value: &mut Const) {
        value.accept(self)
    }
}
