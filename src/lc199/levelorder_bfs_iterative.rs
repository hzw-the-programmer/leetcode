use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut queue = VecDeque::from_iter(root);
    while !queue.is_empty() {
        res.push(queue.back().unwrap().borrow().val);
        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                for n in node.left.clone().into_iter().chain(node.right.clone()) {
                    queue.push_back(n);
                }
            }
        }
    }
    res
}
