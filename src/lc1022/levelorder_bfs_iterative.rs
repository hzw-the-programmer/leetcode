use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    match root {
        None => {}
        Some(node) => {
            let mut queue = VecDeque::new();
            queue.push_back((node, 0));
            while let Some((node, mut n)) = queue.pop_front() {
                let node = node.borrow();
                n = (n << 1) + node.val;
                match (node.left.clone(), node.right.clone()) {
                    (None, None) => res += n,
                    (left, right) => {
                        for node in left.into_iter().chain(right) {
                            queue.push_back((node, n));
                        }
                    }
                }
            }
        }
    }
    res
}
