use crate::ast::*;

pub struct ValidateCtx {
    errors: Vec<String>,
}

pub trait Validator<'v>: Visitor {
    fn new(ctx: &'v mut ValidateCtx) -> Self;
}

pub struct NoGroupValidator<'v> {
    ctx: &'v mut ValidateCtx,
}

impl<'v> Validator<'v> for NoGroupValidator<'v> {
    fn new(ctx: &'v mut ValidateCtx) -> Self {
        Self { ctx }
    }
}

impl Visitor for NoGroupValidator<'_> {
    fn visit_group(&mut self, item: &mut Group) {
        self.ctx.errors.push(format!("Found group: {}", item.name));
    }
}

pub fn validate(proto: &mut Proto) -> Vec<String> {
    let mut ctx = ValidateCtx { errors: vec![] };

    NoGroupValidator::new(&mut ctx).visit_proto(proto);

    ctx.errors
}

#[test]
fn test_validate() {
    use crate::parser::proto_file;
    let (_rest, mut proto) = proto_file(include_str!("../../../proto/com/book/book.proto")).unwrap();
    let results = validate(&mut proto);
    assert_eq!(results.len(), 0, "{:?}", results);
}
