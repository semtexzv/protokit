use core::ops::Deref;
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

use protokit_desc::arcstr::ArcStr;
use protokit_desc::{FileDef, FileSetDef};

use crate::ast::*;
use crate::Parse;

pub mod defs;
pub mod fields;
pub mod imports;
pub mod opts;

#[derive(Debug)]
pub struct TranslateCtx {
    pub current_stack: Vec<PathBuf>,
    pub proto_paths: Vec<PathBuf>,
    pub def: FileSetDef,
    pub file_counter: usize,
    pub errors: Vec<String>,
}

impl Default for TranslateCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslateCtx {
    pub fn new() -> Self {
        // #[cfg(feature = "descriptors")]
        // let def = FileSetDef::from_bytes(include_bytes!("../../../protokit_desc/src/generated/descriptor.bin"));
        // #[cfg(not(feature = "descriptors"))]
        let def = FileSetDef::default();

        TranslateCtx {
            current_stack: vec![],
            proto_paths: vec![],
            def,
            file_counter: 0,
            errors: vec![],
        }
    }
    pub fn include(&mut self, p: impl Into<PathBuf>) {
        self.proto_paths.push(p.into());
    }

    pub fn resolve_file_in_path(&mut self, p: impl AsRef<Path>) -> io::Result<PathBuf> {
        let p = p.as_ref();
        if p.is_absolute() {
            return Ok(p.to_owned());
        }

        let resolved: Vec<PathBuf> = self
            .proto_paths
            .iter()
            .map(|base| base.join(p))
            .filter(|p| p.exists())
            .collect();

        Ok(match &resolved[..] {
            [] => {
                self.error(format!("{p:?} was not found"))?;
                return Err(io::Error::other(format!("{p:?} was not found")));
            }
            [x] => x.clone(),
            [x, ..] => {
                self.error(format!(
                    "{p:?} was found in multiple locations, using first: {resolved:?}"
                ))?;
                x.clone()
            }
        })
    }

    pub fn compile_file(&mut self, path: impl AsRef<Path>) -> io::Result<&FileDef> {
        let path = path.as_ref();
        let name = self.def.cache(path.to_str().unwrap());

        if !self.def.files.contains_key(name.as_str()) {
            return self.do_compile_file(path, name);
        }

        if let Some(f) = self.def.files.get(name.as_str()) {
            Ok(f)
        } else {
            panic!()
        }
    }

    fn do_compile_file(&mut self, path: &Path, name: ArcStr) -> io::Result<&FileDef> {
        let path = self.resolve_file_in_path(path)?;
        if self.current_stack.contains(&path) {
            self.error(format!("Import cycle detected {}", path.to_string_lossy()))?
        }
        let contents = read_to_string(&path)?;
        self.current_stack.push(path);

        let mut parsed = match crate::ast::Proto::parse_format_error(contents.deref()) {
            Ok(parsed) => parsed,
            Err(e) => {
                let mut s = String::new();
                miette::GraphicalReportHandler::new().render_report(&mut s, &e).unwrap();
                panic!("{s}");
            }
        };
        let translated = self.run_passes(&name, &mut parsed);

        self.def.files.insert(name.clone(), translated);
        Ok(self.def.files.get(&name).unwrap())
    }

    fn run_passes(&mut self, name: &str, p: &mut Proto) -> FileDef {
        let mut def = defs::base_def(self, name, p);
        eprintln!("Compiling explicit: {name}");
        {
            imports::ResolveImports { ctx: self }.visit_proto(p);

            let mut defs = defs::FillDefinitions {
                ctx: self,
                unit: &mut def,
                path: String::default(),
            };
            defs.visit_proto(p);
        }
        def.fill_names();
        def.resolve_types(self.def.files.len(), &self.def.files);
        def.resolve_extensions(self.def.files.len(), &mut self.def.files);
        def
    }
    fn error(&mut self, s: impl Into<String>) -> io::Result<()> {
        let s = s.into();
        self.errors.push(s.clone());
        Err(io::Error::other(s))
    }
}

// #[test]
// fn test_translate() {
//     use crate::parser::proto_file;
//     let (_rest, _proto) = proto_file(include_str!("../../../proto/com/book/book.proto")).unwrap();
//     let _tcx = TranslateCtx::new();
// }
