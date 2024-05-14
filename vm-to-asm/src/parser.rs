use crate::CommandType;

#[derive(Debug)]
pub struct Parser {
    source: Vec<String>,
    line: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let source = input
            .lines()
            .map(|l| {
                let offset = l.find("//").unwrap_or(l.len());
                let line = l[0..offset].trim();
                line
            })
            .filter(|&l| !l.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        Parser { source, line: 0 }
    }

    pub fn advance(&mut self) {
        self.line += 1;
    }

    pub fn has_more_commands(&self) -> bool {
        self.line < self.source.len()
    }

    pub fn command_type(&self) -> CommandType {
        let line = &self.source[self.line];
        let mut words = line.split_whitespace();

        use CommandType::*;
        match words.next().unwrap() {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => Arithmetic,
            "push" => Push,
            "pop" => Pop,
            "label" => Label,
            "goto" => Goto,
            "if-goto" => If,
            "function" => Function,
            "call" => Call,
            "return" => Return,
            c => panic!("invalid command {} at line {}", c, self.line + 1),
        }
    }

    pub fn arg1(&self) -> &str {
        let line = &self.source[self.line];
        let mut words = line.split_whitespace();

        let cmd = words.next().unwrap();

        match cmd {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => cmd,
            "pop" | "push" | "label" | "goto" | "if-goto" | "function" | "call" => {
                words.next().unwrap()
            }
            c => panic!("invalid command {} at line {}", c, self.line + 1),
        }
    }

    pub fn arg2(&self) -> u16 {
        let line = &self.source[self.line];
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "push" | "pop" | "function" | "call" => {
                words.next().unwrap();
                words.next().unwrap().parse().unwrap()
            }
            s => panic!("invalid instruction {s}"),
        }
    }
}
