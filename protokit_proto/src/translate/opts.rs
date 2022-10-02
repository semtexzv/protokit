use protokit_textformat::ast::{FieldName, FieldValue, Literal};
use protokit_textformat::Context;

use crate::ast::*;
use crate::translate::TranslateCtx;

/// Visitor that processes opts on definition level, should not recurse
/// into nested definitions
struct InnerOptVisitor<'tcx, E> {
    pub ctx: &'tcx mut TranslateCtx,
    pub opts: &'tcx mut E,
}

impl<E: protokit_textformat::Decodable> Visitor for InnerOptVisitor<'_, E> {
    fn visit_opt(&mut self, item: &mut Opt) {
        let ctx = Context::default();
        let n = match (item.name.name, item.name.field_name) {
            ("default", _) => return,
            ("json_name", _) => return,
            (outer, None) => FieldName::Normal(outer),
            _ => panic!(),
        };

        let v = match &item.value {
            Const::Bool(true) => FieldValue::Scalar(Literal::Identifier("true")),
            Const::Bool(false) => FieldValue::Scalar(Literal::Identifier("false")),
            Const::Ident(id) => FieldValue::Scalar(Literal::Identifier(id)),
            Const::Str(s) => FieldValue::Scalar(Literal::String(vec![s])),
            Const::Int(i) => FieldValue::Scalar(Literal::Int(*i)),
            Const::Float(f) => FieldValue::Scalar(Literal::Float(*f)),
            Const::Compound(c) => FieldValue::Message(c.to_vec()),
        };

        let _ = self.opts.merge_field(&ctx, &n, &v);
    }

    fn visit_field(&mut self, _item: &mut Field) {}
    fn visit_message(&mut self, _item: &mut Message) {}
    fn visit_enum(&mut self, _item: &mut Enum) {}
    fn visit_oneof(&mut self, _item: &mut OneOf) {}
    fn visit_service(&mut self, _item: &mut Service) {}
    fn visit_rpc(&mut self, _rpc: &mut Rpc) {}
    fn visit_extend(&mut self, _item: &mut Extension) {}
    fn visit_group(&mut self, _item: &mut Group) {}
}

struct OuterOptVisitor<'tcx, E> {
    pub ctx: &'tcx mut TranslateCtx,
    pub opts: &'tcx mut E,
}

impl<E: protokit_textformat::Decodable> Visitor for OuterOptVisitor<'_, E> {
    // Option visitor does not recurse into nested definitions
    fn visit_opt(&mut self, item: &mut Opt) {
        InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        }
        .visit_opt(item)
    }
    fn visit_message_item(&mut self, item: &mut MessageItem) {
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_enum_item(&mut self, item: &mut EnumItem) {
        // trace!(visitor = "OuterOpt", "enum_item");
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_enum_field(&mut self, item: &mut EnumField) {
        // trace!(visitor = "OuterOpt", "enum_field");
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_oneof_item(&mut self, item: &mut OneOfItem) {
        // trace!(visitor = "OuterOpt", "oneof_item");
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_field(&mut self, item: &mut Field) {
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_map_field(&mut self, item: &mut MapField) {
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
    fn visit_service_item(&mut self, item: &mut ServiceItem) {
        item.accept(&mut InnerOptVisitor {
            ctx: self.ctx,
            opts: self.opts,
        })
    }
}

/// Directly reads opts from ast node without recursing
// pub fn opts(ctx: &mut TranslateCtx, n: &mut impl AstNode) -> Options {
//     let mut opts = Options::default();
//     n.accept(&mut OuterOptVisitor {
//         ctx,
//         opts: &mut opts,
//     });
//     opts
// }

pub fn opts<T: protokit_textformat::Decodable + Default>(ctx: &mut TranslateCtx, n: &mut impl AstNode) -> T {
    // let mut opts = opts(ctx, n);
    let mut out = T::default();
    // let ctx = Context::default();
    n.accept(&mut OuterOptVisitor { ctx, opts: &mut out });
    out
}

#[test]
fn test_opt_visitors() {
    tracing_subscriber::fmt::init();
    let _ast = Message {
        name: "outer_msg",
        items: vec![
            MessageItem::Option(Opt {
                name: OptName {
                    name: "outer",
                    field_name: None,
                },
                value: Const::Int(0),
            }),
            MessageItem::Message(Message {
                name: "inner_msg",
                items: vec![MessageItem::Option(Opt {
                    name: OptName {
                        name: "inner",
                        field_name: None,
                    },
                    value: Const::Int(1),
                })],
            }),
            MessageItem::Enum(Enum {
                name: "en1",
                items: vec![EnumItem::Option(Opt {
                    name: OptName {
                        name: "allow_alias",
                        field_name: None,
                    },
                    value: Const::Int(2),
                })],
            }),
        ],
    };
    // assert_eq!(
    //     opts(&mut TranslateCtx::new(), &mut ast)
    //         .values()
    //         .collect::<Vec<_>>(),
    //     vec![&Value::Int(0)]
    // );
    // assert_eq!(
    //     opts(&mut TranslateCtx::new(), &mut ast.items[1])
    //         .values()
    //         .collect::<Vec<_>>(),
    //     vec![&Value::Int(1)]
    // );
    // assert_eq!(
    //     opts(&mut TranslateCtx::new(), &mut ast.items[2])
    //         .values()
    //         .collect::<Vec<_>>(),
    //     vec![&Value::Int(2)]
    // );
}
