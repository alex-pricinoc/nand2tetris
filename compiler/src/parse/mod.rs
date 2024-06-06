use std::{error, fmt, result};

use crate::tokenize::*;
use stream::Stream;

pub mod ast;
mod class;
mod stream;

type Result<T> = result::Result<T, ParseError>;

pub fn parse(tokens: Vec<Token>) -> Result<ast::Ast> {
    let mut stream = Stream::new(tokens);
    let class = stream.parse_class()?;

    Ok(ast::Ast { class })
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl error::Error for ParseError {}
