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

#[test]
fn test_3() {
    let mut root = new_tree(
        10,
        new_tree(
            5,
            new_tree(0, new_tree(2, None, None), new_tree(-5, None, None)),
            new_tree(8, new_tree(6, None, None), new_tree(9, None, None)),
        ),
        new_tree(
            15,
            new_tree(13, new_tree(12, None, None), new_tree(14, None, None)),
            new_tree(20, new_tree(18, None, None), new_tree(25, None, None)),
        ),
    );
    let expected = new_tree(
        10,
        new_tree(
            5,
            new_tree(0, new_tree(-5, None, None), new_tree(2, None, None)),
            new_tree(8, new_tree(6, None, None), new_tree(9, None, None)),
        ),
        new_tree(
            15,
            new_tree(13, new_tree(12, None, None), new_tree(14, None, None)),
            new_tree(20, new_tree(18, None, None), new_tree(25, None, None)),
        ),
    );
    recover_tree(&mut root);
    assert_eq!(root, expected);
}
