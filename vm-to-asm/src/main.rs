use std::path::Path;
use std::{env, fs, io, process};
use vm_to_asm::{code_writer::CodeWriter, parser::Parser, CommandType};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("help: vm-to-asm <input vm file..> [output asm file]");
        process::exit(1);
    }

    let (vm_files, asm_file): (Vec<_>, Vec<_>) = args.into_iter().partition(|f| f.ends_with(".vm"));

    let dest = asm_file.first().expect("must provide an output asm file");

    let mut code_writer = CodeWriter::new();
    code_writer.set_file_name(dest);

    if vm_files.len() > 1 {
        code_writer.write_init()?;
    }

    for file in vm_files {
        let input = fs::read_to_string(&file)?;
        let mut parser = Parser::new(&input);

        let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
        code_writer.set_module_name(file_name);

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
                Function => {
                    let function_name = parser.arg1();
                    let n_vars = parser.arg2();
                    code_writer.write_function(function_name, n_vars)?;
                }
                Call => {
                    let function_name = parser.arg1();
                    let n_args = parser.arg2();
                    code_writer.write_call(function_name, n_args)?;
                }
                Return => code_writer.write_return()?,
            }

            parser.advance();
        }
    }

    Ok(())
}
