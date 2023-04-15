use protokit_desc::arcstr::ArcStr;
use protokit_desc::{
    DataType, EnumDef, ExtendDef, FileDef, ImportDef, MessageDef, OneOfDef, RpcDef, ServiceDef, UnresolvedHint,
};

use crate::ast::*;
use crate::deps::*;
use crate::translate::fields::{enum_fields, fields};
// use crate::translate::opts::opts;
use crate::translate::TranslateCtx;

pub struct FillDefinitions<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
    pub unit: &'tcx mut FileDef,
    pub path: String,
}

impl FillDefinitions<'_> {
    fn qualify(&mut self, name: &str) -> (ArcStr, String) {
        // let name = self.ctx.syms.intern(n);
        let name = self.ctx.def.cache(name);
        let mut qualified = self.path.to_string();
        if !qualified.is_empty() {
            qualified.push('.');
        }
        qualified.push_str(&name);
        (name, qualified)
    }
}

impl Visitor for FillDefinitions<'_> {
    // TODO: Add package prefix to all symbols, or work with packages on other level ?
    fn visit_package(&mut self, item: &mut Package) {
        self.unit.package = self.ctx.def.cache(*item.path);
    }
    fn visit_import(&mut self, item: &mut Import) {
        let i = ImportDef {
            name: self.ctx.def.cache(*item.path),
            file_idx: self.ctx.def.files.get_index_of(*item.path).unwrap_or(666),
        };
        if let ImportType::Public = item.typ {
            self.unit.public_imports.push(i.clone());
        }
        self.unit.imports.push(i);
    }
    fn visit_message(&mut self, item: &mut Message) {
        let before = self.path.clone();
        let (_name, qualified_name) = self.qualify(*item.name);

        let name = self.ctx.def.cache(&qualified_name);
        self.unit.messages.insert(
            name.clone(),
            MessageDef {
                name,
                fields: fields(self.ctx, item),
                #[cfg(feature = "descriptors")]
                options: opts(self.ctx, item),
                oneofs: oneofs(self.ctx, item),
                is_virtual_map: false,
                ..Default::default()
            },
        );
        self.path = qualified_name;
        item.accept(self);
        self.path = before;
    }
    fn visit_group(&mut self, item: &mut Group) {
        let before = self.path.clone();
        let (_name, qualified_name) = self.qualify(*item.name);

        let name = self.ctx.def.cache(&qualified_name);
        self.unit.messages.insert(
            name.clone(),
            MessageDef {
                name,
                fields: fields(self.ctx, item),

                #[cfg(feature = "descriptors")]
                options: opts(self.ctx, item),
                oneofs: oneofs(self.ctx, item),
                is_virtual_map: false,
                ..Default::default()
            },
        );
        self.path = qualified_name;
        item.accept(self);
        self.path = before;
    }
    fn visit_extend(&mut self, item: &mut Extension) {
        let name = self.ctx.def.cache(*item.name);
        self.unit.extensions.insert(
            name.clone(),
            ExtendDef {
                in_message: self.ctx.def.cache(self.path.as_str()),
                name,
                fields: fields(self.ctx, item),
            },
        );
    }
    fn visit_enum(&mut self, item: &mut Enum) {
        let (_name, qualified_name) = self.qualify(*item.name);

        let name = self.ctx.def.cache(&qualified_name);
        // TODO: Insert enum variant names into appropriate search place
        self.unit.enums.insert(
            name.clone(),
            EnumDef {
                name,
                variants: enum_fields(self.ctx, item),
                #[cfg(feature = "descriptors")]
                options: opts(self.ctx, item),
                ..Default::default()
            },
        );
    }

    fn visit_service(&mut self, item: &mut Service) {
        let (_name, qualified_name) = self.qualify(*item.name);
        let name = self.ctx.def.cache(&qualified_name);
        let svc = ServiceDef {
            name: name.clone(),
            rpc: rpcs(self.ctx, item),
            #[cfg(feature = "descriptors")]
            options: opts(self.ctx, item),
            ..Default::default()
        };

        self.unit.services.insert(name, svc);
    }
}

pub struct GatherOneOfs<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
    pub message: &'tcx mut IndexMap<ArcStr, OneOfDef>,
}

impl Visitor for GatherOneOfs<'_> {
    fn visit_oneof(&mut self, item: &mut OneOf) {
        let name = self.ctx.def.cache(*item.name);
        self.message.insert(
            name.clone(),
            OneOfDef {
                name,
                fields: fields(self.ctx, item),
                #[cfg(feature = "descriptors")]
                options: opts(self.ctx, item),
                ..Default::default()
            },
        );
    }
}

pub fn oneofs(ctx: &mut TranslateCtx, n: &mut impl AstNode) -> IndexMap<ArcStr, OneOfDef> {
    let mut oneofs = indexmap! {};
    n.accept(&mut GatherOneOfs {
        ctx,
        message: &mut oneofs,
    });
    oneofs
}

pub struct GatherRpcs<'tcx> {
    pub ctx: &'tcx mut TranslateCtx,
    pub rpcs: &'tcx mut IndexMap<ArcStr, RpcDef>,
}

impl Visitor for GatherRpcs<'_> {
    fn visit_rpc(&mut self, rpc: &mut Rpc) {
        let name = self.ctx.def.cache(*rpc.name);
        self.rpcs.insert(
            name.clone(),
            RpcDef {
                name,
                req_typ: match &rpc.msg_type {
                    Type::Builtin(b) => DataType::Builtin(*b),
                    Type::Named(u) => DataType::Unresolved(self.ctx.def.cache(u), UnresolvedHint::Message),
                    other => panic!("{other:?} Not supported as rpc type"),
                },
                req_stream: rpc.msg_stream,
                res_typ: match &rpc.ret_type {
                    Type::Builtin(b) => DataType::Builtin(*b),
                    Type::Named(u) => DataType::Unresolved(self.ctx.def.cache(u), UnresolvedHint::Message),
                    other => panic!("{other:?} Not supported as rpc type"),
                },
                res_stream: rpc.ret_stream,
                #[cfg(feature = "descriptors")]
                options: opts(self.ctx, rpc),
                ..Default::default()
            },
        );
    }
}

pub fn rpcs(ctx: &mut TranslateCtx, n: &mut impl AstNode) -> IndexMap<ArcStr, RpcDef> {
    let mut rpcs = indexmap! {};
    n.accept(&mut GatherRpcs { ctx, rpcs: &mut rpcs });
    rpcs
}

pub fn base_def(_ctx: &mut TranslateCtx, name: &str, p: &Proto) -> FileDef {
    FileDef {
        name: _ctx.def.cache(name),
        imports: vec![],
        public_imports: vec![],
        syntax: p.syntax,
        package: Default::default(),
        messages: Default::default(),
        enums: Default::default(),
        services: Default::default(),
        extensions: Default::default(),
        extenders: Default::default(),

        names: Default::default(),
    }
}
