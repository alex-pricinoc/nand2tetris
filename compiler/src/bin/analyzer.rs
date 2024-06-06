use fs::File;
use io::{BufWriter, Result, Write};
use path::Path;
use path::PathBuf;
use std::{env, fs, io, path, process};

use compiler::tokenize::lexer::{tokenize, LiteralKind::*, TokenKind::*};

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("help: analyzer <input jack file..>");
        process::exit(1);
    }

    for file in args {
        let input = fs::read_to_string(&file)?;

        let output = {
            let mut name = Path::new(&file).file_stem().unwrap().to_owned();
            name.push("T");

            PathBuf::new().with_file_name(name).with_extension("xml")
        };

        let mut to = BufWriter::new(File::create(output)?);

        let mut pos = 0;
        let input = input.as_str();
        writeln!(to, "<tokens>")?;

        for token in tokenize(input) {
            let token_text = &input[pos..pos + token.len];

            match token.kind {
                Ident => match token_text {
                    "class" | "constructor" | "function" | "var" | "int" | "char" | "boolean"
                    | "void" | "true" | "false" | "null" | "this" | "let" | "do" | "if"
                    | "else" | "while" | "return" => {
                        writeln!(to, "<keyword> {token_text} </keyword>")?;
                    }
                    _ => writeln!(to, "<identifier> {token_text} </identifier>")?,
                },
                Literal(Int) => writeln!(to, "<integerConstant> {token_text} </integerConstant>")?,
                Literal(Str) => writeln!(
                    to,
                    "<stringConstant> {} </stringConstant>",
                    &token_text[1..token_text.len() - 1]
                )?,
                Lt => writeln!(to, "<symbol> &lt; </symbol>")?,
                Gt => writeln!(to, "<symbol> &gt; </symbol>")?,
                And => writeln!(to, "<symbol> &amp; </symbol>")?,
                OpenBrace | CloseBrace | OpenParen | CloseParen | OpenBracket | CloseBracket
                | Dot | Comma | Semi | Plus | Minus | Star | Slash | Or | Eq | Tilde => {
                    writeln!(to, "<symbol> {token_text} </symbol>")?;
                }
                Unknown => panic!("Unknown token: {token_text}"),
                _ => (),
            }

            pos += token.len;
        }

        writeln!(to, "</tokens>")?;
    }

    Ok(())
}
