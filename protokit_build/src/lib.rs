use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

pub use anyhow::Result;
use anyhow::bail;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Direction, Graph};
use quote::quote;

mod filegen;

pub use desc::generated::google::protobuf::descriptor::*;

/// A descriptor pool that provides lookup capabilities over a FileDescriptorSet.
/// This is a thin overlay that indexes the descriptors by their fully qualified names.
#[derive(Debug, Default)]
pub struct DescriptorPool {
    /// The underlying FileDescriptorSet from protoc
    pub descriptor_set: FileDescriptorSet,
    /// Index from fully qualified type name to (file_index, type_path)
    /// e.g. ".google.protobuf.Any" -> (file_idx, ["Any"])
    pub types: HashMap<String, TypeLocation>,
    /// Set of types that need to be boxed due to circular references
    pub boxed_types: HashSet<String>,
}

/// Location of a type within the descriptor set
#[derive(Debug, Clone)]
pub struct TypeLocation {
    pub file_idx: usize,
    pub path: Vec<String>,
    pub kind: TypeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    Message,
    Enum,
    Service,
}

impl DescriptorPool {
    pub fn new(descriptor_set: FileDescriptorSet) -> Self {
        let mut pool = Self {
            descriptor_set,
            types: HashMap::new(),
            boxed_types: HashSet::new(),
        };
        pool.build_index();
        pool.detect_cycles();
        pool
    }

    fn build_index(&mut self) {
        // Collect indexing work first to avoid borrow issues
        struct IndexWork {
            file_idx: usize,
            path: Vec<String>,
            fqn: String,
            kind: TypeKind,
        }

        fn collect_message_indices(
            work: &mut Vec<IndexWork>,
            file_idx: usize,
            package: &str,
            parent_path: &[String],
            msg: &DescriptorProto,
        ) {
            let name = msg.name.as_deref().unwrap_or("");
            let mut path = parent_path.to_vec();
            path.push(name.to_string());

            let fqn = if parent_path.is_empty() {
                format!(".{}.{}", package, name)
            } else {
                format!(".{}.{}.{}", package, parent_path.join("."), name)
            };

            work.push(IndexWork {
                file_idx,
                path: path.clone(),
                fqn: fqn.clone(),
                kind: TypeKind::Message,
            });

            // Index nested enums
            for enum_desc in &msg.enum_type {
                let enum_name = enum_desc.name.as_deref().unwrap_or("");
                let enum_fqn = format!("{}.{}", fqn, enum_name);
                let mut enum_path = path.clone();
                enum_path.push(enum_name.to_string());
                work.push(IndexWork {
                    file_idx,
                    path: enum_path,
                    fqn: enum_fqn,
                    kind: TypeKind::Enum,
                });
            }

            // Index nested messages
            for nested_msg in &msg.nested_type {
                collect_message_indices(work, file_idx, package, &path, nested_msg);
            }
        }

        let mut work = Vec::new();

        for (file_idx, file) in self.descriptor_set.file.iter().enumerate() {
            let package = file.package.as_deref().unwrap_or("");

            // Index top-level enums
            for enum_desc in &file.enum_type {
                let name = enum_desc.name.as_deref().unwrap_or("");
                let fqn = format!(".{}.{}", package, name);
                work.push(IndexWork {
                    file_idx,
                    path: vec![name.to_string()],
                    fqn,
                    kind: TypeKind::Enum,
                });
            }

            // Index top-level messages (and nested types)
            for msg_desc in &file.message_type {
                collect_message_indices(&mut work, file_idx, package, &[], msg_desc);
            }

            // Index services
            for svc_desc in &file.service {
                let name = svc_desc.name.as_deref().unwrap_or("");
                let fqn = format!(".{}.{}", package, name);
                work.push(IndexWork {
                    file_idx,
                    path: vec![name.to_string()],
                    fqn,
                    kind: TypeKind::Service,
                });
            }
        }

        // Now apply all the indices
        for item in work {
            self.types.insert(
                item.fqn,
                TypeLocation {
                    file_idx: item.file_idx,
                    path: item.path,
                    kind: item.kind,
                },
            );
        }
    }

    fn detect_cycles(&mut self) {
        // Build a dependency graph
        let mut graph: Graph<String, ()> = Graph::new();
        let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();
        let mut field_counts: HashMap<NodeIndex, usize> = HashMap::new();

        // Add nodes for all messages
        for (fqn, loc) in &self.types {
            if loc.kind == TypeKind::Message {
                let idx = graph.add_node(fqn.clone());
                node_indices.insert(fqn.clone(), idx);
            }
        }

        // Add edges based on field references
        for file in &self.descriptor_set.file {
            let package = file.package.as_deref().unwrap_or("");
            for msg in &file.message_type {
                self.add_message_edges(&mut graph, &node_indices, &mut field_counts, package, &[], msg);
            }
        }

        // Find SCCs using Tarjan's algorithm
        let mut cycles: Vec<HashSet<NodeIndex<DefaultIx>>> = petgraph::algo::tarjan_scc(&graph)
            .into_iter()
            .filter(|scc| scc.len() > 1 || {
                // Check for self-loops
                scc.len() == 1 && graph.find_edge(scc[0], scc[0]).is_some()
            })
            .map(|v| HashSet::from_iter(v.into_iter()))
            .collect();

        // Find nodes to box to break cycles (prefer nodes with more incoming edges)
        let mut to_box = HashSet::new();
        loop {
            let mut counts: HashMap<NodeIndex, usize> = HashMap::new();
            for cycle in &cycles {
                for node in cycle.iter() {
                    *counts.entry(*node).or_default() += 1;
                }
            }

            if let Some(max) = counts
                .iter()
                .filter_map(|a| field_counts.get(a.0).map(|fcount| (a.0, a.1, fcount)))
                .max_by(|a, b| {
                    let ac = graph.edges_directed(*a.0, Direction::Incoming).count();
                    let bc = graph.edges_directed(*b.0, Direction::Incoming).count();
                    (ac * 100 + a.1 * 10 + a.2).cmp(&(bc * 100 + b.1 * 10 + b.2))
                })
            {
                to_box.insert(*max.0);
                cycles.retain_mut(|cycle| !cycle.contains(&max.0));
            } else {
                break;
            }
        }

        // Convert node indices back to type names
        for idx in to_box {
            if let Some(name) = graph.node_weight(idx) {
                self.boxed_types.insert(name.clone());
            }
        }
    }

    fn add_message_edges(
        &self,
        graph: &mut Graph<String, ()>,
        node_indices: &HashMap<String, NodeIndex>,
        field_counts: &mut HashMap<NodeIndex, usize>,
        package: &str,
        parent_path: &[String],
        msg: &DescriptorProto,
    ) {
        let name = msg.name.as_deref().unwrap_or("");
        let src_fqn = if parent_path.is_empty() {
            format!(".{}.{}", package, name)
        } else {
            format!(".{}.{}.{}", package, parent_path.join("."), name)
        };

        let Some(&src_idx) = node_indices.get(&src_fqn) else {
            return;
        };

        // Add edges for fields
        for field in &msg.field {
            if let Some(type_name) = &field.type_name {
                if let Some(&dst_idx) = node_indices.get(type_name) {
                    graph.add_edge(src_idx, dst_idx, ());
                    *field_counts.entry(src_idx).or_default() += 1;
                }
            }
        }

        // Process nested messages
        let mut path = parent_path.to_vec();
        path.push(name.to_string());
        for nested_msg in &msg.nested_type {
            self.add_message_edges(graph, node_indices, field_counts, package, &path, nested_msg);
        }
    }

    pub fn file(&self, idx: usize) -> &FileDescriptorProto {
        &self.descriptor_set.file[idx]
    }

    pub fn is_boxed(&self, type_name: &str) -> bool {
        self.boxed_types.contains(type_name)
    }

    pub fn lookup(&self, type_name: &str) -> Option<&TypeLocation> {
        self.types.get(type_name)
    }

    /// Get a message descriptor by its fully qualified name
    pub fn get_message(&self, fqn: &str) -> Option<&DescriptorProto> {
        let loc = self.types.get(fqn)?;
        if loc.kind != TypeKind::Message {
            return None;
        }
        let file = &self.descriptor_set.file[loc.file_idx];
        self.find_message_in_file(file, &loc.path)
    }

    fn find_message_in_file<'a>(
        &self,
        file: &'a FileDescriptorProto,
        path: &[String],
    ) -> Option<&'a DescriptorProto> {
        if path.is_empty() {
            return None;
        }

        let mut current: Option<&DescriptorProto> = None;
        let first = &path[0];

        for msg in &file.message_type {
            if msg.name.as_deref() == Some(first) {
                current = Some(msg);
                break;
            }
        }

        for name in &path[1..] {
            let msg = current?;
            current = None;
            for nested in &msg.nested_type {
                if nested.name.as_deref() == Some(name) {
                    current = Some(nested);
                    break;
                }
            }
        }

        current
    }

    /// Get an enum descriptor by its fully qualified name
    pub fn get_enum(&self, fqn: &str) -> Option<&EnumDescriptorProto> {
        let loc = self.types.get(fqn)?;
        if loc.kind != TypeKind::Enum {
            return None;
        }
        let file = &self.descriptor_set.file[loc.file_idx];
        self.find_enum_in_file(file, &loc.path)
    }

    fn find_enum_in_file<'a>(
        &self,
        file: &'a FileDescriptorProto,
        path: &[String],
    ) -> Option<&'a EnumDescriptorProto> {
        if path.is_empty() {
            return None;
        }

        if path.len() == 1 {
            // Top-level enum
            for e in &file.enum_type {
                if e.name.as_deref() == Some(&path[0]) {
                    return Some(e);
                }
            }
            return None;
        }

        // Nested enum - find parent message first
        let msg = self.find_message_in_file(file, &path[..path.len() - 1])?;
        let enum_name = &path[path.len() - 1];
        for e in &msg.enum_type {
            if e.name.as_deref() == Some(enum_name) {
                return Some(e);
            }
        }
        None
    }
}

/// Build context for generating Rust code from protobuf files
#[derive(Default, Debug)]
pub struct ProtocContext {
    pub includes: Vec<PathBuf>,
    pub proto_paths: Vec<PathBuf>,
}

impl ProtocContext {
    pub fn include(&mut self, p: impl Into<PathBuf>) {
        self.includes.push(p.into());
    }

    pub fn compile(&mut self, p: impl Into<PathBuf>) -> Result<()> {
        self.proto_paths.push(p.into());
        Ok(())
    }

    pub fn finish(self) -> Result<DescriptorPool> {
        let mut cmd = std::process::Command::new("protoc");

        cmd.arg("--experimental_allow_proto3_optional");
        cmd.arg("--include_imports");

        for i in self.includes {
            cmd.arg(format!("-I{}", i.display()));
        }
        for p in self.proto_paths {
            cmd.arg(format!("{}", p.display()));
        }

        let out_dir = std::env::var("OUT_DIR").unwrap();
        cmd.arg(format!("-o{}/descriptor.bin", out_dir));

        let out = cmd.output().expect("PROTOC invocation failed");
        if !out.status.success() {
            bail!("Protoc error: {}", String::from_utf8_lossy(&out.stderr))
        }

        let data = std::fs::read(Path::new(&out_dir).join("descriptor.bin"))?;
        let desc = binformat::decode::<FileDescriptorSet>(data.as_slice())?;

        Ok(DescriptorPool::new(desc))
    }
}

#[must_use]
#[derive(Default, Debug)]
pub struct Build {
    pub ctx: ProtocContext,
    pub options: filegen::Options,
    pub out_dir: Option<PathBuf>,
}

fn generate(opts: &filegen::Options, pool: &DescriptorPool, out_dir: PathBuf) -> Result<()> {
    std::fs::create_dir_all(&out_dir)?;

    let mut generated_names = vec![];
    for (file_idx, file) in pool.descriptor_set.file.iter().enumerate() {
        let file_name = file.name.as_deref().unwrap_or("unknown.proto");
        let package = file.package.as_deref().unwrap_or("");

        let path = Path::new(file_name);
        let out_name = package.replace('.', "/")
            + "/"
            + path.with_extension("rs").file_name().unwrap().to_str().unwrap();
        let out_path = out_dir.join(&out_name);

        generated_names.push(out_name.clone());
        filegen::generate_file(pool, opts, out_path, file_idx)?;
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
        filegen::generate_mod(out_dir.join(k), opts, v.iter().copied())?;
    }

    Ok(())
}

impl Build {
    pub fn new() -> Self {
        Self::default()
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
        self.options.import_root = Some(core::str::FromStr::from_str(s).unwrap());
        self
    }

    pub fn string_type(mut self, typ: &str) -> Self {
        self.options.string_type = core::str::FromStr::from_str(typ).unwrap();
        self
    }

    pub fn bytes_type(mut self, typ: &str) -> Self {
        self.options.bytes_type = core::str::FromStr::from_str(typ).unwrap();
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

    pub fn generate(self) -> anyhow::Result<()> {
        let out_dir = self
            .out_dir
            .unwrap_or_else(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));
        let pool = self.ctx.finish()?;
        generate(&self.options, &pool, out_dir)
    }
}
