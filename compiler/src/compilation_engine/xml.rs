use lazy_format::lazy_format;

use std::fmt::Display;

use crate::*;

pub trait Xml {
    fn to_xml(&self) -> impl Display;
}

impl Xml for Token<'_> {
    fn to_xml(&self) -> impl Display + '_ {
        let (tag, val): (_, &dyn Display) = match self {
            Keyword(kind) => ("keyword", kind),

            Symbol(kind) => {
                let kind: &dyn Display = match kind {
                    And => &"&amp;",
                    Lt => &"&lt;",
                    Gt => &"&gt;",
                    _ => kind,
                };

                ("symbol", kind)
            }
            StringConstant(s) => ("stringConstant", s),
            IntegerConstant(i) => ("integerConstant", i),
            Identifier(id) => ("identifier", id),
        };

        lazy_format!("<{tag}> {val} </{tag}>")
    }
}
