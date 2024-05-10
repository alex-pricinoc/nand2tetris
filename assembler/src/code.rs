use crate::{Comp, CompValue, Dest, Jump};

pub fn dest(dest: Dest) -> u8 {
    use Dest::*;

    match dest {
        NULL => 0,
        M => 1,
        D => 2,
        MD => 3,
        A => 4,
        AM => 5,
        AD => 6,
        AMD => 7,
    }
}

/// returns 7 bits
pub fn comp(comp: Comp) -> u8 {
    use Comp::*;
    use CompValue::*;

    match comp {
        Literal(Zero) => 0b010_1010,
        Literal(One) => 0b011_1111,
        Negative(One) => 0b011_1010,
        Literal(RegD) => 0b000_1100,
        Literal(RegA) => 0b011_0000,
        Literal(RegM) => 0b111_0000,
        Not(RegD) => 0b000_1101,
        Not(RegA) => 0b011_0001,
        Not(RegM) => 0b111_0001,
        Negative(RegD) => 0b000_1111,
        Negative(RegA) => 0b011_0011,
        Negative(RegM) => 0b111_0011,
        Add(RegD, One) => 0b001_1111,
        Add(RegA, One) => 0b011_0111,
        Add(RegM, One) => 0b111_0111,
        Sub(RegD, One) => 0b000_1110,
        Sub(RegA, One) => 0b011_0010,
        Sub(RegM, One) => 0b111_0010,
        Add(RegD, RegA) => 0b000_0010,
        Add(RegD, RegM) => 0b100_0010,
        Sub(RegD, RegA) => 0b001_0011,
        Sub(RegD, RegM) => 0b101_0011,
        Sub(RegA, RegD) => 0b000_0111,
        Sub(RegM, RegD) => 0b100_0111,
        And(RegD, RegA) => 0b000_0000,
        And(RegD, RegM) => 0b100_0000,
        Or(RegA, RegD) => 0b001_0101,
        Or(RegD, RegM) => 0b101_0101,
        _ => panic!("Invalid operation encountered: {comp:?}"),
    }
}

pub fn jump(jump: Jump) -> u8 {
    use Jump::*;

    match jump {
        NULL => 0,
        JGT => 1,
        JEQ => 2,
        JGE => 3,
        JLT => 4,
        JNE => 5,
        JLE => 6,
        JMP => 7,
    }
}
