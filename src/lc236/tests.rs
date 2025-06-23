use super::lowest_common_ancestor;
use crate::utils::binary_tree::TreeNode;
use crate::utils::binary_tree::btree;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn t1() {
    let tests = [
        // (btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 1, 3),
        (btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 4, 5),
        // (btree![1, 2], 1, 2, 1),
    ];

    for (i, test) in tests.iter().enumerate() {
        let root = test.0.clone();
        let (p, q, res) = (
            search(root.as_ref(), test.1),
            search(root.as_ref(), test.2),
            search(root.as_ref(), test.3),
        );
        assert_eq!(p.as_deref().unwrap().borrow().val, test.1);
        assert_eq!(q.as_deref().unwrap().borrow().val, test.2);
        assert_eq!(res.as_deref().unwrap().borrow().val, test.3);

        assert_eq!(lowest_common_ancestor(root, p, q), res, "{}", i);
    }
}

fn search(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            if node.borrow().val == val {
                Some(node.clone())
            } else if let Some(node) = search(node.borrow().left.as_ref(), val) {
                Some(node.clone())
            } else {
                search(node.borrow().right.as_ref(), val)
            }
        }
    }
}
