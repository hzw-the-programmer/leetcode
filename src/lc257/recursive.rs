use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut res = vec![];
    let mut temp = String::new();
    recursive(root.as_deref(), &mut temp, &mut res);
    res
}

fn recursive(root: Option<&RefCell<TreeNode>>, temp: &mut String, res: &mut Vec<String>) {
    if let Some(node) = root {
        let node = node.borrow();
        let len = temp.len();
        temp.push_str(&node.val.to_string());
        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => res.push(temp.clone()),
            (left, right) => {
                for node in left.into_iter().chain(right) {
                    let len = temp.len();
                    temp.push_str("->");
                    recursive(Some(node), temp, res);
                    temp.truncate(len);
                }
            }
        }
        temp.truncate(len);
    }
}
