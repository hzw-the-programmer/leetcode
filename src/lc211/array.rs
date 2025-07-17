struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: [const { None }; 26],
            end: false,
        }
    }
}

pub struct WordDictionary {
    root: Trie,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self { root: Trie::new() }
    }

    pub fn add_word(&mut self, word: String) {
        word.chars()
            .map(|c| Self::index(c))
            .fold(&mut self.root, |trie, idx| {
                trie.children[idx].get_or_insert_with(|| Box::new(Trie::new()))
            })
            .end = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::find(&self.root, &word).map_or(false, |trie| trie.end)
    }

    fn index(c: char) -> usize {
        c as usize - 'a' as usize
    }

    fn find<'a>(mut trie: &'a Trie, word: &'a str) -> Option<&'a Trie> {
        for (i, c) in word.chars().enumerate() {
            if c == '.' {
                for child in trie.children.iter().flatten() {
                    if let Some(trie) = Self::find(child, &word[i + 1..]) {
                        if trie.end {
                            return Some(trie);
                        }
                    }
                }
                return None;
            } else {
                trie = trie.children[Self::index(c)].as_deref()?;
            }
        }

        Some(trie)
    }
}
