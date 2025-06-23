use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match (root, p, q) {
        (Some(root), Some(p), Some(q)) => {
            let (root_val, p_val, q_val) = (root.borrow().val, p.borrow().val, q.borrow().val);
            if p_val < root_val && q_val < root_val {
                lowest_common_ancestor(root.borrow().left.clone(), Some(p), Some(q))
            } else if p_val > root_val && q_val > root_val {
                lowest_common_ancestor(root.borrow().right.clone(), Some(p), Some(q))
            } else {
                Some(root)
            }
        }
        _ => None,
    }
}
