use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut res = vec![];

    match root {
        None => {}
        Some(node) => {
            let mut queue = VecDeque::new();
            queue.push_front((node, String::new()));
            while let Some((node, mut path)) = queue.pop_front() {
                let node = node.borrow();
                path.push_str(&node.val.to_string());
                match (node.left.clone(), node.right.clone()) {
                    (None, None) => res.push(path),
                    (Some(node), None) | (None, Some(node)) => {
                        path.push_str("->");
                        queue.push_front((node, path));
                    }
                    (Some(left), Some(right)) => {
                        path.push_str("->");
                        queue.push_front((left, path.clone()));
                        queue.push_front((right, path));
                    }
                }
            }
        }
    }

    res
}
