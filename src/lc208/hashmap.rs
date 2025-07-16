use std::collections::HashMap;

pub struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(Trie::new);
        }
        node.end = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.find(&word).map_or(false, |node| node.end)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }

    fn find(&self, word: &str) -> Option<&Trie> {
        let mut node = self;
        for c in word.chars() {
            node = node.children.get(&c)?;
        }
        Some(node)
    }
}
