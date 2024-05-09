#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    fields: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.fields.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.fields.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.fields.get(symbol).copied()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        let registers = (0..=15).map(|i| (format!("R{i}"), i));

        let predefined_symbols = [
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("SCREEN", 16384),
            ("KBD", 24576),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v));

        let iter = registers.chain(predefined_symbols);

        let fields = HashMap::from_iter(iter);

        Self { fields }
    }
}
