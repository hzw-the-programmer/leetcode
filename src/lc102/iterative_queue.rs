use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();

    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            let n = queue.len();
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    v.push(node.val);
                    if let Some(node) = node.left.clone() {
                        queue.push_back(node);
                    }
                    if let Some(node) = node.right.clone() {
                        queue.push_back(node);
                    }
                }
            }
            res.push(v);
        }
    }

    res
}
