use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    let root = new_tree(
        3,
        new_tree(9, None, None),
        new_tree(20, new_tree(15, None, None), new_tree(7, None, None)),
    );
    let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
    assert_eq!(level_order(root), expected);
}

#[test]
fn test_2() {
    let root = new_tree(1, None, None);
    let expected = vec![vec![1]];
    assert_eq!(level_order(root), expected);
}

#[test]
fn test_3() {
    let root = None;
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(level_order(root), expected);
}
