use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    match root {
        None => {}
        Some(node) => {
            let mut stack = vec![(node, 0)];
            while let Some((node, mut n)) = stack.pop() {
                let mut current = Some(node);
                while let Some(node) = current.take() {
                    let node = node.borrow();
                    n = (n << 1) + node.val;
                    match (node.left.clone(), node.right.clone()) {
                        (None, None) => res += n,
                        (left, right) => {
                            current = left;
                            if let Some(node) = right {
                                stack.push((node, n));
                            }
                        }
                    }
                }
            }
        }
    }
    res
}
