use core::str::FromStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::usize;

pub use anyhow::Result;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Direction, Graph};
use quote::__private::TokenStream;
use quote::quote;

use crate::deps::*;

mod deps;
mod filegen;

#[cfg(all(not(feature = "protoc"), not(feature = "parser")))]
compile_error!("Either enable 'protoc' (to use system protoc) or 'parser' (to use builtin parser) feature");

#[cfg(all(feature = "protoc", feature = "parser"))]
compile_error!("Either disable 'protoc' or 'parser' feature");

const REMAPS: &[(&str, &str)] = &[
    // ("google/protobuf/any.proto", "root::types::any"),
    // ("google/protobuf/empty.proto", "root::types::empty"),
    // ("google/protobuf/timestamp.proto", "root::types::timestamp"),
    // ("google/protobuf/field_mask.proto", "root::types::field_mask"),
    // ("google/protobuf/descriptor.proto", "root::types::descriptor"),
];

#[cfg(feature = "parser")]
#[derive(Default, Debug)]
pub struct ParserContext {
    ctx: protokit_proto::translate::TranslateCtx,
}

#[cfg(feature = "parser")]
impl ParserContext {
    pub fn include(&mut self, p: impl Into<PathBuf>) {
        self.ctx.include(p)
    }
    pub fn compile(&mut self, p: impl Into<PathBuf>) -> Result<()> {
        self.ctx.compile_file(p.into())?;
        Ok(())
    }
    pub fn finish(self) -> Result<FileSetDef> {
        Ok(self.ctx.def)
    }
}

#[cfg(feature = "protoc")]
#[derive(Default, Debug)]
pub struct ProtocContext {
    pub includes: Vec<PathBuf>,
    pub proto_paths: Vec<PathBuf>,
}

#[cfg(feature = "protoc")]
impl ProtocContext {
    pub fn include(&mut self, p: impl Into<PathBuf>) {
        self.includes.push(p.into());
    }
    pub fn compile(&mut self, p: impl Into<PathBuf>) -> Result<()> {
        self.proto_paths.push(p.into());
        Ok(())
    }
    pub fn finish(self) -> Result<FileSetDef> {
        let mut cmd = std::process::Command::new("protoc");

        cmd.arg("--experimental_allow_proto3_optional");
        cmd.arg("--include_imports");

        for i in self.includes {
            cmd.arg(format!("-I{}", i.display()));
        }
        for p in self.proto_paths {
            cmd.arg(format!("{}", p.display()));
        }

        cmd.arg(format!("-o{}/descriptor.bin", std::env::var("OUT_DIR").unwrap()));
        let out = cmd.output().expect("PROTOC invocation failed");
        if !out.status.success() {
            bail!("Protoc error: {}", String::from_utf8_lossy(&out.stderr))
        }

        let data = std::fs::read(Path::new(&std::env::var("OUT_DIR").unwrap()).join("descriptor.bin")).unwrap();
        let desc = protokit_binformat::decode::<protokit_desc::FileDescriptorSet>(data.as_slice())?;

        Ok(FileSetDef::from_descriptor(desc))
    }
}

#[must_use]
#[derive(Default, Debug)]
pub struct Build {
    #[cfg(feature = "parser")]
    pub ctx: ParserContext,
    #[cfg(feature = "protoc")]
    pub ctx: ProtocContext,
    pub options: filegen::Options,
    pub out_dir: Option<PathBuf>,
}

fn generate(opts: &mut filegen::Options, set: &protokit_desc::FileSetDef, out_dir: PathBuf) -> Result<()> {
    create_dir_all(&out_dir).unwrap();

    let mut graph = Graph::new();
    let mut fields = HashMap::<NodeIndex, usize>::new();
    for (fidx, file) in set.files.values().enumerate() {
        for (midx, msg) in file.messages.values().enumerate() {
            eprintln!("msg: {:?}", msg.name);
            let src = local_to_global(fidx, LOCAL_DEFID_MSG | (midx as u32));
            // let src = resolve_name(set, src).unwrap();
            let src = graph.add_node(src);
            for field in msg.fields.by_number.values() {
                match &field.typ {
                    DataType::Unresolved(_, _) => {
                        panic!("Should be resolved");
                    }
                    DataType::Message(msg) => {
                        // let msg = resolve_name(set, *msg).unwrap();
                        let dst = graph.add_node(*msg);
                        graph.add_edge(src, dst, ());
                        *fields.entry(src).or_default() += 1;
                    }
                    DataType::Group(msg) => {
                        // let msg = resolve_name(set, *msg).unwrap();
                        let dst = graph.add_node(*msg);
                        graph.add_edge(src, dst, ());
                        *fields.entry(src).or_default() += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    let mut cycles: Vec<HashSet<NodeIndex<DefaultIx>>> = petgraph::algo::tarjan_scc(&graph)
        .into_iter()
        .map(|v| HashSet::from_iter(v.into_iter()))
        .collect();

    let mut to_remove = HashSet::new();

    loop {
        let mut counts: HashMap<NodeIndex, usize> = HashMap::new();
        for cycle in &cycles {
            for node in cycle.iter() {
                *counts.entry(*node).or_default() += 1;
            }
        }
        if let Some(max) = counts
            .iter()
            .filter_map(|a| fields.get(a.0).map(|fcount| (a.0, a.1, fcount)))
            .max_by(|a, b| {
                let ac = graph.edges_directed(*a.0, Direction::Incoming).count();
                let bc = graph.edges_directed(*b.0, Direction::Incoming).count();
                (ac * 100 + a.1 * 10 + a.2).cmp(&(bc * 100 + b.1 * 10 + b.2))
            })
        {
            to_remove.insert(*max.0);
            cycles.retain_mut(|cycle| !cycle.contains(&max.0))
        } else {
            break;
        }
    }

    let nodes = to_remove
        .into_iter()
        .map(|item| graph.node_weight(item).cloned().unwrap())
        .collect::<HashSet<_>>();

    // panic!("TO REMOVE: {:?}", nodes);

    opts.force_box = nodes;

    // TODO: Use package name + file name
    let mut generated_names = vec![];
    for (_, file) in set.files.values().enumerate() {
        // if self.ctx.replacement.contains_key(file.name.as_str()) {
        //     continue;
        // }
        let path = Path::new(file.name.as_str());
        let file_name =
            file.package.replace('.', "/") + "/" + path.with_extension("rs").file_name().unwrap().to_str().unwrap();
        let out_name = out_dir.join(&file_name);
        generated_names.push(file_name.clone());
        filegen::generate_file(set, opts, out_name, file).unwrap();
    }

    let dirs: Vec<Vec<&str>> = generated_names.iter().map(|v| v.split('/').collect()).collect();

    let mut subdirs = BTreeMap::new();

    // Generate a valid module file in every subdirectory
    for path in &dirs {
        for i in 0..path.len() {
            subdirs
                .entry(path[0..i].join("/"))
                .or_insert_with(BTreeSet::new)
                .insert(path[i]);
        }
    }

    for (k, v) in &subdirs {
        eprintln!("Creating module in: {:?}", out_dir.join(k));
        filegen::generate_mod(out_dir.join(k), opts, v.iter().copied())?;
    }

    // #[cfg(feature = "descriptors")]
    // filegen::generate_descriptor(&self.ctx, out_dir.join("descriptor.bin"));
    Ok(())
}

impl Build {
    pub fn new() -> Self {
        let mut this = Self::without_replacements();
        for (from, to) in REMAPS {
            this.options.replace_import(from, to);
        }
        this
    }

    pub fn without_replacements() -> Self {
        Self {
            ctx: Default::default(),
            ..Default::default()
        }
    }
    pub fn include(mut self, p: impl Into<PathBuf>) -> Self {
        self.ctx.include(p);
        self
    }

    pub fn borrow_bufs(mut self) -> Self {
        self.options.generics.buf_arg = Some(quote! { 'buf });
        self.options.string_type = quote! {&'buf str };
        self.options.bytes_type = quote! {&'buf [u8] };
        self.options.unknown_type = quote! { binformat::UnknownFieldsBorrow<'buf> };
        self.options.protoattrs.push(quote! { borrow = 'buf });
        self
    }

    pub fn allocator_api(mut self) -> Self {
        self.options.generics.alloc_arg = Some(quote! { A });
        self
    }

    pub fn track_unknowns(mut self, t: bool) -> Self {
        self.options.track_unknowns = t;
        self
    }
    pub fn root(mut self, s: &str) -> Self {
        self.options.import_root = Some(TokenStream::from_str(s).unwrap());
        self
    }
    pub fn string_type(mut self, typ: &str) -> Self {
        self.options.string_type = TokenStream::from_str(typ).unwrap();
        self
    }
    pub fn bytes_type(mut self, typ: &str) -> Self {
        self.options.bytes_type = TokenStream::from_str(typ).unwrap();
        self
    }
    pub fn out_dir(mut self, p: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(p.into());
        self
    }

    pub fn compile(mut self, name: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let name = name.into();
        self.ctx.compile(name)?;
        Ok(self)
    }

    pub fn generate(mut self) -> anyhow::Result<()> {
        let out_dir = self
            .out_dir
            .unwrap_or_else(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));
        generate(&mut self.options, &self.ctx.finish()?, out_dir)
    }
}
