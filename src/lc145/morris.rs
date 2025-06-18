use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n)
// space : O(1)
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];

    let mut current = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: root.clone(),
        right: None,
    })));

    while let Some(node) = current {
        if node.borrow().left.is_some() {
            let left = node.borrow().left.clone().unwrap();
            let mut rightmost = left.clone();
            while rightmost.borrow().right.is_some() {
                let right = rightmost.borrow().right.clone().unwrap();
                if Rc::ptr_eq(&right, &node) {
                    break;
                }
                rightmost = right;
            }

            if rightmost.borrow().right.is_none() {
                rightmost.borrow_mut().right = Some(node);
                current = Some(left);
            } else {
                let start = ans.len();
                ans.push(left.borrow().val);
                let mut rightmost = left.clone();
                while rightmost.borrow().right.is_some() {
                    let right = rightmost.borrow().right.clone().unwrap();
                    if Rc::ptr_eq(&right, &node) {
                        break;
                    }
                    ans.push(right.borrow().val);
                    rightmost = right;
                }
                ans[start..].reverse();

                rightmost.borrow_mut().right = None;
                current = node.borrow().right.clone();
            }
        } else {
            current = node.borrow().right.clone();
        }
    }

    ans
}
