use std::collections::HashMap;

#[derive(Default)]
pub struct Labels {
    counts: HashMap<Label, usize>,
}

impl Labels {
    pub fn clear(&mut self) {
        self.counts.clear();
    }
    pub fn generate(&mut self, label: Label) -> usize {
        let entry = self.counts.entry(label).or_default();
        let index = *entry;
        *entry += 1;

        index
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Label {
    If,
    While,
}
