use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    let tree1 = new_tree(1, new_tree(2, None, None), new_tree(3, None, None));
    let tree2 = new_tree(1, new_tree(2, None, None), new_tree(3, None, None));
    assert!(is_same_tree(tree1, tree2));
}

#[test]
fn test_2() {
    let tree1 = new_tree(1, new_tree(2, None, None), None);
    let tree2 = new_tree(1, None, new_tree(2, None, None));
    assert!(!is_same_tree(tree1, tree2));
}

#[test]
fn test_3() {
    let tree1 = new_tree(1, new_tree(2, None, None), new_tree(1, None, None));
    let tree2 = new_tree(1, new_tree(1, None, None), new_tree(2, None, None));
    assert!(!is_same_tree(tree1, tree2));
}
