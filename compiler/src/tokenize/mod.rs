use std::error;

pub use token::*;

pub mod lexer;
pub mod token;

pub fn tokenize(input: &str) -> impl Iterator<Item = Result<Token, Box<dyn error::Error>>> + '_ {
    use lexer::TokenKind::*;

    let mut pos = 0;
    lexer::tokenize(input)
        .map(move |t| {
            let value = &input[pos..pos + t.len];
            pos += t.len;
            (t.kind, value)
        })
        .filter(|(k, _)| !matches!(k, LineComment | BlockComment | Whitespace))
        .map(Token::try_from)
}

impl TryFrom<(lexer::TokenKind, &str)> for Token {
    type Error = Box<dyn error::Error>;

    fn try_from((kind, value): (lexer::TokenKind, &str)) -> Result<Self, Self::Error> {
        use lexer::LiteralKind::{Int as lInt, Str as lStr};
        use lexer::TokenKind as lt;
        use KeywordKind::*;
        use SymbolKind::*;
        use Token::*;

        let token = match kind {
            lt::Literal(lStr) => {
                let mut chars = value.chars();
                chars.next();
                chars.next_back();
                StringConstant(chars.as_str().into())
            }
            lt::Literal(lInt) => IntegerConstant(value.parse()?),
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
                _ => Identifier(value.into()),
            },
            _ => Err(format!("cannot parse Token from: ({kind:?} '{value}')"))?,
        };

        Ok(token)
    }
}
