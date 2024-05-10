use crate::{code, Instruction, SymbolTable, Token};
use std::error::Error;
use std::io::Write;

pub fn run(input: &str, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut symbol_table = SymbolTable::new();

    // first pass
    let mut next_address = 0;

    for instruction in instructions(input) {
        match instruction {
            Instruction::Label(symbol) => symbol_table.add_entry(symbol, next_address),
            _ => next_address += 1,
        }
    }

    // second pass
    let mut var_address = 16;

    for instruction in instructions(input) {
        match instruction {
            Instruction::Address(Token::Number(number)) => writeln!(out, "{number:016b}")?,
            Instruction::Address(Token::Symbol(symbol)) => {
                if let Some(address) = symbol_table.get_address(&symbol) {
                    writeln!(out, "{address:016b}")?;
                } else {
                    symbol_table.add_entry(symbol, var_address);

                    writeln!(out, "{var_address:016b}")?;
                    var_address += 1;
                }
            }
            Instruction::Command(dest, comp, jump) => {
                let dest = code::dest(dest);
                let comp = code::comp(comp);
                let jump = code::jump(jump);

                writeln!(out, "111{comp:07b}{dest:03b}{jump:03b}")?;
            }
            Instruction::Label(..) => {}
        };
    }

    Ok(())
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .map(|l| {
            let offset = l.find("//").unwrap_or(l.len());
            let line = l[0..offset].trim();

            line
        })
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse().unwrap())
}
