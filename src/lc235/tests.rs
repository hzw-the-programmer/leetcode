use super::lowest_common_ancestor;
use crate::utils::binary_tree::TreeNode;
use crate::utils::binary_tree::btree;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn t1() {
    let tests = [
        (btree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], 2, 8, 6),
        (btree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], 2, 4, 2),
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
            let b = node.borrow();
            if val < b.val {
                search(b.left.as_ref(), val)
            } else if val > b.val {
                search(b.right.as_ref(), val)
            } else {
                Some(node.clone())
            }
        }
    }
}
