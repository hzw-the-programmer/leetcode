// 208. Implement Trie (Prefix Tree)

struct Node {
    children: [Option<Box<Node>>; 26],
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: [const { None }; 26],
            is_end: false,
        }
    }
}

pub struct Trie {
    root: Box<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(Node::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }

        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        for c in word.chars() {
            let index = c as usize - 'a' as usize;

            node = match node.children[index].as_ref() {
                None => return false,
                Some(child) => child,
            }
        }

        node.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;

        for c in prefix.chars() {
            let index = c as usize - 'a' as usize;

            node = match node.children[index].as_ref() {
                None => return false,
                Some(child) => child,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests;
