use parser::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use symbol_table::SymbolTable;

mod code;
mod parser;
mod symbol_table;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => Err("Didn't get a file path")?,
        };

        Ok(Config { file_path })
    }
}

pub fn run(config: Config, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut symbol_table = SymbolTable::new();

    let mut parser = Parser::build(File::open(&config.file_path)?)?;

    // first pass
    while parser.has_more_commands() {
        parser.advance();

        if parser.command_type() == "L_COMMAND" {
            let symbol = parser.symbol();
            let address = parser.next_address();
            symbol_table.add_entry(symbol.to_string(), address);
        }
    }

    // second pass
    parser = Parser::build(File::open(&config.file_path)?)?;

    let mut var_address = 16;

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            "A_COMMAND" => {
                let symbol = parser.symbol();

                if let Ok(number) = symbol.parse::<u16>() {
                    writeln!(out, "{:016b}", number)?;
                } else if let Some(address) = symbol_table.get_address(symbol) {
                    writeln!(out, "{:016b}", address)?;
                } else {
                    symbol_table.add_entry(symbol.to_owned(), var_address);

                    writeln!(out, "{:016b}", var_address)?;
                    var_address += 1;
                }
            }
            "L_COMMAND" => {}
            "C_COMMAND" => {
                let dest = code::dest(parser.dest());
                let comp = code::comp(parser.comp());
                let jump = code::jump(parser.jump());

                writeln!(out, "111{comp:07b}{dest:03b}{jump:03b}")?;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
