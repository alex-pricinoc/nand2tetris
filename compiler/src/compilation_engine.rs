use lazy_format::lazy_format;

use std::fmt::Display;
use std::io::{self, Write};
use std::iter::Peekable;
use std::matches as m;

mod xml;
use xml::Xml;

use crate::*;

#[cfg(test)]
mod tests;

type Result<T = ()> = io::Result<T>;

type TokenStream<'a> = Box<dyn Iterator<Item = Token<'a>> + 'a>;

pub struct CompilationEngine<'a, W> {
    tokens: Peekable<TokenStream<'a>>,
    indent: usize,
    output: W,
}

impl<'a, W: Write> CompilationEngine<'a, W> {
    pub fn new(input: &'a str, output: W) -> Self {
        let tokens = parse(input).map(|t| t.unwrap());
        let tokens = Box::new(tokens) as TokenStream;

        Self {
            tokens: tokens.peekable(),
            indent: 0,
            output,
        }
    }

    pub fn compile(&mut self) -> Result {
        self.increase_level("class")?;

        self.compile_class()?;

        self.decrease_level("class")?;

        Ok(())
    }

    /// Compiles a complete class
    fn compile_class(&mut self) -> Result {
        self.expect_token(|t| t == Keyword(Class), "`class`");
        self.write_token()?;

        self.identifier()?;

        self.open_brace()?;

        while self.next_token(|t| m!(t, Keyword(Static | Field))) {
            self.compile_class_var_dec()?;
        }

        while self.next_token(|t| m!(t, Keyword(Constructor | Function | Method))) {
            self.compile_subroutine()?;
        }

        self.close_brace()?;

        Ok(())
    }

    /// ('constructor' | 'function' | 'method')
    /// ('void' | type) subroutineName '(' parameterList ')'
    /// subroutineBody
    fn compile_subroutine(&mut self) -> Result {
        self.increase_level("subroutineDec")?;

        self.expect_token(
            |t| m!(t, Keyword(Constructor | Function | Method)),
            "expected `constructor` | `function` | `method`",
        );
        self.write_token()?;

        self.expect_token(
            |t| m!(t, Keyword(Void | Int | Char | Boolean) | Identifier(..)),
            "expected `void` | `int` | `char` | `boolean` | className",
        );
        self.write_token()?;

        self.identifier()?;

        self.open_paren()?;

        self.compile_parameter_list()?;

        self.close_paren()?;

        self.compile_subroutine_body()?;

        self.decrease_level("subroutineDec")?;

        Ok(())
    }

    /// ('static' | 'field') type varName (',' varName)* ';'
    fn compile_class_var_dec(&mut self) -> Result {
        self.increase_level("classVarDec")?;

        self.expect_token(
            |t| m!(t, Keyword(Static | Field)),
            "expected static | field",
        );
        self.write_token()?;

        self.r#type()?;

        self.identifier()?;

        while self.next_token(|t| m!(t, Symbol(Comma))) {
            self.write_token()?;
            self.identifier()?;
        }

        self.semi()?;

        self.decrease_level("classVarDec")?;

        Ok(())
    }

    fn compile_parameter_list(&mut self) -> Result {
        self.increase_level("parameterList")?;

        if self.next_token(|t| m!(t, Keyword(Int | Char | Boolean) | Identifier(..))) {
            self.r#type()?;
            self.identifier()?;

            while self.next_token(|t| m!(t, Symbol(Comma))) {
                self.write_token()?;
                self.r#type()?;
                self.identifier()?;
            }
        }

        self.decrease_level("parameterList")?;

        Ok(())
    }

    fn compile_subroutine_body(&mut self) -> Result {
        self.increase_level("subroutineBody")?;

        self.open_brace()?;

        while self.next_token(|t| t == Keyword(Var)) {
            self.compile_var_dec()?;
        }

        self.compile_statements()?;

        self.close_brace()?;

        self.decrease_level("subroutineBody")?;

        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result {
        self.increase_level("varDec")?;

        self.expect_token(|t| t == Keyword(Var), "var");
        self.write_token()?;

        self.r#type()?;

        self.identifier()?;

        while self.next_token(|t| m!(t, Symbol(Comma))) {
            self.write_token()?;
            self.identifier()?;
        }

        self.semi()?;

        self.decrease_level("varDec")?;

        Ok(())
    }

    fn compile_statements(&mut self) -> Result {
        self.increase_level("statements")?;

        while self.next_token(|t| m!(t, Keyword(Let | If | While | Do | Return))) {
            match self.tokens.peek() {
                Some(Keyword(Let)) => self.compile_let()?,
                Some(Keyword(If)) => self.compile_if()?,
                Some(Keyword(While)) => self.compile_while()?,
                Some(Keyword(Return)) => self.compile_return()?,
                Some(Keyword(Do)) => self.compile_do()?,
                _ => (),
            }
        }

        self.decrease_level("statements")?;
        Ok(())
    }

    /// 'let' varName ('[' expression ']')? '=' expression ';'
    fn compile_let(&mut self) -> Result {
        self.increase_level("letStatement")?;

        self.expect_token(|t| t == Keyword(Let), "`let`");
        self.write_token()?;

        self.identifier()?;

        if self.next_token(|t| t == Symbol(OpenBracket)) {
            self.write_token()?;

            self.compile_expression()?;

            self.close_bracket()?;
        }

        self.expect_token(|t| t == Symbol(Eq), "`=`");
        self.write_token()?;

        self.compile_expression()?;

        self.semi()?;

        self.decrease_level("letStatement")?;

        Ok(())
    }

    fn compile_if(&mut self) -> Result {
        self.increase_level("ifStatement")?;

        self.expect_token(|t| t == Keyword(If), "`if`");
        self.write_token()?;

        self.open_paren()?;

        self.compile_expression()?;

        self.close_paren()?;

        self.open_brace()?;

        self.compile_statements()?;

        self.close_brace()?;

        if self.next_token(|t| t == Keyword(Else)) {
            self.write_token()?;
            self.open_brace()?;

            self.compile_statements()?;

            self.close_brace()?;
        }

        self.decrease_level("ifStatement")?;

        Ok(())
    }

    fn compile_while(&mut self) -> Result {
        self.increase_level("whileStatement")?;

        self.expect_token(|t| t == Keyword(While), "`while`");
        self.write_token()?;

        self.open_paren()?;
        self.compile_expression()?;
        self.close_paren()?;

        self.open_brace()?;

        self.compile_statements()?;

        self.close_brace()?;

        self.decrease_level("whileStatement")?;

        Ok(())
    }

    fn compile_do(&mut self) -> Result {
        self.increase_level("doStatement")?;

        self.expect_token(|t| t == Keyword(Do), "`do`");

        self.write_token()?;

        self.term()?;

        while self.next_token(|t| {
            m!(
                t,
                Symbol(Plus | Minus | Star | Slash | And | Or | Lt | Gt | Eq)
            )
        }) {
            self.write_token()?;
            self.term()?;
        }

        self.semi()?;

        self.decrease_level("doStatement")?;

        Ok(())
    }

    fn compile_return(&mut self) -> Result {
        self.increase_level("returnStatement")?;

        self.expect_token(|t| t == Keyword(Return), "`return`");
        self.write_token()?;

        if self.next_token(|t| t != Symbol(Semi)) {
            self.compile_expression()?;
        }

        self.semi()?;

        self.decrease_level("returnStatement")?;

        Ok(())
    }

    /// Compiles a term. This routine is faced with a slight difficulty
    /// when trying to decide between some of the alternative parsing
    /// rules. Specifically, if the current token is an identifier, the routine
    /// must distinguish between a variable, an array entry, and a
    /// subroutine call. A single lookahead token, which may be one
    /// of `[`, `(`, or `.` suffices to distinguish between the three possi-
    /// bilities. Any other token is not part of this term and should not be advanced over.
    fn compile_term(&mut self) -> Result {
        self.increase_level("term")?;

        self.term()?;

        self.decrease_level("term")?;

        Ok(())
    }

    fn compile_expression(&mut self) -> Result {
        self.increase_level("expression")?;

        self.compile_term()?;

        while self.next_token(|t| {
            m!(
                t,
                Symbol(Plus | Minus | Star | Slash | And | Or | Lt | Gt | Eq)
            )
        }) {
            self.write_token()?;
            self.compile_term()?;
        }

        self.decrease_level("expression")?;

        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<usize> {
        let mut expression_count = 0;

        self.increase_level("expressionList")?;

        if self.next_token(|t| t != Symbol(CloseParen)) {
            self.compile_expression()?;
            expression_count += 1;

            while self.next_token(|t| t == Symbol(Comma)) {
                self.write_token()?;
                self.compile_expression()?;
                expression_count += 1;
            }
        }

        self.decrease_level("expressionList")?;

        Ok(expression_count)
    }

    fn term(&mut self) -> Result {
        match self.tokens.peek() {
            Some(StringConstant(..) | IntegerConstant(..) | Keyword(..)) => {
                self.write_token()?;
            }
            Some(Symbol(Tilde | Minus)) => {
                self.write_token()?;
                self.compile_term()?;
            }
            Some(Symbol(OpenParen)) => {
                self.write_token()?;
                self.compile_expression()?;
                self.close_paren()?;
            }
            _ => (),
        }

        if self.next_token(|t| m!(t, Identifier(..))) {
            self.identifier()?;

            // foo | foo[expression] | foo(expressionList) | Foo.bar(expressionList) | foo.bar(expressionList)
            match self.tokens.peek() {
                // foo(expressionList)
                Some(Symbol(OpenParen)) => {
                    self.write_token()?;
                    self.compile_expression_list()?;
                    self.expect_token(|t| t == Symbol(CloseParen), "`)`");
                    self.write_token()?;
                }
                // foo[expression]
                Some(Symbol(OpenBracket)) => {
                    self.write_token()?;
                    self.compile_expression()?;
                    self.close_bracket()?;
                }
                // Foo.bar(expressionList) | foo.bar(expressionList)
                Some(Symbol(Dot)) => {
                    self.write_token()?;
                    self.identifier()?;
                    self.open_paren()?;
                    self.compile_expression_list()?;
                    self.close_paren()?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn r#type(&mut self) -> Result {
        self.expect_token(
            |t| m!(t, Keyword(Int | Char | Boolean) | Identifier(..)),
            "expected `int` | `char` | `boolean` | className",
        );

        self.write_token()?;

        Ok(())
    }

    fn semi(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(Semi), "`;`");

        self.write_token()
    }

    fn identifier(&mut self) -> Result {
        self.expect_token(|t| m!(t, Identifier(..)), "identifier");
        self.write_token()
    }

    fn open_paren(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(OpenParen), "`(`");
        self.write_token()
    }

    fn close_paren(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(CloseParen), "`)`");
        self.write_token()
    }

    fn open_brace(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(OpenBrace), "`{`");
        self.write_token()
    }

    fn close_brace(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(CloseBrace), "`}`");
        self.write_token()
    }

    #[allow(dead_code)]
    fn open_bracket(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(OpenBracket), "`[`");
        self.write_token()
    }

    fn close_bracket(&mut self) -> Result {
        self.expect_token(|t| t == Symbol(CloseBracket), "`]`");
        self.write_token()
    }

    /* ----------     Helper methods     ----------   */

    fn write_token(&mut self) -> Result {
        let token = self.tokens.next().unwrap();
        self.write_xml(token.to_xml())?;

        Ok(())
    }

    fn expect_token(&mut self, f: impl FnOnce(&Token) -> bool, expected: impl Display) {
        let next = self.tokens.peek();

        assert!(next.is_some_and(f), "expected {expected}, found: {next:?}");
    }

    fn next_token(&mut self, f: impl FnOnce(&Token) -> bool) -> bool {
        self.tokens.peek().is_some_and(f)
    }

    fn increase_level(&mut self, tag: &str) -> Result {
        self.write_xml(lazy_format!("<{tag}>"))?;
        self.indent += 2;

        Ok(())
    }

    fn decrease_level(&mut self, tag: &str) -> Result {
        self.indent -= 2;
        self.write_xml(lazy_format!("</{tag}>"))
    }

    fn write_xml(&mut self, val: impl Display) -> Result {
        writeln!(
            self.output,
            "{:indent$}{xml}",
            "",
            indent = self.indent,
            xml = val
        )
    }
}
