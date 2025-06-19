use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Codec {
    none: String,
    delimiter: String,
    start: String,
    end: String,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            none: "#".to_string(),
            delimiter: ",".to_string(),
            start: String::new(),
            end: String::new(),
        }
    }

    pub fn with(none: String, delimiter: String, start: String, end: String) -> Self {
        Self {
            none,
            delimiter,
            start,
            end,
        }
    }

    // time : O(n)
    // space: O(n)
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        res.push_str(&self.start);

        let mut stack = Vec::new();
        stack.push(root);
        while let Some(node) = stack.pop() {
            match node {
                None => {
                    res.push_str(&self.none);
                    res.push_str(&self.delimiter);
                }
                Some(node) => {
                    let node = node.borrow();
                    res.push_str(&node.val.to_string());
                    res.push_str(&self.delimiter);
                    stack.push(node.right.clone());
                    stack.push(node.left.clone());
                }
            }
        }
        res.truncate(res.len() - self.delimiter.len());

        res.push_str(&self.end);
        res
    }

    // time : O(n)
    // space: O(n)
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let values = data
            .trim_start_matches(&self.start)
            .trim_end_matches(&self.end)
            .split(&self.delimiter)
            .map(|s| match s {
                "#" => None,
                _ => s.parse().ok(),
            })
            .collect::<Vec<Option<i32>>>();

        Self::from_values(&values)
    }

    fn from_values(mut values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        values = &values[1..];

        let mut stack = vec![root.clone()];
        let mut none_count = 0;
        for value in values {
            let top = stack.last().unwrap();
            if let Some(val) = value {
                let node = Rc::new(RefCell::new(TreeNode::new(*val)));
                if none_count == 1 || top.borrow().left.is_some() {
                    top.borrow_mut().right = Some(node.clone());
                    stack.pop();
                    none_count = 0;
                } else {
                    top.borrow_mut().left = Some(node.clone());
                }
                stack.push(node);
            } else {
                none_count += 1;
                if none_count == 2 || top.borrow().left.is_some() {
                    stack.pop();
                    none_count = 0;
                }
            }
        }

        Some(root)
    }
}
