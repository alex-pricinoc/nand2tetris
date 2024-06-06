use std::{fmt, io};

use crate::parse::ast;

use super::Analyzer;

impl<'a, T: io::Write> Analyzer for XMLAnalyzer<'a, T> {
    type Result = io::Result<()>;

    fn analyze(&mut self, tree: &ast::Ast) -> Self::Result {
        self.writer.compile(tree)
    }
}

pub struct XMLAnalyzer<'a, T> {
    writer: XMLWriter<'a, T>,
}

impl<'a, T> XMLAnalyzer<'a, T> {
    pub fn new(out: &'a mut T) -> Self {
        Self {
            writer: XMLWriter { out, level: 0 },
        }
    }
}

struct XMLWriter<'a, T> {
    out: &'a mut T,
    level: usize,
}

impl<'a, T: io::Write> XMLWriter<'a, T> {
    fn compile(&mut self, ast: &ast::Ast) -> io::Result<()> {
        self.compile_class(&ast.class)
    }

    fn compile_class(&mut self, class: &ast::Class) -> io::Result<()> {
        self.write("<class>")?;
        self.level += 2;
        self.keyword("class")?;
        self.ident(&class.name)?;
        self.symbol('{')?;
        self.compile_class_vars(&class.vars)?;
        self.compile_subroutines(&class.subroutines)?;
        self.symbol('}')?;
        self.level -= 2;
        self.write("</class>")?;

        Ok(())
    }

    fn compile_class_vars(&mut self, vars: &[ast::ClassVar]) -> io::Result<()> {
        for var in vars {
            self.write("<classVarDec>")?;
            self.level += 2;
            self.keyword(var.kind)?;
            self.r#type(&var.r#type)?;

            for (i, n) in var.names.iter().enumerate() {
                if i > 0 {
                    self.symbol(',')?;
                }
                self.ident(n)?;
            }

            self.symbol(';')?;
            self.level -= 2;
            self.write("</classVarDec>")?;
        }

        Ok(())
    }

    fn compile_subroutines(&mut self, subs: &[ast::Subroutine]) -> io::Result<()> {
        for sub in subs {
            self.write("<subroutineDec>")?;
            self.level += 2;
            self.keyword(&sub.kind)?;
            self.return_type(&sub.r#type)?;
            self.ident(&sub.name)?;
            self.symbol('(')?;
            self.parameter_list(&sub.parameters)?;
            self.symbol(')')?;
            self.write("<subroutineBody>")?;
            self.level += 2;
            self.symbol('{')?;
            self.var_dec(&sub.body.vars)?;
            self.statements(&sub.body.statements)?;
            self.symbol('}')?;
            self.level -= 2;
            self.write("</subroutineBody>")?;
            self.level -= 2;
            self.write("</subroutineDec>")?;
        }

        Ok(())
    }

    fn statements(&mut self, stms: &[ast::Statement]) -> io::Result<()> {
        self.write("<statements>")?;
        self.level += 2;

        for s in stms {
            self.statement(s)?;
        }

        self.level -= 2;
        self.write("</statements>")?;
        Ok(())
    }

    fn statement(&mut self, statement: &ast::Statement) -> io::Result<()> {
        match statement {
            ast::Statement::Let {
                var,
                accessor,
                expression,
            } => {
                self.write("<letStatement>")?;
                self.level += 2;
                self.keyword("let")?;
                self.ident(var)?;

                if let Some(acc) = accessor {
                    self.symbol('[')?;
                    self.expression(acc)?;
                    self.symbol(']')?;
                }

                self.symbol('=')?;
                self.expression(expression)?;
                self.symbol(';')?;
                self.level -= 2;
                self.write("</letStatement>")?;

                Ok(())
            }
            ast::Statement::If {
                cond,
                statements,
                r#else,
            } => {
                self.write("<ifStatement>")?;
                self.level += 2;
                self.keyword("if")?;
                self.symbol('(')?;
                self.expression(cond)?;
                self.symbol(')')?;
                self.symbol('{')?;
                self.statements(statements)?;
                self.symbol('}')?;

                if let Some(stms) = r#else {
                    self.keyword("else")?;
                    self.symbol('{')?;
                    self.statements(stms)?;
                    self.symbol('}')?;
                }

                self.level -= 2;
                self.write("</ifStatement>")?;

                Ok(())
            }
            ast::Statement::While { cond, body } => {
                self.write("<whileStatement>")?;
                self.level += 2;
                self.keyword("while")?;
                self.symbol('(')?;
                self.expression(cond)?;
                self.symbol(')')?;
                self.symbol('{')?;
                self.statements(body)?;
                self.symbol('}')?;
                self.level -= 2;
                self.write("</whileStatement>")?;

                Ok(())
            }
            ast::Statement::Do { subroutine_call } => {
                self.write("<doStatement>")?;
                self.level += 2;
                self.keyword("do")?;
                self.subroutine_call(subroutine_call)?;
                self.symbol(';')?;
                self.level -= 2;
                self.write("</doStatement>")?;

                Ok(())
            }
            ast::Statement::Return { value } => {
                self.write("<returnStatement>")?;
                self.level += 2;

                self.keyword("return")?;

                if let Some(value) = value {
                    self.expression(value)?;
                }

                self.symbol(';')?;
                self.level -= 2;
                self.write("</returnStatement>")?;

                Ok(())
            }
        }
    }

    fn expression(&mut self, expr: &ast::Expression) -> io::Result<()> {
        self.write("<expression>")?;
        self.level += 2;

        self.term(&expr.0)?;

        for (op, term) in &expr.1 {
            self.symbol(op)?;
            self.term(term)?;
        }

        self.level -= 2;
        self.write("</expression>")
    }

    fn parameter_list(&mut self, params: &[ast::Parameter]) -> io::Result<()> {
        self.write("<parameterList>")?;
        self.level += 2;

        for (i, p) in params.iter().enumerate() {
            if i > 0 {
                self.symbol(',')?;
            }
            self.keyword(&p.r#type)?;
            self.ident(&p.name)?;
        }

        self.level -= 2;
        self.write("</parameterList>")?;

        Ok(())
    }

    fn var_dec(&mut self, vars: &[ast::SubroutineVar]) -> io::Result<()> {
        for v in vars {
            self.write("<varDec>")?;
            self.level += 2;
            self.keyword("var")?;
            self.r#type(&v.r#type)?;

            for (i, n) in v.names.iter().enumerate() {
                if i > 0 {
                    self.symbol(',')?;
                }
                self.ident(n)?;
            }

            self.symbol(';')?;
            self.level -= 2;
            self.write("</varDec>")?;
        }

        Ok(())
    }

    fn term(&mut self, term: &ast::Term) -> io::Result<()> {
        self.write("<term>")?;
        self.level += 2;

        match term {
            ast::Term::Int(i) => {
                self.write(format_args!("<integerConstant> {i} </integerConstant>"))?;
            }
            ast::Term::Str(s) => {
                self.write(format_args!("<stringConstant> {s} </stringConstant>"))?;
            }
            ast::Term::Keyword(k) => self.keyword(k)?,
            ast::Term::Var(var) => self.ident(var)?,
            ast::Term::Index(acc, expr) => {
                self.ident(acc)?;
                self.symbol('[')?;
                self.expression(expr)?;
                self.symbol(']')?;
            }
            ast::Term::Call(call) => self.subroutine_call(call)?,
            ast::Term::Paren(expr) => {
                self.symbol('(')?;
                self.expression(expr)?;
                self.symbol(')')?;
            }
            ast::Term::Unary(op, term) => {
                self.symbol(op)?;
                self.term(term)?;
            }
        }

        self.level -= 2;
        self.write("</term>")
    }

    fn subroutine_call(&mut self, call: &ast::SubroutineCall) -> io::Result<()> {
        match call {
            ast::SubroutineCall::Function { name, expressions } => {
                self.ident(name)?;
                self.symbol('(')?;
                self.expression_list(expressions)?;
                self.symbol(')')?;
            }
            ast::SubroutineCall::Method {
                name,
                receiver,
                expressions,
            } => {
                self.ident(receiver)?;
                self.symbol('.')?;
                self.ident(name)?;
                self.symbol('(')?;
                self.expression_list(expressions)?;
                self.symbol(')')?;
            }
        }

        Ok(())
    }

    fn expression_list(&mut self, exprs: &[ast::Expression]) -> io::Result<()> {
        self.write("<expressionList>")?;
        self.level += 2;

        for (i, e) in exprs.iter().enumerate() {
            if i > 0 {
                self.symbol(',')?;
            }
            self.expression(e)?;
        }

        self.level -= 2;
        self.write("</expressionList>")?;
        Ok(())
    }

    fn r#type(&mut self, r#type: &ast::Type) -> io::Result<()> {
        match r#type {
            ast::Type::ClassName(c) => self.ident(c),
            t => self.keyword(t),
        }
    }

    fn return_type(&mut self, rtype: &ast::ReturnType) -> io::Result<()> {
        match rtype {
            ast::ReturnType::Void => self.keyword("void"),
            ast::ReturnType::Type(t) => self.r#type(t),
        }
    }

    fn symbol(&mut self, val: impl fmt::Display) -> io::Result<()> {
        self.write(format_args!("<symbol> {val} </symbol>"))
    }

    fn keyword(&mut self, val: impl fmt::Display) -> io::Result<()> {
        self.write(format_args!("<keyword> {val} </keyword>"))
    }

    fn ident(&mut self, ident: impl fmt::Display) -> io::Result<()> {
        self.write(format_args!("<identifier> {ident} </identifier>"))
    }

    fn write(&mut self, val: impl fmt::Display) -> io::Result<()> {
        writeln!(self.out, "{:level$}{val}", "", level = self.level)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use crate::tokenize::tokenize;
    use pretty_assertions::assert_eq;

    use super::*;

    fn compile_xml(input: &str) -> String {
        let tokens = tokenize(input).collect::<Result<Vec<_>, _>>().unwrap();
        let tree = parse(tokens).unwrap();
        let mut output = vec![];
        let mut analyzer = XMLAnalyzer::new(&mut output);
        analyzer.analyze(&tree).unwrap();

        String::from_utf8(output).unwrap()
    }

    #[test]
    fn expressionless_square() {
        let input = include_str!("../../../projects/10/ExpressionLessSquare/Main.jack");
        let output = include_str!("../../../projects/10/ExpressionLessSquare/Main.xml");
        assert_eq!(compile_xml(input), output);

        let input = include_str!("../../../projects/10/ExpressionLessSquare/Square.jack");
        let output = include_str!("../../../projects/10/ExpressionLessSquare/Square.xml");
        assert_eq!(compile_xml(input), output);

        let input = include_str!("../../../projects/10/ExpressionLessSquare/SquareGame.jack");
        let output = include_str!("../../../projects/10/ExpressionLessSquare/SquareGame.xml");
        assert_eq!(compile_xml(input), output);
    }

    #[test]
    fn array_test() {
        let input = include_str!("../../../projects/10/ArrayTest/Main.jack");
        let output = include_str!("../../../projects/10/ArrayTest/Main.xml");

        assert_eq!(compile_xml(input), output);
    }

    #[test]
    fn square() {
        let input = include_str!("../../../projects/10/Square/Main.jack");
        let output = include_str!("../../../projects/10/Square/Main.xml");

        assert_eq!(compile_xml(input), output);

        let input = include_str!("../../../projects/10/Square/Square.jack");
        let output = include_str!("../../../projects/10/Square/Square.xml");

        assert_eq!(compile_xml(input), output);

        let input = include_str!("../../../projects/10/Square/SquareGame.jack");
        let output = include_str!("../../../projects/10/Square/SquareGame.xml");

        assert_eq!(compile_xml(input), output);
    }
}
