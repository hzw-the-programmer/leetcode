use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let (p_val, q_val) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
    let mut root = root;
    while let Some(node) = root {
        let val = node.borrow().val;
        if p_val < val && q_val < val {
            root = node.borrow().left.clone();
        } else if p_val > val && q_val > val {
            root = node.borrow().right.clone();
        } else {
            return Some(node);
        }
    }
    None
}
