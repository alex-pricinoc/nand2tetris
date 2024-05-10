#![allow(clippy::upper_case_acronyms)]

use std::convert::Infallible;
use std::str::FromStr;

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

#[derive(Debug)]
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
    NULL,
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

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().expect("instruction is not empty");

        let cmd = match c {
            '@' => {
                if let Ok(number) = chars.as_str().parse() {
                    Instruction::Address(Token::Number(number))
                } else {
                    Instruction::Address(Token::Symbol(chars.as_str().to_owned()))
                }
            }
            '(' => {
                chars.next_back().unwrap();
                Instruction::Label(chars.as_str().to_owned())
            }
            _ => {
                let start = s.find('=').map_or(0, |i| i + 1);
                let end = s.find(';').unwrap_or(s.len());

                let dest = match &s[..start] {
                    "M=" => Dest::M,
                    "D=" => Dest::D,
                    "MD=" => Dest::MD,
                    "A=" => Dest::A,
                    "AM=" => Dest::AM,
                    "AD=" => Dest::AD,
                    "AMD=" => Dest::AMD,
                    _ => Dest::NULL,
                };

                use Comp::*;
                use CompValue::*;
                let comp = match &s[start..end] {
                    "0" => Literal(Zero),
                    "1" => Literal(One),
                    "-1" => Negative(One),
                    "D" => Literal(RegD),
                    "A" => Literal(RegA),
                    "!D" => Not(RegD),
                    "!A" => Not(RegA),
                    "-D" => Negative(RegD),
                    "-A" => Negative(RegA),
                    "D+1" => Add(RegD, One),
                    "A+1" => Add(RegA, One),
                    "D-1" => Sub(RegD, One),
                    "A-1" => Sub(RegA, One),
                    "D+A" => Add(RegD, RegA),
                    "D-A" => Sub(RegD, RegA),
                    "A-D" => Sub(RegA, RegD),
                    "D&A" => And(RegD, RegA),
                    "D|A" => Or(RegD, RegA),
                    "M" => Literal(RegM),
                    "!M" => Not(RegM),
                    "-M" => Negative(RegM),
                    "M+1" => Add(RegM, One),
                    "M-1" => Sub(RegM, One),
                    "D+M" => Add(RegD, RegM),
                    "D-M" => Sub(RegD, RegM),
                    "M-D" => Sub(RegM, RegD),
                    "D&M" => And(RegD, RegM),
                    "D|M" => Or(RegD, RegM),
                    i => unreachable!("{i}"),
                };

                let jump = match &s[end..] {
                    ";JGT" => Jump::JGT,
                    ";JEQ" => Jump::JEQ,
                    ";JGE" => Jump::JGE,
                    ";JLT" => Jump::JLT,
                    ";JNE" => Jump::JNE,
                    ";JLE" => Jump::JLE,
                    ";JMP" => Jump::JMP,
                    _ => Jump::NULL,
                };

                Instruction::Command(dest, comp, jump)
            }
        };

        Ok(cmd)
    }
}
