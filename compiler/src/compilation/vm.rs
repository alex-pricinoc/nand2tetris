use std::io;

use crate::parse::ast::{self, ClassVarKind, SubroutineKind};

use self::vm_writer::*;

use super::Analyzer;

mod symbol_table;
mod vm_writer;
use symbol_table::{Kind, SymbolTable};
mod labels;
use labels::{Label, Labels};

pub struct VMWriter<'a, T> {
    out: &'a mut T,
    symbols: SymbolTable,
    labels: Labels,
    class_name: Option<String>,
}

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

impl<'a, T: io::Write> Analyzer for VMWriter<'a, T> {
    type Result = Result<()>;

    fn analyze(&mut self, tree: &ast::Ast) -> Self::Result {
        self.compile(tree)
    }
}

impl<'a, T: io::Write> VMWriter<'a, T> {
    pub fn new(out: &'a mut T) -> Self {
        Self {
            out,
            class_name: None,
            symbols: Default::default(),
            labels: Default::default(),
        }
    }

    fn compile(&mut self, ast: &ast::Ast) -> Result {
        self.class_name = Some(ast.class.name.clone());
        self.compile_class(&ast.class)
    }

    fn compile_class(&mut self, class: &ast::Class) -> Result {
        self.compile_class_var_dec(&class.vars);

        for sub in &class.subroutines {
            self.compile_subroutine(sub, &class.name)?;
        }

        Ok(())
    }

    fn compile_class_var_dec(&mut self, vars: &[ast::ClassVar]) {
        for v in vars {
            for n in &v.names {
                self.symbols.define(&v.r#type, v.kind.into(), n);
            }
        }
    }

    fn compile_subroutine(&mut self, sub: &ast::Subroutine, class: &str) -> Result {
        self.symbols.start_subroutine();
        self.labels.clear();

        let locals = sub.body.vars.iter().flat_map(|v| &v.names).count();

        write_function(
            &mut self.out,
            format_args!("{}.{}", class, sub.name),
            locals,
        )?;

        match sub.kind {
            SubroutineKind::Constructor => {
                write_push(&mut self.out, Segment::Const, self.symbols.fields() as _)?;
                write_call(&mut self.out, "Memory.alloc", 1)?;
                write_pop(&mut self.out, Segment::Pointer, 0)?;
            }
            SubroutineKind::Method => {
                write_push(&mut self.out, Segment::Arg, 0)?;
                write_pop(&mut self.out, Segment::Pointer, 0)?;
                self.symbols.define(
                    &ast::Type::ClassName(class.to_owned()),
                    Kind::Argument,
                    "this",
                );
            }
            SubroutineKind::Function => (),
        }

        self.compile_parameter_list(&sub.parameters);
        self.compile_subroutine_body(&sub.body)?;

        Ok(())
    }

    fn compile_parameter_list(&mut self, parameters: &[ast::Parameter]) {
        for p in parameters {
            self.symbols.define(&p.r#type, Kind::Argument, &p.name);
        }
    }

    fn compile_subroutine_body(&mut self, body: &ast::SubroutineBody) -> Result {
        for v in &body.vars {
            self.compile_var_dec(v);
        }

        for s in &body.statements {
            self.compile_statement(s)?;
        }

        Ok(())
    }

    fn compile_var_dec(&mut self, var: &ast::SubroutineVar) {
        for n in &var.names {
            self.symbols.define(&var.r#type, Kind::Local, n);
        }
    }

    fn compile_statement(&mut self, statement: &ast::Statement) -> Result {
        match statement {
            ast::Statement::Let {
                var,
                accessor,
                expression,
            } => {
                if let Some(expr) = accessor {
                    self.compile_expresssion(expr)?;
                    let s = self
                        .symbols
                        .get(var)
                        .ok_or_else(|| self.undefined_symbol(var))?;

                    write_push(&mut self.out, s.kind.into(), s.index as _)?;
                    write_arithmetic(&mut self.out, Arithmetic::Add)?;

                    self.compile_expresssion(expression)?;
                    write_pop(&mut self.out, Segment::Temp, 0)?;
                    write_pop(&mut self.out, Segment::Pointer, 1)?;
                    write_push(&mut self.out, Segment::Temp, 0)?;
                    write_pop(&mut self.out, Segment::That, 0)?;
                } else {
                    self.compile_expresssion(expression)?;
                    let s = self
                        .symbols
                        .get(var)
                        .ok_or_else(|| self.undefined_symbol(var))?;

                    write_pop(&mut self.out, s.kind.into(), s.index as _)?;
                }
            }
            ast::Statement::If {
                cond,
                statements,
                r#else,
            } => {
                self.compile_expresssion(cond)?;
                let l = self.labels.generate(Label::If);
                let if_true = format!("IF_TRUE{l}");
                let if_false = format!("IF_FALSE{l}");
                let if_end = format!("IF_END{l}");

                write_if(&mut self.out, &if_true)?;
                write_goto(&mut self.out, &if_false)?;
                write_label(&mut self.out, &if_true)?;

                self.compile_statements(statements)?;

                if let Some(s) = r#else {
                    write_goto(&mut self.out, &if_end)?;
                    write_label(&mut self.out, &if_false)?;
                    self.compile_statements(s)?;
                    write_label(&mut self.out, &if_end)?;
                } else {
                    write_label(&mut self.out, &if_false)?;
                }
            }
            ast::Statement::Do { subroutine_call } => {
                self.compile_subroutine_call(subroutine_call)?;
                write_pop(&mut self.out, Segment::Temp, 0)?; // discard return value
            }
            ast::Statement::While { cond, body } => {
                let i = self.labels.generate(Label::While);

                let (l1, l2) = (format!("WHILE_EXP{i}"), format!("WHILE_END{i}"));

                write_label(&mut self.out, &l1)?; // label L1
                self.compile_expresssion(cond)?;
                write_arithmetic(&mut self.out, Arithmetic::Not)?; // not
                write_if(&mut self.out, &l2)?; // if-goto L2

                self.compile_statements(body)?;

                write_goto(&mut self.out, &l1)?; // goto L1
                write_label(&mut self.out, &l2)?; // label L2
            }
            ast::Statement::Return { value } => {
                if let Some(expr) = value {
                    self.compile_expresssion(expr)?;
                } else {
                    write_push(&mut self.out, Segment::Const, 0)?;
                }

                write_return(&mut self.out)?;
            }
        }

        Ok(())
    }

    fn compile_statements(&mut self, statements: &[ast::Statement]) -> Result {
        for s in statements {
            self.compile_statement(s)?;
        }
        Ok(())
    }

    fn compile_expression_list(&mut self, exprs: &[ast::Expression]) -> Result<usize> {
        let mut args = 0;

        for e in exprs {
            self.compile_expresssion(e)?;
            args += 1;
        }

        Ok(args)
    }

    fn compile_expresssion(&mut self, expr: &ast::Expression) -> Result {
        self.compile_term(&expr.0)?;

        for (op, term) in &expr.1 {
            self.compile_term(term)?;
            self.compile_op(op)?;
        }

        Ok(())
    }

    fn compile_term(&mut self, term: &ast::Term) -> Result {
        match term {
            ast::Term::Int(i) => write_push(&mut self.out, Segment::Const, *i)?,
            ast::Term::Str(s) => {
                write_push(&mut self.out, Segment::Const, s.len() as _)?;
                write_call(&mut self.out, "String.new", 1)?;

                for c in s.chars() {
                    write_push(&mut self.out, Segment::Const, c as _)?;
                    write_call(&mut self.out, "String.appendChar", 2)?;
                }
            }
            ast::Term::Keyword(k) => self.keyword(k)?,
            ast::Term::Var(name) => {
                let s = self
                    .symbols
                    .get(name)
                    .ok_or_else(|| self.undefined_symbol(name))?;

                write_push(&mut self.out, s.kind.into(), s.index as _)?;
            }
            ast::Term::Index(var, expr) => {
                self.compile_expresssion(expr)?;

                let s = self
                    .symbols
                    .get(var)
                    .ok_or_else(|| self.undefined_symbol(var))?;

                write_push(&mut self.out, s.kind.into(), s.index as _)?;
                write_arithmetic(&mut self.out, Arithmetic::Add)?;
                write_pop(&mut self.out, Segment::Pointer, 1)?;
                write_push(&mut self.out, Segment::That, 0)?;
            }
            ast::Term::Call(c) => self.compile_subroutine_call(c)?,
            ast::Term::Paren(expr) => self.compile_expresssion(expr)?,
            ast::Term::Unary(op, term) => {
                self.compile_term(term)?;
                match op {
                    ast::UnaryOp::Minus => write_arithmetic(&mut self.out, Arithmetic::Neg)?,
                    ast::UnaryOp::Not => write_arithmetic(&mut self.out, Arithmetic::Not)?,
                }
            }
        }

        Ok(())
    }

    fn compile_op(&mut self, op: &ast::Op) -> Result {
        match op {
            ast::Op::Plus => write_arithmetic(&mut self.out, Arithmetic::Add)?,
            ast::Op::Minus => write_arithmetic(&mut self.out, Arithmetic::Sub)?,
            ast::Op::And => write_arithmetic(&mut self.out, Arithmetic::And)?,
            ast::Op::Or => write_arithmetic(&mut self.out, Arithmetic::Or)?,
            ast::Op::Lt => write_arithmetic(&mut self.out, Arithmetic::Lt)?,
            ast::Op::Gt => write_arithmetic(&mut self.out, Arithmetic::Gt)?,
            ast::Op::Eq => write_arithmetic(&mut self.out, Arithmetic::Eq)?,
            ast::Op::Star => write_call(&mut self.out, "Math.multiply", 2)?,
            ast::Op::Div => write_call(&mut self.out, "Math.divide", 2)?,
        }

        Ok(())
    }

    fn compile_subroutine_call(&mut self, call: &ast::SubroutineCall) -> Result {
        match call {
            ast::SubroutineCall::Function { name, expressions } => {
                write_push(&mut self.out, Segment::Pointer, 0)?; // push THIS
                let args = self.compile_expression_list(expressions)? + 1;
                let class = self.class_name.as_ref().unwrap();
                write_call(&mut self.out, format_args!("{class}.{name}"), args)?;
            }
            ast::SubroutineCall::Method {
                receiver,
                name,
                expressions,
            } => {
                let (receiver, args) = match self.symbols.get(receiver) {
                    Some(symbol) => {
                        write_push(&mut self.out, symbol.kind.into(), symbol.index as _)?;

                        (
                            symbol.r#type.to_string(),
                            self.compile_expression_list(expressions)? + 1,
                        )
                    }
                    None => (receiver.clone(), self.compile_expression_list(expressions)?),
                };

                write_call(&mut self.out, format_args!("{receiver}.{name}"), args)?;
            }
        }

        Ok(())
    }

    fn undefined_symbol(&self, name: &str) -> String {
        format!("symbol `{name}` is not defined")
    }

    fn keyword(&mut self, keyword: &ast::Keyword) -> Result {
        match keyword {
            ast::Keyword::True => {
                write_push(&mut self.out, Segment::Const, 0)?;
                write_arithmetic(&mut self.out, Arithmetic::Not)?;
            }
            ast::Keyword::False => write_push(&mut self.out, Segment::Const, 0)?,
            ast::Keyword::This => write_push(&mut self.out, Segment::Pointer, 0)?,
            ast::Keyword::Null => write_push(&mut self.out, Segment::Const, 0)?,
        }

        Ok(())
    }
}

impl From<Kind> for Segment {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Static => Segment::Static,
            Kind::This => Segment::This,
            Kind::Local => Segment::Local,
            Kind::Argument => Segment::Arg,
        }
    }
}

impl From<ast::ClassVarKind> for symbol_table::Kind {
    fn from(value: ast::ClassVarKind) -> Self {
        use symbol_table::Kind::*;
        match value {
            ClassVarKind::Static => Static,
            ClassVarKind::Field => This,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use glob::glob;
    use pretty_assertions::assert_eq;

    use crate::log;
    use crate::parse::parse;
    use crate::tokenize::tokenize;

    use super::*;

    fn write(input: &str) -> String {
        let tokens = tokenize(input)
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();

        let mut out: Vec<u8> = vec![];
        let tree = parse(tokens).unwrap();
        let mut writer = VMWriter::new(&mut out);
        if let Err(err) = writer.analyze(&tree) {
            log!("{err}");
        }

        String::from_utf8(out).unwrap()
    }

    #[test]
    fn test_seven() {
        let input = include_str!("../../../projects/11/Seven/Main.jack");
        let output = include_str!("../../../projects/11/Seven/Main.vm");

        assert_eq!(write(input), output);
    }

    #[test]
    fn convert_to_bin() {
        let input = include_str!("../../../projects/11/ConvertToBin/Main.jack");
        let output = include_str!("../../../projects/11/ConvertToBin/Main.vm");

        assert_eq!(write(input), output);
    }

    #[test]
    fn square() {
        for entry in glob("../projects/11/Square/*.jack").unwrap() {
            let path = entry.unwrap();
            let input = fs::read_to_string(&path).unwrap();
            let output = fs::read_to_string(&path.with_extension("vm")).unwrap();

            assert_eq!(write(&input), output);
        }
    }

    #[test]
    fn average() {
        let input = include_str!("../../../projects/11/Average/Main.jack");
        let output = include_str!("../../../projects/11/Average/Main.vm");

        assert_eq!(write(input), output);
    }

    #[test]
    fn pong() {
        for entry in glob("../projects/11/Pong/*.jack").unwrap() {
            let path = entry.unwrap();
            let input = fs::read_to_string(&path).unwrap();
            let output = fs::read_to_string(&path.with_extension("vm")).unwrap();

            assert_eq!(write(&input), output);
        }
    }

    #[test]
    fn complex_arrays() {
        let input = include_str!("../../../projects/11/ComplexArrays/Main.jack");
        let output = include_str!("../../../projects/11/ComplexArrays/Main.vm");

        assert_eq!(write(input), output);
    }
}
