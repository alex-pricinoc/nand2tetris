#![allow(unused_imports)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::{env, fs, io, process};
use vm_to_asm::{code_writer::CodeWriter, parser::Parser, CommandType};

fn main() -> io::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("help: vm-to-asm <input vm file>");
        process::exit(1);
    }

    let file = Path::new(&args[0]);
    let dest = file.with_extension("asm");
    let dest = dest.to_str().unwrap();

    let mut code_writer = CodeWriter::new();
    code_writer.set_file_name(dest);

    let input = fs::read_to_string(file)?;

    let mut parser = Parser::new(&input);

    code_writer.set_module_name(file.file_name().unwrap().to_str().unwrap());
    while parser.has_more_commands() {
        let command_type = parser.command_type();

        use CommandType::*;

        match command_type {
            Arithmetic => {
                let command = parser.arg1();
                code_writer.write_arithmetic(command)?;
            }
            Push | Pop => {
                let segment = parser.arg1();
                let index = parser.arg2();
                code_writer.write_pushpop(command_type, segment, index)?;
            }
            Label => {
                let label = parser.arg1();
                code_writer.write_label(label)?;
            }
            If => {
                let label = parser.arg1();
                code_writer.write_if(label)?;
            }
            Goto => {
                let label = parser.arg1();
                code_writer.write_goto(label)?;
            }
            Function => unimplemented!(),
            Return => unimplemented!(),
            Call => unimplemented!(),
        }

        parser.advance();
    }

    Ok(())
}
