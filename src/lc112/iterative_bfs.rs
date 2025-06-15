use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let mut stack = Vec::from_iter(root.map(|node| (node, 0)));
    while let Some((node, sum)) = stack.pop() {
        let mut node = node.borrow_mut();
        let sum = node.val + sum;
        if node.left.is_none() && node.right.is_none() && sum == target_sum {
            return true;
        }
        stack.extend(node.left.take().map(|node| (node, sum)));
        stack.extend(node.right.take().map(|node| (node, sum)));
    }
    false
}
