use cursor::Cursor;
use LiteralKind::*;
use TokenKind::*;

mod cursor;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

// TODO: remove these derive traits
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    LineComment,
    BlockComment,
    Whitespace,

    /// "ident" or "continue"
    ///
    /// At this step, keywords are also considered identifiers.
    Ident,

    Literal(LiteralKind),

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

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,

    /// End of input.
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LiteralKind {
    Int,
    Str,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);

    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != Eof {
            Some(token)
        } else {
            None
        }
    })
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

#[allow(clippy::manual_is_ascii_check)]
fn is_id_start(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '_')
}

fn is_id_continue(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_')
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(Eof, 0),
        };
        let token_kind = match first_char {
            // Slash, comment or block comment.
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if is_id_start(c) => self.ident(),

            // Numeric literal.
            '0'..='9' => {
                let literal_kind = self.eat_digits();
                TokenKind::Literal(literal_kind)
            }

            // One-symbol tokens.
            ';' => Semi,
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '~' => Tilde,
            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '-' => Minus,
            '&' => And,
            '|' => Or,
            '+' => Plus,
            '*' => Star,

            // String literal.
            '"' => {
                self.double_quoted_string();
                Literal(Str)
            }

            _ => Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump();
        self.eat_while(|c| c != '\n');
        LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.bump();

        let mut depth = 1usize;

        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        // This block comment is closed, so for a construction like "/* */ */"
                        // there will be a successfully parsed block comment "/* */"
                        // and " */" will be processed separately.
                        break;
                    }
                }
                _ => (),
            }
        }
        BlockComment
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        Whitespace
    }
    fn double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.bump() {
            if c == '"' {
                return true;
            }
        }
        false
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(is_id_continue);
        Ident
    }

    fn eat_digits(&mut self) -> LiteralKind {
        while let '0'..='9' = self.first() {
            self.bump();
        }
        Int
    }
}
