use std::io::{self, BufRead, BufReader, Read};
use std::iter::Peekable;

type Stream = Box<dyn Iterator<Item = io::Result<String>>>;

pub struct Parser {
    lines: Peekable<Stream>,
    pub current_command: Option<String>,
    next_address: u16,
}

impl Parser {
    pub fn build(reader: impl Read + 'static) -> io::Result<Self> {
        let reader = BufReader::new(reader);

        let iter = reader.lines();

        let lines = Box::new(iter) as Stream;

        Ok(Self {
            lines: lines.peekable(),
            current_command: None,
            next_address: 0,
        })
    }

    /// Are there more commands in the input?
    pub fn has_more_commands(&mut self) -> bool {
        if let Some(Ok(line)) = self.lines.peek() {
            if line.trim().starts_with("//") {
                self.lines.next();

                self.has_more_commands()
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Reads the next command from the input and makes it the current
    /// command. Should be called only if has_more_commands() is true.
    /// Initially there is no current command.
    pub fn advance(&mut self) {
        let line = self.lines.next().unwrap().unwrap();
        let offset = line.find("//").unwrap_or(line.len());
        let line = line[0..offset].trim().to_owned();

        self.current_command = Some(line);

        match self.command_type() {
            "A_COMMAND" | "C_COMMAND" => {
                self.next_address += 1;
            }
            _ => (),
        }
    }

    /// Returns the type of the current command:
    /// A_COMMAND for @Xxx where Xxx is either a symbol or a decimal number
    /// C_COMMAND for dest=comp;jump
    /// L_COMMAND (actually, pseudo-command) for (Xxx) where Xxx is a symbol.
    pub fn command_type(&self) -> &'static str {
        let current_command = self.current_command.as_ref().unwrap();

        if current_command.starts_with('@') {
            "A_COMMAND"
        } else if current_command.starts_with('(') {
            "L_COMMAND"
        } else {
            "C_COMMAND"
        }
    }

    ///Returns the symbol or decimalXxx of the current command
    ///@Xxx or (Xxx). Should be called only when command_type() is
    ///A_COMMAND or L_COMMAND.
    pub fn symbol(&self) -> &str {
        let current_command = self.current_command.as_ref().unwrap();

        if self.command_type() == "A_COMMAND" {
            current_command.split_once('@').unwrap().1
        } else {
            let mut chars = current_command.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        }
    }

    /// Returns the dest mnemonic in the current C-command (8 possibilities).
    /// Should be called only when command_type() is C_COMMAND.
    pub fn dest(&self) -> &str {
        if let Some((dest, _)) = self.current_command.as_ref().unwrap().split_once('=') {
            dest
        } else {
            "null"
        }
    }

    /// Returns the comp mnemonic in the current C-command (28 possibilities).
    ///Should be called only when command_type() is C_COMMAND.
    pub fn comp(&self) -> &str {
        let command = self.current_command.as_ref().unwrap();

        let start = command.find('=').map(|i| i + 1).unwrap_or(0);
        let end = command.find(';').unwrap_or(command.len());

        &command[start..end]
    }

    /// Returns the jump mnemonic in the current C-command (8 possibilities).
    /// Should be called only when command_type() is C_COMMAND.
    pub fn jump(&self) -> &str {
        if let Some((_, jump)) = self.current_command.as_ref().unwrap().split_once(';') {
            jump
        } else {
            "null"
        }
    }

    pub fn next_address(&self) -> u16 {
        self.next_address
    }
}
