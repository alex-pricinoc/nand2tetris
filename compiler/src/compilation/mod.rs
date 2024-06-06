use crate::parse::ast;

pub use vm::VMWriter;
pub use xml::XMLAnalyzer;

mod vm;
mod xml;

pub trait Analyzer {
    type Result;
    fn analyze(&mut self, tree: &ast::Ast) -> Self::Result;
}
