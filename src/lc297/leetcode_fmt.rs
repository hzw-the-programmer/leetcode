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
                res.push_str(&node.borrow().val.to_string());
                let mut queue = VecDeque::new();
                queue.push_back(node.borrow_mut().left.take());
                queue.push_back(node.borrow_mut().right.take());
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

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}
