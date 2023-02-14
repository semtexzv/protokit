use protokit_desc::{BuiltinType, DataType, EnumFields, FieldDef, MessageFields, VariantDef};

use crate::ast::*;
use crate::translate::opts::opts;
use crate::translate::TranslateCtx;

struct FieldVisitor<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
    pub fields: &'tcx mut MessageFields,
}

impl Visitor for FieldVisitor<'_> {
    fn visit_message(&mut self, _item: &mut Message) {
        // todo!()
    }
    fn visit_oneof(&mut self, _item: &mut OneOf) {
        // todo!()
    }
    fn visit_enum(&mut self, _item: &mut Enum) {
        // todo!()
    }
    fn visit_map_field(&mut self, item: &mut MapField) {
        let val_type = match &item.val_type {
            Type::Builtin(b) => DataType::Builtin(*b),
            Type::Named(u) => DataType::Unresolved(self.ctx.def.cache(u)),
            Type::Map(_, _) => {
                self.ctx.error("Nested maps are not supported".to_string());
                DataType::Builtin(BuiltinType::Bool)
            }
        };
        self.fields.insert(FieldDef {
            name: self.ctx.def.cache(*item.name),
            frequency: Frequency::Singular,
            typ: DataType::Map(Box::new((item.key_type, val_type))),
            num: item.number,
            #[cfg(feature = "descriptors")]
            options: opts(self.ctx, item),
        })
    }
    fn visit_field(&mut self, item: &mut Field) {
        let dtyp = match &item.typ {
            Type::Builtin(b) => DataType::Builtin(*b),
            Type::Named(e) => DataType::Unresolved(self.ctx.def.cache(e)),
            Type::Map(k, v) => DataType::Map(Box::new((*k, DataType::Unresolved(self.ctx.def.cache(v))))),
        };
        let def = FieldDef {
            name: self.ctx.def.cache(*item.name),
            frequency: item.frequency,
            typ: dtyp,
            num: item.number,
            #[cfg(feature = "descriptors")]
            options: opts(self.ctx, item),
        };

        self.fields.insert(def);
    }
}

pub fn fields(ctx: &mut TranslateCtx, m: &mut impl AstNode) -> MessageFields {
    let mut f = MessageFields::default();
    m.accept(&mut FieldVisitor { ctx, fields: &mut f });
    f
}

pub struct EnumFieldVisitor<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
    pub variants: &'tcx mut EnumFields,
}

impl Visitor for EnumFieldVisitor<'_> {
    fn visit_enum_field(&mut self, item: &mut EnumField) {
        // let name = self.ctx.syms.intern(item.name);
        let name = self.ctx.def.cache(*item.name);
        let def = VariantDef {
            name: name.clone(),
            num: item.value,
            #[cfg(feature = "descriptors")]
            options: opts(self.ctx, item),
        };
        self.variants.by_name.insert(name, def);
    }
}

pub fn enum_fields(ctx: &mut TranslateCtx, n: &mut impl AstNode) -> EnumFields {
    let mut variants = EnumFields::default();
    n.accept(&mut EnumFieldVisitor {
        ctx,
        variants: &mut variants,
    });
    variants
}
