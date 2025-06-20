use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        let mut is_left = true;
        while !queue.is_empty() {
            let mut que = VecDeque::with_capacity(queue.len());
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    let mut node = node.borrow_mut();
                    if is_left {
                        que.push_back(node.val);
                    } else {
                        que.push_front(node.val);
                    }
                    if let Some(node) = node.left.take() {
                        queue.push_back(node);
                    }
                    if let Some(node) = node.right.take() {
                        queue.push_back(node);
                    }
                }
            }
            is_left = !is_left;
            res.push(que.into());
        }
    }

    res
}
