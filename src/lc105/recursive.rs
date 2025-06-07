use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let map: HashMap<_, _> = inorder.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    recursive(&preorder, &map, 0)
}

fn recursive(
    preorder: &[i32],
    map: &HashMap<i32, usize>,
    start: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }

    let root = preorder[0];
    let idx = *map.get(&root).unwrap();
    let len = idx - start;
    Some(Rc::new(RefCell::new(TreeNode {
        val: root,
        left: recursive(&preorder[1..len + 1], map, start),
        right: recursive(&preorder[len + 1..], map, idx + 1),
    })))
}
