use glob::glob;

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::compilation::{Analyzer, VMWriter, XMLAnalyzer};
use crate::parse::parse;
use crate::tokenize::tokenize;

pub enum Mode {
    Xml,
    Vm,
}

pub enum Source {
    File(String),
    Directory(String),
}

pub struct Compiler {
    source: Source,
    mode: Mode,
}

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Compiler {
    pub fn new(source: Source, mode: Mode) -> Self {
        Self { source, mode }
    }

    pub fn compile(&self) -> Result {
        match &self.source {
            Source::File(file) => self.compile_file(file),
            Source::Directory(dir) => {
                let path = PathBuf::from(dir).join("*.jack");
                let path = path.to_str().unwrap();

                for entry in glob(path)? {
                    self.compile_file(entry?.to_str().unwrap())?;
                }

                Ok(())
            }
        }
    }
    fn compile_file(&self, name: &str) -> Result {
        let input = fs::read_to_string(name)?;
        let tokens = tokenize(&input).collect::<Result<Vec<_>>>()?;
        let ast = parse(tokens)?;

        match self.mode {
            Mode::Xml => {
                let output = Path::new(name).with_extension("C.xml");
                let mut output = BufWriter::new(File::create(output)?);
                let mut analyzer = XMLAnalyzer::new(&mut output);
                analyzer.analyze(&ast)?;
            }
            Mode::Vm => {
                let output = Path::new(name).with_extension("vm");
                let mut output = BufWriter::new(File::create(output)?);
                let mut vm_writer = VMWriter::new(&mut output);
                vm_writer.analyze(&ast)?;
            }
        }

        Ok(())
    }
}
