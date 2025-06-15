use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n^2)
// space: O(n)
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let (mut res, mut temp) = (vec![], vec![]);
    recursive(root.as_deref(), target_sum, &mut temp, &mut res);
    res
}

fn recursive(
    root: Option<&RefCell<TreeNode>>,
    mut target_sum: i32,
    temp: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        temp.push(node.val);
        target_sum -= node.val;
        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => {
                if target_sum == 0 {
                    res.push(temp.clone());
                }
            }
            (left, right) => {
                recursive(left, target_sum, temp, res);
                recursive(right, target_sum, temp, res);
            }
        }
        temp.pop();
    }
}
