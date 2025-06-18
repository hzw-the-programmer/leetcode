use crate::utils::binary_tree::TreeNode;
use core::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    // time : O(n)
    // space: O(n)
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

    // time : O(n)
    // space: O(n)
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
        values = &values[1..];

        let mut queue = VecDeque::from(vec![root.clone()]);
        while let Some(node) = queue.pop_front() {
            let node = node.borrow_mut();
            let children = RefMut::map_split(node, |node| (&mut node.left, &mut node.right));
            let children = vec![children.0, children.1];

            for mut child in children {
                if values.is_empty() {
                    break;
                }
                if let Some(val) = values[0] {
                    let node = Rc::new(RefCell::new(TreeNode::new(val)));
                    *child = Some(node.clone());
                    queue.push_back(node);
                }
                values = &values[1..];
            }
        }

        Some(root)
    }
}
