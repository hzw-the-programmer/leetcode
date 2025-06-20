use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    recursive(root, 0, &mut res);
    res
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let node = node.borrow();
        if level < res.len() {
            res[level].push(node.val);
        } else {
            res.push(vec![node.val]);
        }
        recursive(node.left.clone(), level + 1, res);
        recursive(node.right.clone(), level + 1, res);
    }
}
