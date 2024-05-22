use std::{error, fmt::Display, result};

use crate::{lexer, tokenize};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Keyword(KeywordKind),
    Symbol(SymbolKind),
    StringConstant(&'a str),
    IntegerConstant(u16),
    Identifier(&'a str),
}

impl<'a> PartialEq<&'a Token<'_>> for Token<'_> {
    fn eq(&self, other: &&'a Token) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Token<'_>> for &'a Token<'_> {
    fn eq(&self, other: &Token) -> bool {
        *self == other
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeywordKind {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "."
    Dot,
    /// ","
    Comma,
    /// ";"
    Semi,
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "&"
    And,
    /// "|"
    Or,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "="
    Eq,
    /// "~"
    Tilde,
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn parse(input: &str) -> impl Iterator<Item = Result<Token<'_>>> + '_ {
    use lexer::TokenKind::*;

    let mut pos = 0;

    let it = tokenize(input)
        .map(move |t| {
            let value = &input[pos..pos + t.len];
            pos += t.len;
            (t.kind, value)
        })
        .filter(|(k, ..)| !matches!(k, LineComment | BlockComment | Whitespace))
        .map(Token::try_from);

    it
}

impl<'a> TryFrom<(lexer::TokenKind, &'a str)> for Token<'a> {
    type Error = Box<dyn error::Error>;

    fn try_from((kind, value): (lexer::TokenKind, &'a str)) -> result::Result<Self, Self::Error> {
        use lexer::LiteralKind::{Int as lInt, Str as lStr};
        use lexer::TokenKind as lt;
        use KeywordKind::*;
        use SymbolKind::*;
        use Token::*;

        let token = match kind {
            lt::Literal(lInt) => IntegerConstant(value.parse()?),
            lt::Literal(lStr) => {
                let mut chars = value.chars();
                chars.next();
                chars.next_back();
                StringConstant(chars.as_str())
            }
            lt::OpenParen => Symbol(OpenParen),
            lt::CloseParen => Symbol(CloseParen),
            lt::OpenBrace => Symbol(OpenBrace),
            lt::CloseBrace => Symbol(CloseBrace),
            lt::OpenBracket => Symbol(OpenBracket),
            lt::CloseBracket => Symbol(CloseBracket),
            lt::Dot => Symbol(Dot),
            lt::Comma => Symbol(Comma),
            lt::Semi => Symbol(Semi),
            lt::Plus => Symbol(Plus),
            lt::Minus => Symbol(Minus),
            lt::Star => Symbol(Star),
            lt::Slash => Symbol(Slash),
            lt::And => Symbol(And),
            lt::Or => Symbol(Or),
            lt::Lt => Symbol(Lt),
            lt::Gt => Symbol(Gt),
            lt::Eq => Symbol(Eq),
            lt::Tilde => Symbol(Tilde),
            lt::Ident => match value {
                "class" => Keyword(Class),
                "constructor" => Keyword(Constructor),
                "function" => Keyword(Function),
                "method" => Keyword(Method),
                "field" => Keyword(Field),
                "static" => Keyword(Static),
                "var" => Keyword(Var),
                "int" => Keyword(Int),
                "char" => Keyword(Char),
                "boolean" => Keyword(Boolean),
                "void" => Keyword(Void),
                "true" => Keyword(True),
                "false" => Keyword(False),
                "null" => Keyword(Null),
                "this" => Keyword(This),
                "let" => Keyword(Let),
                "do" => Keyword(Do),
                "if" => Keyword(If),
                "else" => Keyword(Else),
                "while" => Keyword(While),
                "return" => Keyword(Return),
                _ => Identifier(value),
            },
            _ => Err(format!("cannot parse Token from: ({kind:?} '{value}')"))?,
        };

        Ok(token)
    }
}

impl Token<'_> {
    pub fn variant_matches(&self, other: Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&other)
    }
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SymbolKind::*;

        let s = match self {
            OpenParen => "(",
            CloseParen => ")",
            OpenBrace => "{",
            CloseBrace => "}",
            OpenBracket => "[",
            CloseBracket => "]",
            Dot => ".",
            Comma => ",",
            Semi => ";",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            And => "&",
            Or => "|",
            Lt => "<",
            Gt => ">",
            Eq => "=",
            Tilde => "~",
        };
        write!(f, "{s}")
    }
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use KeywordKind::*;
        let s = match self {
            Class => "class",
            Constructor => "constructor",
            Function => "function",
            Method => "method",
            Field => "field",
            Static => "static",
            Var => "var",
            Int => "int",
            Char => "char",
            Boolean => "boolean",
            Void => "void",
            True => "true",
            False => "false",
            Null => "null",
            This => "this",
            Let => "let",
            Do => "do",
            If => "if",
            Else => "else",
            While => "while",
            Return => "return",
        };

        write!(f, "{s}")
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Token::*;
        match *self {
            Keyword(kw) => write!(f, "{kw}"),
            Symbol(sym) => write!(f, "{sym}"),
            StringConstant(s) => write!(f, "{s}"),
            IntegerConstant(i) => write!(f, "{i}"),
            Identifier(id) => write!(f, "{id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variance() {
        let a = Token::Keyword(KeywordKind::Class);
        let b = Token::Keyword(KeywordKind::Class);

        assert!(a.variant_matches(b));

        let b = Token::Symbol(SymbolKind::Dot);
        assert!(!a.variant_matches(b));

        let a = Token::StringConstant("hello");
        let b = Token::StringConstant("world");
        assert!(a.variant_matches(b));
    }
}
