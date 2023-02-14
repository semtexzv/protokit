use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub use anyhow::Result;
use quote::__private::TokenStream;

use crate::deps::*;

mod deps;
mod filegen;

const REMAPS: &[(&str, &str)] = &[
    ("google/protobuf/any.proto", "root::types::any"),
    ("google/protobuf/empty.proto", "root::types::empty"),
    ("google/protobuf/timestamp.proto", "root::types::timestamp"),
    ("google/protobuf/field_mask.proto", "root::types::field_mask"),
    ("google/protobuf/descriptor.proto", "root::types::descriptor"),
];

#[must_use]
#[derive(Default, Debug)]
pub struct Build {
    ctx: TranslateCtx,
    options: filegen::Options,
    out_dir: Option<PathBuf>,
}

impl Build {
    pub fn new() -> Self {
        let mut this = Self::without_replacements();
        for (from, to) in REMAPS {
            this.ctx.replace_import(from, to);
        }
        this
    }

    pub fn without_replacements() -> Self {
        let ctx = TranslateCtx::default();

        Self {
            ctx,
            ..Default::default()
        }
    }
    pub fn include(mut self, p: impl Into<PathBuf>) -> Self {
        self.ctx.include(p);
        self
    }
    pub fn textformat(mut self, t: bool) -> Self {
        self.options.generate_textformat = t;
        self
    }
    pub fn track_unknowns(mut self, t: bool) -> Self {
        self.options.track_unknowns = t;
        self
    }
    pub fn root(mut self, s: &str) -> Self {
        self.options.import_root = TokenStream::from_str(s).unwrap();
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
        self.ctx.compile_file(&name)?;
        if !self.ctx.errors.is_empty() {
            bail!("Compilation: {:?}", self.ctx.errors)
        }
        Ok(self)
    }

    pub fn generate(self) -> anyhow::Result<()> {
        let out_dir = self
            .out_dir
            .unwrap_or_else(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));

        create_dir_all(&out_dir).unwrap();

        // TODO: Use package name + file name
        let mut generated_names = vec![];
        for (file_idx, file) in self.ctx.def.files.values().enumerate() {
            if self.ctx.replacement.contains_key(file.name.as_str()) {
                continue;
            }
            let path = Path::new(file.name.as_str());
            let file_name =
                file.package.replace('.', "/") + "/" + path.with_extension("rs").file_name().unwrap().to_str().unwrap();
            let out_name = out_dir.join(&file_name);
            generated_names.push(file_name.clone());
            filegen::generate_file(&self.ctx, &self.options, out_name, file_idx, file);
        }

        let dirs: Vec<Vec<&str>> = generated_names.iter().map(|v| v.split('/').collect()).collect();

        let mut subdirs = HashMap::new();

        // Generate a valid module file in every subdirectory
        for path in &dirs {
            for i in 0 .. path.len() {
                subdirs
                    .entry(path[0 .. i].join("/"))
                    .or_insert_with(HashSet::new)
                    .insert(path[i]);
            }
        }

        for (k, v) in &subdirs {
            eprintln!("Creating module in: {:?}", out_dir.join(k));
            filegen::generate_mod(out_dir.join(k), &self.options, v.iter().copied());
        }

        #[cfg(feature = "descriptors")]
        filegen::generate_descriptor(&self.ctx, out_dir.join("descriptor.bin"));
        Ok(())
    }
}
