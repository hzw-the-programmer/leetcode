use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    let tree = new_tree(
        1,
        new_tree(2, new_tree(3, None, None), new_tree(4, None, None)),
        new_tree(2, new_tree(3, None, None), new_tree(4, None, None)),
    );
    assert!(is_symmetric(tree));
}

#[test]
fn test_2() {
    let tree = new_tree(
        1,
        new_tree(2, None, new_tree(3, None, None)),
        new_tree(2, None, new_tree(3, None, None)),
    );
    assert!(is_symmetric(tree));
}
