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
        let mut none_count = 0;
        for value in iter {
            if queue.front().is_none() {
                break;
            }

            let front = queue.front().unwrap();
            match value? {
                None => {
                    none_count += 1;
                    if none_count == 2 || front.borrow().left.is_some() {
                        queue.pop_front();
                        none_count = 0;
                    }
                }
                Some(val) => {
                    let node = Rc::new(RefCell::new(TreeNode::new(val)));
                    if none_count == 1 || front.borrow().left.is_some() {
                        front.borrow_mut().right = Some(node.clone());
                        queue.pop_front();
                        none_count = 0;
                    } else {
                        front.borrow_mut().left = Some(node.clone());
                    }
                    queue.push_back(node);
                }
            }
        }

        Ok(Some(root))
    }
}
