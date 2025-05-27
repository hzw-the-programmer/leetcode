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

#[test]
fn test_4() {
    let root = new_tree(1, None, new_tree(2, None, None));
    let expected: Vec<Vec<i32>> = vec![vec![1], vec![2]];
    assert_eq!(level_order(root), expected);
}

#[test]
fn test_5() {
    let root = new_tree(
        1,
        new_tree(2, new_tree(4, None, None), new_tree(5, None, None)),
        new_tree(3, None, None),
    );
    let expected: Vec<Vec<i32>> = vec![vec![1], vec![2, 3], vec![4, 5]];
    assert_eq!(level_order(root), expected);
}

#[test]
fn test_6() {
    let root = new_tree(
        1,
        new_tree(2, new_tree(4, None, None), None),
        new_tree(3, None, new_tree(5, None, None)),
    );
    let expected: Vec<Vec<i32>> = vec![vec![1], vec![2, 3], vec![4, 5]];
    assert_eq!(level_order(root), expected);
}
