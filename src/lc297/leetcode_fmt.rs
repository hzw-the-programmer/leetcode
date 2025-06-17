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
                            queue.push_back(None);
                            queue.push_back(None);
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
        Self::recursive(&values, 0)
    }

    fn recursive(values: &[Option<i32>], idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if idx >= values.len() {
            return None;
        }

        match values[idx] {
            None => None,
            Some(val) => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::recursive(values, 2 * idx + 1),
                right: Self::recursive(values, 2 * idx + 2),
            }))),
        }
    }
}
