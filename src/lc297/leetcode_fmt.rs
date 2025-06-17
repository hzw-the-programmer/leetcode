use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        res.push('[');
        match root {
            None => {}
            Some(node) => {
                let mut node = node.borrow_mut();
                res.push_str(&node.val.to_string());
                let mut queue = VecDeque::from(vec![node.left.take(), node.right.take()]);
                while let Some(node) = queue.pop_front() {
                    match node {
                        None => {
                            res.push_str(",null");
                        }
                        Some(node) => {
                            let mut node = node.borrow_mut();
                            res.push(',');
                            res.push_str(&node.val.to_string());
                            queue.push_back(node.left.take());
                            queue.push_back(node.right.take());
                        }
                    }
                    if queue.iter().all(|node| node.is_none()) {
                        break;
                    }
                }
            }
        }
        res.push(']');
        res
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let values = data
            .trim_matches(&['[', ']'])
            .split(',')
            .map(|s| match s {
                "null" => None,
                _ => s.parse().ok(),
            })
            .collect::<Vec<Option<i32>>>();
        Self::from_slice(&values)
    }

    fn from_slice(mut values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::from(vec![root.clone()]);
        values = &values[1..];

        while let Some(node) = queue.pop_front() {
            let mut node = node.borrow_mut();

            if values.is_empty() {
                break;
            }
            if let Some(val) = values[0] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                queue.push_back(left);
            }
            values = &values[1..];

            if values.is_empty() {
                break;
            }
            if let Some(val) = values[0] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                queue.push_back(right);
            }
            values = &values[1..];
        }

        Some(root)
    }
}
