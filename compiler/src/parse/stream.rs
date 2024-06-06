use std::fmt;
use std::iter::Peekable;

use crate::tokenize::{self, KeywordKind::*, SymbolKind::*, Token::*};
use tokenize::Token;

use super::{ast, ParseError, Result};

type TokenStream = Box<dyn Iterator<Item = Token>>;

pub struct Stream {
    tokens: Peekable<TokenStream>,
}

impl Stream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: (Box::new(tokens.into_iter()) as TokenStream).peekable(),
        }
    }

    #[must_use]
    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn token(&mut self, token: Token) -> Result<Token> {
        let t = self
            .eat_if(|t| t == token)
            .ok_or_else(|| self.unexpected_token(&token))?;

        Ok(t)
    }

    pub fn next_token(&mut self, f: impl FnOnce(&Token) -> bool) -> bool {
        self.current().is_some_and(f)
    }

    pub fn identifier(&mut self) -> Result<String> {
        let id = match self.current() {
            Some(Token::Identifier(id)) => id.clone(),
            _ => return Err(self.unexpected_token("`identifier`")),
        };

        self.advance().unwrap();

        Ok(id)
    }

    pub fn r#type(&mut self) -> Result<ast::Type> {
        let t = match self.current() {
            Some(Keyword(Int)) => ast::Type::Int,
            Some(Keyword(Char)) => ast::Type::Char,
            Some(Keyword(Boolean)) => ast::Type::Boolean,
            Some(Identifier(id)) => ast::Type::ClassName(id.clone()),
            _ => Err(self.unexpected_token("`int` | `char` | `boolean` | className"))?,
        };

        self.advance().unwrap();

        Ok(t)
    }

    pub fn op(&mut self) -> Result<ast::Op> {
        let op = match self.current() {
            Some(Symbol(Plus)) => ast::Op::Plus,
            Some(Symbol(Minus)) => ast::Op::Minus,
            Some(Symbol(Star)) => ast::Op::Star,
            Some(Symbol(Slash)) => ast::Op::Div,
            Some(Symbol(And)) => ast::Op::And,
            Some(Symbol(Or)) => ast::Op::Or,
            Some(Symbol(Lt)) => ast::Op::Lt,
            Some(Symbol(Gt)) => ast::Op::Gt,
            Some(Symbol(Eq)) => ast::Op::Eq,
            _ => Err(self.unexpected_token("`+` | `-` | `*` | `/` | `&` | `|` | `<` | `>` | `=`"))?,
        };

        self.advance().unwrap();

        Ok(op)
    }

    #[must_use]
    fn eat_if(&mut self, f: impl FnOnce(&Token) -> bool) -> Option<Token> {
        self.current()
            .is_some_and(f)
            .then(|| self.advance().unwrap())
    }

    #[must_use]
    pub fn current(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn unexpected_token(&mut self, expected: impl fmt::Display) -> ParseError {
        ParseError(format!(
            "expected `{expected}`, found: {current:?}",
            current = self.current()
        ))
    }
}
