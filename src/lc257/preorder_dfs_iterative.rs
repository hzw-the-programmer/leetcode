use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut res = vec![];
    let mut temp = String::new();
    match root {
        None => {}
        Some(node) => {
            let mut stack = vec![(node, 0)];
            while let Some((node, len)) = stack.pop() {
                temp.truncate(len);
                let mut current = Some(node);
                while let Some(node) = current.take() {
                    let node = node.borrow();
                    temp.push_str(&node.val.to_string());
                    temp.push_str("->");
                    match (node.left.clone(), node.right.clone()) {
                        (None, None) => {
                            let mut t = temp.clone();
                            t.truncate(t.len() - 2);
                            res.push(t);
                        }
                        (left, right) => {
                            current = left;
                            if let Some(node) = right {
                                stack.push((node, temp.len()));
                            }
                        }
                    }
                }
            }
        }
    }
    res
}
