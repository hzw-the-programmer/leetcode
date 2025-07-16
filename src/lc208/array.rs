struct Node {
    children: [Option<Box<Node>>; 26],
    end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: [const { None }; 26],
            end: false,
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
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

        node.end = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.find(word).map_or(false, |node| node.end)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: String) -> Option<&Node> {
        let mut node = &self.root;

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
