use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    recursive(root.as_deref(), &mut res);
    res
}

fn recursive(root: Option<&RefCell<TreeNode>>, res: &mut Vec<i32>) {
    match root {
        None => {}
        Some(node) => {
            let node = node.borrow();
            recursive(node.left.as_deref(), res);
            res.push(node.val);
            recursive(node.right.as_deref(), res);
        }
    }
}
