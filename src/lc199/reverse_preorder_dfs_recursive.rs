use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    recursive(root, 0, &mut res);
    res
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<i32>) {
    match root {
        None => {}
        Some(root) => {
            let root = root.borrow();
            if res.len() == depth {
                res.push(root.val);
            }
            recursive(root.right.clone(), depth + 1, res);
            recursive(root.left.clone(), depth + 1, res);
        }
    }
}
