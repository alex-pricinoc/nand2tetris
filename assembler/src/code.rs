/// returns 3 bits
pub fn dest(mnemonic: &str) -> u8 {
    match mnemonic {
        "null" => 0b000,
        "M" => 0b001,
        "D" => 0b010,
        "MD" => 0b011,
        "A" => 0b100,
        "AM" => 0b101,
        "AD" => 0b110,
        "AMD" => 0b111,
        d => panic!("unrecognized dest mnemonic: {d}"),
    }
}

/// returns 7 bits
pub fn comp(mnemonic: &str) -> u8 {
    match mnemonic {
        "0" => 0b101010,
        "1" => 0b111111,
        "-1" => 0b111010,
        "D" => 0b001100,
        "A" => 0b110000,
        "!D" => 0b001101,
        "!A" => 0b110011,
        "-D" => 0b001111,
        "-A" => 0b110011,
        "D+1" => 0b011111,
        "A+1" => 0b110111,
        "D-1" => 0b001110,
        "A-1" => 0b110010,
        "D+A" => 0b000010,
        "D-A" => 0b010011,
        "A-D" => 0b000111,
        "D&A" => 0b000000,
        "D|A" => 0b010101,
        "M" => 0b1110000,
        "!M" => 0b1110001,
        "-M" => 0b1110011,
        "M+1" => 0b1110111,
        "M-1" => 0b1110010,
        "D+M" => 0b1000010,
        "D-M" => 0b1010011,
        "M-D" => 0b1000111,
        "D&M" => 0b1000000,
        "D|M" => 0b1010101,
        m => panic!("unrecognized comp mnemonic: {m}"),
    }
}

/// returns 3 bits
pub fn jump(mnemonic: &str) -> u8 {
    match mnemonic {
        "null" => 0b000,
        "JGT" => 0b001,
        "JEQ" => 0b010,
        "JGE" => 0b011,
        "JLT" => 0b100,
        "JNE" => 0b101,
        "JLE" => 0b110,
        "JMP" => 0b111,
        j => panic!("unrecognized jump mnemonic: {j}"),
    }
}
