use std::matches as m;

use super::*;
use KeywordKind::*;
use SymbolKind::*;
use Token::*;

impl Stream {
    pub fn parse_class(&mut self) -> Result<ast::Class> {
        self.token(Keyword(Class))?;

        let name = self.identifier()?;

        self.token(Symbol(OpenBrace))?;
        let vars = self.parse_class_vars()?;
        let subroutines = self.parse_subroutines()?;
        self.token(Symbol(CloseBrace))?;

        if self.current().is_some() {
            return Err(self.unexpected_token("None"));
        }

        Ok(ast::Class {
            name,
            vars,
            subroutines,
        })
    }

    fn parse_class_vars(&mut self) -> Result<Vec<ast::ClassVar>> {
        let mut vars = vec![];

        while self.next_token(|t| m!(t, Keyword(Field | Static))) {
            let kind = self.class_var_kind()?;
            let r#type = self.r#type()?;
            let name = self.identifier()?;
            let mut names = vec![name];

            while self.next_token(|t| m!(t, Symbol(Comma))) {
                self.advance().unwrap();
                let name = self.identifier()?;
                names.push(name);
            }

            vars.push(ast::ClassVar {
                kind,
                r#type,
                names,
            });

            self.token(Symbol(Semi))?;
        }

        Ok(vars)
    }

    fn parse_subroutines(&mut self) -> Result<Vec<ast::Subroutine>> {
        let mut subroutines = vec![];

        while self.next_token(|t| m!(t, Keyword(Constructor | Function | Method))) {
            let kind = self.subroutine_kind()?;

            let r#type = if self.next_token(|t| t == Keyword(Void)) {
                self.advance().unwrap();
                ast::ReturnType::Void
            } else {
                ast::ReturnType::Type(self.r#type()?)
            };

            let name = self.identifier()?;
            self.token(Symbol(OpenParen))?;
            let parameters = self.parse_parameter_list()?;
            self.token(Symbol(CloseParen))?;
            let body = self.parse_subroutine_body()?;

            subroutines.push(ast::Subroutine {
                kind,
                name,
                r#type,
                parameters,
                body,
            });
        }

        Ok(subroutines)
    }

    fn class_var_kind(&mut self) -> Result<ast::ClassVarKind> {
        let k = match self.current() {
            Some(Keyword(Field)) => ast::ClassVarKind::Field,
            Some(Keyword(Static)) => ast::ClassVarKind::Static,
            _ => return Err(self.unexpected_token("`field` | `static`")),
        };

        self.advance().unwrap();

        Ok(k)
    }

    fn subroutine_kind(&mut self) -> Result<ast::SubroutineKind> {
        let k = match self.current() {
            Some(Keyword(Constructor)) => ast::SubroutineKind::Constructor,
            Some(Keyword(Function)) => ast::SubroutineKind::Function,
            Some(Keyword(Method)) => ast::SubroutineKind::Method,
            _ => return Err(self.unexpected_token("`constructor` | `function` | `method`")),
        };

        self.advance().unwrap();

        Ok(k)
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<ast::Parameter>> {
        let mut params = vec![];

        if self.next_token(|t| m!(t, Keyword(Int | Char | Boolean) | Identifier(..))) {
            let r#type = self.r#type()?;
            let name = self.identifier()?;

            params.push(ast::Parameter { r#type, name });

            while self.next_token(|t| m!(t, Symbol(Comma))) {
                self.advance().unwrap();
                let r#type = self.r#type()?;
                let name = self.identifier()?;
                params.push(ast::Parameter { r#type, name });
            }
        }

        Ok(params)
    }

    fn parse_subroutine_body(&mut self) -> Result<ast::SubroutineBody> {
        self.token(Symbol(OpenBrace))?;

        let mut vars = vec![];

        while self.next_token(|t| t == Keyword(Var)) {
            self.token(Keyword(Var))?;

            let r#type = self.r#type()?;
            let name = self.identifier()?;
            let mut names = vec![name];

            while self.next_token(|t| m!(t, Symbol(Comma))) {
                self.token(Symbol(Comma))?;
                let name = self.identifier()?;
                names.push(name);
            }

            vars.push(ast::SubroutineVar { r#type, names });
            self.token(Symbol(Semi))?;
        }

        let statements = self.parse_statements()?;
        self.token(Symbol(CloseBrace))?;

        Ok(ast::SubroutineBody { vars, statements })
    }

    fn parse_statements(&mut self) -> Result<Vec<ast::Statement>> {
        let mut statements = vec![];

        while self.next_token(|t| m!(t, Keyword(Let | If | While | Do | Return))) {
            let s = match self.current() {
                Some(Keyword(Let)) => self.parse_let(),
                Some(Keyword(If)) => self.parse_if(),
                Some(Keyword(While)) => self.parse_while(),
                Some(Keyword(Return)) => self.parse_return(),
                Some(Keyword(Do)) => self.parse_do(),
                _ => unreachable!(),
            };

            statements.push(s?);
        }

        Ok(statements)
    }

    fn parse_let(&mut self) -> Result<ast::Statement> {
        self.token(Keyword(Let))?;

        let var = self.identifier()?;
        let mut accessor = None;

        if self.next_token(|t| t == Symbol(OpenBracket)) {
            self.advance().unwrap();
            let expression = self.parse_expression()?;
            self.token(Symbol(CloseBracket))?;
            accessor = Some(expression);
        }

        self.token(Symbol(Eq))?;
        let expression = self.parse_expression()?;
        self.token(Symbol(Semi))?;

        Ok(ast::Statement::Let {
            var,
            accessor,
            expression,
        })
    }

    fn parse_if(&mut self) -> Result<ast::Statement> {
        self.token(Keyword(If))?;
        self.token(Symbol(OpenParen))?;
        let cond = self.parse_expression()?;
        self.token(Symbol(CloseParen))?;
        self.token(Symbol(OpenBrace))?;
        let statements = self.parse_statements()?;
        self.token(Symbol(CloseBrace))?;

        let r#else = {
            if self.next_token(|t| t == Keyword(Else)) {
                self.token(Keyword(Else))?;
                self.token(Symbol(OpenBrace))?;
                let statements = self.parse_statements()?;
                self.token(Symbol(CloseBrace))?;
                Some(statements)
            } else {
                None
            }
        };

        Ok(ast::Statement::If {
            cond,
            statements,
            r#else,
        })
    }

    fn parse_while(&mut self) -> Result<ast::Statement> {
        self.token(Keyword(While))?;
        self.token(Symbol(OpenParen))?;
        let cond = self.parse_expression()?;
        self.token(Symbol(CloseParen))?;
        self.token(Symbol(OpenBrace))?;
        let body = self.parse_statements()?;
        self.token(Symbol(CloseBrace))?;

        Ok(ast::Statement::While { cond, body })
    }

    fn parse_return(&mut self) -> Result<ast::Statement> {
        self.token(Keyword(Return))?;

        let mut value = None;

        if self.next_token(|t| t != Symbol(Semi)) {
            value = Some(self.parse_expression()?);
        };

        self.token(Symbol(Semi))?;

        Ok(ast::Statement::Return { value })
    }

    fn parse_do(&mut self) -> Result<ast::Statement> {
        self.token(Keyword(Do))?;

        let ident = self.identifier()?;
        let subroutine_call = self.parse_subroutine_call(ident)?;
        self.token(Symbol(Semi))?;

        Ok(ast::Statement::Do { subroutine_call })
    }

    fn parse_expression(&mut self) -> Result<ast::Expression> {
        let term = self.parse_term()?;

        let mut operations = vec![];

        while let Ok(op) = self.op() {
            let term = self.parse_term()?;
            operations.push((op, term));
        }

        Ok(ast::Expression(term, operations))
    }

    fn parse_subroutine_call(&mut self, ident: String) -> Result<ast::SubroutineCall> {
        let (name, receiver) = match self.current() {
            Some(Symbol(Dot)) => {
                self.advance().unwrap();
                (self.identifier()?, Some(ident))
            }
            _ => (ident, None),
        };

        self.token(Symbol(OpenParen))?;
        let expressions = self.parse_expression_list()?;
        self.token(Symbol(CloseParen))?;

        match (name, receiver) {
            (name, Some(receiver)) => Ok(ast::SubroutineCall::Method {
                name,
                receiver,
                expressions,
            }),
            (name, None) => Ok(ast::SubroutineCall::Function { name, expressions }),
        }
    }

    fn parse_expression_list(&mut self) -> Result<Vec<ast::Expression>> {
        let mut expressions = vec![];

        if self.next_token(|t| t != Symbol(CloseParen)) {
            expressions.push(self.parse_expression()?);

            while self.next_token(|t| t == Symbol(Comma)) {
                self.advance().unwrap();
                expressions.push(self.parse_expression()?);
            }
        }

        Ok(expressions)
    }

    pub fn parse_term(&mut self) -> Result<ast::Term> {
        let t = match self.advance() {
            Some(IntegerConstant(i)) => ast::Term::Int(i),
            Some(StringConstant(s)) => ast::Term::Str(s),
            Some(Keyword(True)) => ast::Term::Keyword(ast::Keyword::True),
            Some(Keyword(False)) => ast::Term::Keyword(ast::Keyword::False),
            Some(Keyword(Null)) => ast::Term::Keyword(ast::Keyword::Null),
            Some(Keyword(This)) => ast::Term::Keyword(ast::Keyword::This),
            Some(Symbol(Minus)) => {
                let term = self.parse_term()?;
                ast::Term::Unary(ast::UnaryOp::Minus, Box::new(term))
            }
            Some(Symbol(Tilde)) => {
                let term = self.parse_term()?;
                ast::Term::Unary(ast::UnaryOp::Not, Box::new(term))
            }
            Some(Symbol(OpenParen)) => {
                let expr = self.parse_expression()?;
                self.token(Symbol(CloseParen))?;
                ast::Term::Paren(Box::new(expr))
            }
            Some(Identifier(id)) => match self.current() {
                Some(Symbol(OpenParen | Dot)) => ast::Term::Call(self.parse_subroutine_call(id)?),
                Some(Symbol(OpenBracket)) => {
                    self.advance().unwrap();
                    let expr = self.parse_expression()?;
                    self.token(Symbol(CloseBracket))?;
                    ast::Term::Index(id, Box::new(expr))
                }
                _ => ast::Term::Var(id),
            },
            _ => return Err(self.unexpected_token("`term`")),
        };

        Ok(t)
    }
}
