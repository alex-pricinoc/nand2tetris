use crate::tokenize;
use crate::LiteralKind::*;
use crate::TokenKind::*;
use std::io;
use std::io::Write;

pub struct Parser<W> {
    input: String,
    output: W,
}

impl<W: Write> Parser<W> {
    pub fn new(input: String, output: W) -> Self {
        Self { input, output }
    }

    pub fn to_xml_0(&mut self) -> io::Result<()> {
        let mut pos = 0;

        let input = self.input.as_str();

        writeln!(self.output, "<tokens>")?;

        for token in tokenize(input) {
            let token_text = &input[pos..pos + token.len];

            match token.kind {
                Ident => match token_text {
                    "class" | "constructor" | "function" | "var" | "int" | "char" | "boolean"
                    | "void" | "true" | "false" | "null" | "this" | "let" | "do" | "if"
                    | "else" | "while" | "return" => {
                        writeln!(self.output, "<keyword> {} </keyword>", token_text)?
                    }
                    _ => writeln!(self.output, "<identifier> {} </identifier>", token_text)?,
                },
                Literal(Int) => writeln!(
                    self.output,
                    "<integerConstant> {} </integerConstant>",
                    token_text
                )?,
                Literal(Str) => writeln!(
                    self.output,
                    "<stringConstant> {} </stringConstant>",
                    &token_text[1..token_text.len() - 1]
                )?,
                Lt => writeln!(self.output, "<symbol> &lt; </symbol>")?,
                Gt => writeln!(self.output, "<symbol> &gt; </symbol>")?,
                And => writeln!(self.output, "<symbol> &amp; </symbol>")?,
                OpenBrace | CloseBrace | OpenParen | CloseParen | OpenBracket | CloseBracket
                | Dot | Comma | Semi | Plus | Minus | Star | Slash | Or | Eq | Tilde => {
                    writeln!(self.output, "<symbol> {} </symbol>", token_text)?;
                }
                Unknown => panic!("Unknown token: {token_text}"),
                _ => (),
            }

            pos += token.len;
        }

        writeln!(self.output, "</tokens>")?;

        Ok(())
    }
}
