use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
pub struct Cursor<'a> {
    pub len_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    pub chars: Chars<'a>,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
        }
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }

    pub fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Checks if there is nothing more to consume.
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns amount of already consumed symbols.
    pub fn pos_within_token(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to 0.
    pub fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
