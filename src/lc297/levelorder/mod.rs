use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use core::num::ParseIntError;
use std::collections::VecDeque;
use std::rc::Rc;

#[cfg(test)]
mod tests;

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
            root => {
                let mut queue = VecDeque::new();
                queue.push_back(root);
                while let Some(node) = queue.pop_front() {
                    match node {
                        None => {
                            res.push_str("null,");
                        }
                        Some(node) => {
                            let node = node.borrow();
                            res.push_str(&node.val.to_string());
                            res.push(',');
                            queue.push_back(node.left.clone());
                            queue.push_back(node.right.clone());
                        }
                    }
                    if queue.iter().all(|node| node.is_none()) {
                        break;
                    }
                }
                res.truncate(res.len() - 1);
            }
        }
        res.push(']');
        res
    }

    // time : O(n)
    // space: O(n)
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let iter = data.trim_matches(&['[', ']']).split(',').map(|s| match s {
            "null" | "" => Ok(None),
            _ => s.parse().map(|val| Some(val)),
        });

        Self::from_iter(iter).unwrap()
    }

    fn from_iter(
        mut iter: impl Iterator<Item = Result<Option<i32>, ParseIntError>>,
    ) -> Result<Option<Rc<RefCell<TreeNode>>>, ParseIntError> {
        let root = match iter.next() {
            None | Some(Ok(None)) => return Ok(None),
            Some(Ok(Some(val))) => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(Err(err)) => return Err(err),
        };

        let mut queue = VecDeque::from(vec![root.clone()]);
        while let Some(front) = queue.pop_front() {
            if let Some(value) = iter.next() {
                if let Some(val) = value? {
                    let node = Rc::new(RefCell::new(TreeNode::new(val)));
                    front.borrow_mut().left = Some(node.clone());
                    queue.push_back(node);
                }
                if let Some(value) = iter.next() {
                    if let Some(val) = value? {
                        let node = Rc::new(RefCell::new(TreeNode::new(val)));
                        front.borrow_mut().right = Some(node.clone());
                        queue.push_back(node);
                    }
                }
            }
        }

        Ok(Some(root))
    }
}
