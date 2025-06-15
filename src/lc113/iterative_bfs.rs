use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

// time : O(n^2)
// space: O(n)
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut map: HashMap<*const RefCell<TreeNode>, *const RefCell<TreeNode>> = HashMap::new();
    let mut stack = Vec::from_iter(root.clone().map(|node| (node, 0)));
    while let Some((node, sum)) = stack.pop() {
        let sum = sum + node.borrow().val;
        match (node.borrow().left.clone(), node.borrow().right.clone()) {
            (None, None) => {
                if sum == target_sum {
                    let mut path = VecDeque::new();
                    path.push_front(node.borrow().val);
                    let mut child = Rc::as_ptr(&node);
                    while let Some(&parent) = map.get(&child) {
                        unsafe {
                            path.push_front((*parent).borrow().val);
                        }
                        child = parent;
                    }
                    res.push(path.into());
                }
            }
            (left, right) => {
                if let Some(left) = left {
                    map.insert(Rc::as_ptr(&left), Rc::as_ptr(&node));
                    stack.push((left, sum))
                }
                if let Some(right) = right {
                    map.insert(Rc::as_ptr(&right), Rc::as_ptr(&node));
                    stack.push((right, sum))
                }
            }
        }
    }
    res
}
