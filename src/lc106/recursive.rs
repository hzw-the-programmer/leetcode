use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut map = HashMap::new();
    for (i, &n) in inorder.iter().enumerate() {
        map.insert(n, i);
    }
    recursive(&postorder, &map, 0)
}

fn recursive(
    postorder: &[i32],
    map: &HashMap<i32, usize>,
    start: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    match postorder.len() {
        0 => None,
        n => {
            let root = postorder[n - 1];
            let idx = *map.get(&root).unwrap();
            let len = idx - start;
            Some(Rc::new(RefCell::new(TreeNode {
                val: root,
                left: recursive(&postorder[..len], map, start),
                right: recursive(&postorder[len..n - 1], map, idx + 1),
            })))
        }
    }
}
