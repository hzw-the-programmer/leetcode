use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    let mut root = new_tree(1, new_tree(3, None, new_tree(2, None, None)), None);
    let expected = new_tree(3, new_tree(1, None, new_tree(2, None, None)), None);
    recover_tree(&mut root);
    assert_eq!(root, expected);
}

#[test]
fn test_2() {
    let mut root = new_tree(
        3,
        new_tree(1, None, None),
        new_tree(4, new_tree(2, None, None), None),
    );
    let expected = new_tree(
        2,
        new_tree(1, None, None),
        new_tree(4, new_tree(3, None, None), None),
    );
    recover_tree(&mut root);
    assert_eq!(root, expected);
}
