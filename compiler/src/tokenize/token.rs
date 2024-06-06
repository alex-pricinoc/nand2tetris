use std::fmt;

impl PartialEq<Token> for &Token {
    fn eq(&self, other: &Token) -> bool {
        *self == other
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(KeywordKind),
    Symbol(SymbolKind),
    StringConstant(String),
    IntegerConstant(u16),
    Identifier(String),
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

impl Token {
    #[must_use]
    pub fn variant_matches(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    #[must_use]
    pub fn is_class(&self) -> bool {
        match self {
            Token::Identifier(id) => id.chars().next().is_some_and(char::is_uppercase),
            _ => false,
        }
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SymbolKind::*;

        let s = match self {
            OpenParen => '(',
            CloseParen => ')',
            OpenBrace => '{',
            CloseBrace => '}',
            OpenBracket => '[',
            CloseBracket => ']',
            Dot => '.',
            Comma => ',',
            Semi => ';',
            Plus => '+',
            Minus => '-',
            Star => '*',
            Slash => '/',
            And => '&',
            Or => '|',
            Lt => '<',
            Gt => '>',
            Eq => '=',
            Tilde => '~',
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        match &self {
            Keyword(kw) => write!(f, "{kw}"),
            Symbol(sym) => write!(f, "{sym}"),
            StringConstant(s) => write!(f, "{s}"),
            IntegerConstant(i) => write!(f, "{i}"),
            Identifier(id) => write!(f, "{id}"),
        }
    }
}
