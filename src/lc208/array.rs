pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: [const { None }; 26],
            end: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;

        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(Trie::new()));
            }
            node = node.children[index].as_deref_mut().unwrap();
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
            let index = c as usize - 'a' as usize;
            if node.children[index].is_none() {
                return None;
            }
            node = node.children[index].as_deref().unwrap();
        }

        Some(node)
    }
}
