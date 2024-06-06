use std::collections::HashMap;

use crate::parse::ast;
use ast::Type;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kind {
    Static,
    This,  // field
    Local, // var
    Argument,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub r#type: Type,
    pub kind: Kind,
    pub index: usize,
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    globals: HashMap<String, Symbol>,
    locals: HashMap<String, Symbol>,
    var_counts: HashMap<Kind, usize>,
}

impl SymbolTable {
    /// Starts a new subroutine scope (i.e., resets the subroutine's symbol table).
    pub fn start_subroutine(&mut self) {
        self.locals.clear();
        self.var_counts.remove(&Kind::Argument);
        self.var_counts.remove(&Kind::Local);
    }

    pub fn define(&mut self, r#type: &Type, kind: Kind, name: &str) {
        let index = self.next_index(kind);

        use Kind::*;
        let symbols = match kind {
            Static | This => &mut self.globals,
            Local | Argument => &mut self.locals,
        };

        assert!(
            !symbols.contains_key(name),
            "symbol `{name}` is already defined"
        );

        symbols.insert(
            name.to_owned(),
            Symbol {
                name: name.to_owned(),
                r#type: r#type.clone(),
                kind,
                index,
            },
        );
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.locals.get(name).or_else(|| self.globals.get(name))
    }

    fn next_index(&mut self, kind: Kind) -> usize {
        let entry = self.var_counts.entry(kind).or_default();
        let index = *entry;
        *entry += 1;
        index
    }

    pub fn fields(&self) -> usize {
        self.globals
            .values()
            .filter(|v| v.kind == Kind::This)
            .count()
    }
}
