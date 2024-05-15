use crate::CommandType;
use std::fs::File;
use std::io::Result;
use std::io::Write;

#[derive(Debug, Default)]
pub struct CodeWriter {
    output_file: Option<File>,
    module: Option<String>,
    jump_counter: u16,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_file_name(&mut self, name: &str) {
        let file = File::create(name).expect("set_filename: file not found");
        self.output_file = Some(file);
        self.jump_counter = 0;
    }

    pub fn set_module_name(&mut self, name: &str) {
        self.module = Some(name.to_string());
    }

    pub fn close(&mut self) {
        self.output_file = None;
    }

    pub fn write_arithmetic(&mut self, command: &str) -> Result<()> {
        let out = self.output_file.as_mut().expect("output_file is set");

        match command {
            "add" | "sub" | "and" | "or" => {
                let op = match command {
                    "add" => "+",
                    "sub" => "-",
                    "and" => "&",
                    "or" => "|",
                    _ => unreachable!(),
                };

                writeln!(
                    out,
                    "\
@SP
AM=M-1
D=M
A=A-1
M=M{op}D // {command}"
                )
            }

            "neg" | "not" => {
                let op = match command {
                    "neg" => "-",
                    "not" => "!",
                    _ => unreachable!(),
                };

                writeln!(
                    out,
                    "\
@SP
A=M-1
M={op}M // {command}"
                )
            }

            "eq" | "gt" | "lt" => {
                let jump = match command {
                    "eq" => "JEQ",
                    "gt" => "JGT",
                    "lt" => "JLT",
                    _ => unreachable!(),
                };

                let branch = command;
                let id = self.jump_counter;
                self.jump_counter += 1;

                writeln!(
                    out,
                    "\
@SP
AM=M-1
D=M
A=A-1
D=M-D
@__{branch}_{id}_true
D;{jump}
D=0
@__{branch}_{id}_end
0;JMP
(__{branch}_{id}_true)
D=-1
(__{branch}_{id}_end)
@SP
A=M-1
M=D // {command}"
                )
            }

            c => unreachable!("invavalid command{c}"),
        }
    }

    pub fn write_pushpop(&mut self, command: CommandType, segment: &str, index: u16) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");
        let filename = self.module.as_ref().expect("module is set");

        let basename = filename
            .strip_suffix(".vm")
            .expect("file is a list of vm instructions");

        let static_symbol = format!("{basename}.{index}");

        use CommandType::*;
        match command {
            Push => match segment {
                "constant" => {
                    writeln!(
                        out,
                        "\
@{index}
D=A
{PUSH_REGD} // push {segment} {index}"
                    )
                }
                "argument" | "local" | "this" | "that" => {
                    let symbol = get_segment_symbol(segment);

                    writeln!(
                        out,
                        "\
@{symbol}
D=M
@{index}
A=A+D
D=M
{PUSH_REGD} // push {segment} {index}"
                    )
                }
                "pointer" | "temp" => {
                    let index = index as usize;
                    let symbol = match segment {
                        "pointer" => ["THIS", "THAT"][index],
                        "temp" => ["R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12"][index],
                        _ => unreachable!(),
                    };

                    writeln!(
                        out,
                        "\
@{symbol}
D=M
{PUSH_REGD}
"
                    )
                }
                "static" => {
                    writeln!(
                        out,
                        "\
@{static_symbol}
D=M
{PUSH_REGD}
"
                    )
                }

                s => panic!("invalid segment: {s}"),
            },
            Pop => match segment {
                "argument" | "local" | "this" | "that" => {
                    let symbol = get_segment_symbol(segment);

                    writeln!(
                        out,
                        "\
@{symbol}
D=M
@{index}
D=D+A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D"
                    )
                }
                "pointer" | "temp" => {
                    let index = index as usize;

                    let symbol = match segment {
                        "pointer" => ["THIS", "THAT"][index],
                        "temp" => ["R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12"][index],
                        _ => unreachable!(),
                    };

                    writeln!(
                        out,
                        "\
@SP
AM=M-1
D=M
@{symbol}
M=D"
                    )
                }

                "static" => {
                    writeln!(
                        out,
                        "\
@SP
AM=M-1
D=M
@{static_symbol}
M=D",
                    )
                }

                _ => unreachable!("invalid segment: {segment}"),
            },

            _ => panic!("invalid command: {command:?}"),
        }
    }

    pub fn write_label(&mut self, label: &str) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");

        writeln!(
            out,
            "\
({label})"
        )
    }

    pub fn write_if(&mut self, label: &str) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");

        writeln!(
            out,
            "\
@SP
AM=M-1
D=M
@{label}
D;JGT"
        )
    }

    pub fn write_goto(&mut self, label: &str) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");

        writeln!(out, "@{label}\n0;JMP")
    }

    pub fn write_function(&mut self, name: &str, n_vars: u16) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");

        writeln!(out, "({name})")?;

        for i in 0..n_vars {
            writeln!(
                out,
                "\
@LCL
D=M
@{i}
A=A+D
D=M
@SP
M=M+1 // push {i}"
            )?;
        }

        Ok(())
    }

    pub fn write_call(&mut self, name: &str, n_args: u16) -> Result<()> {
        unimplemented!()
    }

    pub fn write_return(&mut self) -> Result<()> {
        let out = self.output_file.as_mut().expect("file is set");

        writeln!(
            out,
            "\
@LCL
D=M
@R5
M=D // endFrame = LCL
@5
D=D-A
A=D
D=M
@R6
M=D // retAddr = *(endFrame - 5)
@SP
AM=M-1
D=M
@ARG
A=M
M=D // *ARG = pop()
D=A+1
@SP
M=D // SP = ARG + 1
@R5
D=M
@1
D=D-A
A=D
D=M
@THAT
M=D // THAT = *(endFrame - 1)
@R5
D=M
@2
D=D-A
A=D
D=M
@THIS
M=D // THIS = *(endFrame - 2)
@R5
D=M
@3
D=D-A
A=D
D=M
@ARG
M=D // ARG = *(endFrame - 3)
@R5
D=M
@4
D=D-A
A=D
D=M
@LCL
M=D // LCL = *(endFrame - 4)
@R6
A=M
0;JMP // goto retAddr"
        )
    }
}

fn get_segment_symbol(segment: &str) -> &'static str {
    match segment {
        "argument" => "ARG",
        "local" => "LCL",
        "this" => "THIS",
        "that" => "THAT",
        _ => panic!("invalid segment name: {}", segment),
    }
}

const PUSH_REGD: &str = "\
@SP
A=M
M=D
@SP
M=M+1";
