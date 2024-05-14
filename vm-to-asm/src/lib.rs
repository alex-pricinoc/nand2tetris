// use std::error::Error;
// use std::io::Write;

use std::fmt;

pub mod code_writer;
pub mod parser;

#[derive(Debug)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CommandType::*;

        match self {
            Arithmetic => todo!(),
            Push => write!(f, "push"),
            Pop => write!(f, "pop"),
            Label => todo!(),
            Goto => todo!(),
            If => todo!(),
            Function => todo!(),
            Return => todo!(),
            Call => todo!(),
        }
    }
}
