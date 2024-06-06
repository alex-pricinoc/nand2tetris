use std::fmt;
use std::io::{self, Write};

type Result<T = ()> = io::Result<T>;

#[derive(Debug, Clone, Copy)]
pub enum Segment {
    Const,
    Arg,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Clone, Copy)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

pub fn write_push(mut to: impl Write, seg: Segment, index: u16) -> Result {
    writeln!(to, "push {seg} {index}")
}

pub fn write_pop(mut to: impl Write, seg: Segment, index: u16) -> Result {
    writeln!(to, "pop {seg} {index}")
}

pub fn write_arithmetic(mut to: impl Write, cmd: Arithmetic) -> Result {
    writeln!(to, "{cmd}")
}

pub fn write_label(mut to: impl Write, label: impl fmt::Display) -> Result {
    writeln!(to, "label {label}")
}

pub fn write_goto(mut to: impl Write, label: impl fmt::Display) -> Result {
    writeln!(to, "goto {label}")
}

pub fn write_if(mut to: impl Write, label: impl fmt::Display) -> Result {
    writeln!(to, "if-goto {label}")
}

pub fn write_call(mut to: impl Write, name: impl fmt::Display, args: usize) -> Result {
    writeln!(to, "call {name} {args}")
}

pub fn write_function(mut to: impl Write, name: impl fmt::Display, locals: usize) -> Result {
    writeln!(to, "function {name} {locals}")
}

pub fn write_return(mut to: impl Write) -> Result {
    writeln!(to, "return")
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Segment::*;
        match self {
            Const => "constant",
            Arg => "argument",
            Local => "local",
            Static => "static",
            This => "this",
            That => "that",
            Pointer => "pointer",
            Temp => "temp",
        }
        .fmt(f)
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Arithmetic::*;
        match self {
            Add => "add",
            Sub => "sub",
            Neg => "neg",
            Eq => "eq",
            Gt => "gt",
            Lt => "lt",
            And => "and",
            Or => "or",
            Not => "not",
        }
        .fmt(f)
    }
}
