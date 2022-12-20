use std::path::Path;

use crate::ast::*;
use crate::translate::TranslateCtx;

/// Resolves imports & recursively translates imported files
pub struct ResolveImports<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
}

impl Visitor for ResolveImports<'_> {
    fn visit_import(&mut self, item: &mut Import) {
        let inner_path = Path::new(&*item.path);
        if self.ctx.def.files.get(*item.path).is_none() {
            eprintln!("Compiling imported: {inner_path:?}");
            self.ctx.compile_file(inner_path).unwrap();
        }
    }
}
