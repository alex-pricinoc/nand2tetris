#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

use crate::{Op, Segment};
use assembly::{Comp::*, CompValue::*, Dest::*, Instruction::*, Jump::*, Token::*};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_id(inc: usize) -> usize {
    COUNTER.fetch_add(inc, Ordering::Relaxed)
}

pub fn write(command: crate::Instruction) -> Vec<assembly::Instruction> {
    use crate::Instruction::*;

    match command {
        Arithmetic(op) => write_arithmetic(op),
        Push(seg, i) => write_push(seg, i),
        Pop(seg, i) => write_pop(seg, i),
        _ => unimplemented!(),
    }
}

fn write_push(seg: Segment, i: u16) -> Vec<assembly::Instruction> {
    use Segment::*;

    match seg {
        Constant => {
            vec![
                Address(Number(i)),
                Command(D, Literal(RegA), NOJ),
                Address(Symbol("SP".into())),
                Command(A, Literal(RegM), NOJ),
                Command(M, Literal(RegD), NOJ),
                Address(Symbol("SP".into())),
                Command(M, Add(RegM, One), NOJ),
            ]
        }
        Local | Argument | This | That => {
            let ptr = match seg {
                Argument => "ARG",
                Local => "LCL",
                This => "THIS",
                That => "THAT",
                _ => unreachable!(),
            };

            vec![
                // addr <- segmentPointer + i
                Address(Symbol(ptr.into())),
                Command(D, Literal(RegM), NOJ),
                Address(Number(i)),
                Command(D, Add(RegA, RegD), NOJ),
                // RAM[SP] <- RAM[addr]
                Command(A, Literal(RegD), NOJ),
                Command(D, Literal(RegM), NOJ),
                Address(Symbol("SP".into())),
                Command(A, Literal(RegM), NOJ),
                Command(M, Literal(RegD), NOJ),
                // SP++
                Address(Symbol("SP".into())),
                Command(M, Add(RegM, One), NOJ),
            ]
        }

        Temp => {
            vec![
                // addr <- 5 + i
                Address(Number(5)),
                Command(D, Literal(RegA), NOJ),
                Address(Number(i)),
                Command(D, Add(RegA, RegD), NOJ),
                // RAM[SP] <- RAM[addr]
                Command(A, Literal(RegD), NOJ),
                Command(D, Literal(RegM), NOJ),
                Address(Symbol("SP".into())),
                Command(A, Literal(RegM), NOJ),
                Command(M, Literal(RegD), NOJ),
                // SP++
                Address(Symbol("SP".into())),
                Command(M, Add(RegM, One), NOJ),
            ]
        }
        Pointer => todo!(),
        Static => todo!(),
    }
}

fn write_pop(seg: Segment, i: u16) -> Vec<assembly::Instruction> {
    use Segment::*;

    match seg {
        Local | Argument | This | That => {
            let ptr = match seg {
                Argument => "ARG",
                Local => "LCL",
                This => "THIS",
                That => "THAT",
                _ => unreachable!(),
            };

            vec![
                // addr <- segmentPointer + i
                Address(Symbol(ptr.into())),
                Command(D, Literal(RegM), NOJ),
                Address(Number(i)),
                Command(D, Add(RegA, RegD), NOJ),
                Address(Symbol("R13".into())),
                Command(M, Literal(RegD), NOJ),
                // SP--
                Address(Symbol("SP".into())),
                Command(AM, Sub(RegM, One), NOJ),
                // RAM[addr] <- RAM[SP]
                Command(D, Literal(RegM), NOJ),
                Address(Symbol("R13".into())),
                Command(A, Literal(RegM), NOJ),
                Command(M, Literal(RegD), NOJ),
            ]
        }
        Temp => {
            vec![
                // addr <- 5 + i
                Address(Number(5)),
                Command(D, Literal(RegA), NOJ),
                Address(Number(i)),
                Command(D, Add(RegA, RegD), NOJ),
                Address(Symbol("R13".into())),
                Command(M, Literal(RegD), NOJ),
                // SP--
                Address(Symbol("SP".into())),
                Command(AM, Sub(RegM, One), NOJ),
                // RAM[addr] <- RAM[SP]
                Command(D, Literal(RegM), NOJ),
                Address(Symbol("R13".into())),
                Command(A, Literal(RegM), NOJ),
                Command(M, Literal(RegD), NOJ),
            ]
        }
        Static => todo!(),
        Pointer => unimplemented!(),
        Constant => unreachable!(),
    }
}

fn write_arithmetic(op: Op) -> Vec<assembly::Instruction> {
    match op {
        Op::Add => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Add(RegM, RegD), NOJ),
            Command(M, Literal(RegD), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Sub => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Sub(RegM, RegD), NOJ),
            Command(M, Literal(RegD), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Neg => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(M, Negative(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Eq => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Sub(RegM, RegD), NOJ),
            Address(Symbol(format!("IF_TRUE_{}", get_id(0)))),
            Command(NULL, Literal(RegD), JEQ),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Literal(Zero), NOJ),
            Address(Symbol(format!("END_{}", get_id(0)))),
            Command(NULL, Literal(Zero), JMP),
            Label(format!("IF_TRUE_{}", get_id(0))),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Negative(One), NOJ),
            Label(format!("END_{}", get_id(1))),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Gt => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Sub(RegM, RegD), NOJ),
            Address(Symbol(format!("IF_TRUE_{}", get_id(0)))),
            Command(NULL, Literal(RegD), JGT),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Literal(Zero), NOJ),
            Address(Symbol(format!("END_{}", get_id(0)))),
            Command(NULL, Literal(Zero), JMP),
            Label(format!("IF_TRUE_{}", get_id(0))),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Negative(One), NOJ),
            Label(format!("END_{}", get_id(1))),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Lt => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Sub(RegM, RegD), NOJ),
            Address(Symbol(format!("IF_TRUE_{}", get_id(0)))),
            Command(NULL, Literal(RegD), JLT),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Literal(Zero), NOJ),
            Address(Symbol(format!("END_{}", get_id(0)))),
            Command(NULL, Literal(Zero), JMP),
            Label(format!("IF_TRUE_{}", get_id(0))),
            Address(Symbol("SP".into())),
            Command(A, Literal(RegM), NOJ),
            Command(M, Negative(One), NOJ),
            Label(format!("END_{}", get_id(1))),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::And => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, And(RegM, RegD), NOJ),
            Command(M, Literal(RegD), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Or => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Literal(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(D, Or(RegM, RegD), NOJ),
            Command(M, Literal(RegD), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
        Op::Not => vec![
            Address(Symbol("SP".into())),
            Command(AM, Sub(RegM, One), NOJ),
            Command(M, Not(RegM), NOJ),
            Address(Symbol("SP".into())),
            Command(M, Add(RegM, One), NOJ),
        ],
    }
}

mod assembly {
    use std::fmt;

    #[derive(Debug)]
    pub enum Token {
        Symbol(String),
        Number(u16),
    }

    #[derive(Debug)]
    pub enum Instruction {
        Address(Token),
        Command(Dest, Comp, Jump),
        Label(String),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Dest {
        NULL,
        M,
        D,
        MD,
        A,
        AM,
        AD,
        AMD,
    }

    #[derive(Debug)]
    pub enum Jump {
        NOJ,
        JGT,
        JEQ,
        JGE,
        JLT,
        JNE,
        JLE,
        JMP,
    }

    #[derive(Debug)]
    pub enum CompValue {
        RegA,
        RegD,
        RegM,
        Zero,
        One,
    }

    #[derive(Debug)]
    pub enum Comp {
        Literal(CompValue),
        Not(CompValue),
        Negative(CompValue),
        Add(CompValue, CompValue),
        Sub(CompValue, CompValue),
        And(CompValue, CompValue),
        Or(CompValue, CompValue),
    }

    impl fmt::Display for Comp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Comp::*;

            match self {
                Literal(val) => write!(f, "{val}"),
                Not(val) => write!(f, "!{val}"),
                Negative(val) => write!(f, "-{val}"),
                Add(a, b) => write!(f, "{a}+{b}"),
                Sub(a, b) => write!(f, "{a}-{b}"),
                And(a, b) => write!(f, "{a}&{b}"),
                Or(a, b) => write!(f, "{a}|{b}"),
            }
        }
    }

    impl fmt::Display for Dest {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Dest::*;

            match self {
                M => write!(f, "M="),
                D => write!(f, "D="),
                MD => write!(f, "MD="),
                A => write!(f, "A="),
                AM => write!(f, "AM="),
                AD => write!(f, "AD="),
                AMD => write!(f, "AMD="),
                _ => Ok(()),
            }
        }
    }

    impl fmt::Display for Jump {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Jump::*;

            match self {
                JGT => write!(f, ";JGT"),
                JEQ => write!(f, ";JEQ"),
                JGE => write!(f, ";JGE"),
                JLT => write!(f, ";JLT"),
                JNE => write!(f, ";JNE"),
                JLE => write!(f, ";JLE"),
                JMP => write!(f, ";JMP"),
                _ => Ok(()),
            }
        }
    }

    impl fmt::Display for CompValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use CompValue::*;

            match self {
                RegA => write!(f, "A"),
                RegD => write!(f, "D"),
                RegM => write!(f, "M"),
                Zero => write!(f, "0"),
                One => write!(f, "1"),
            }
        }
    }

    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Instruction::Address(Token::Symbol(sym)) => write!(f, "@{sym}"),
                Instruction::Address(Token::Number(i)) => write!(f, "@{i}"),
                Instruction::Command(dest, comp, jump) => write!(f, "{dest}{comp}{jump}"),
                Instruction::Label(label) => write!(f, "({label})"),
            }
        }
    }
}
