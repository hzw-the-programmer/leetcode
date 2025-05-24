use super::*;

#[test]
fn test_is_valid_bst() {
    let root = new_tree(2, new_tree(1, None, None), new_tree(3, None, None));
    assert_eq!(is_valid_bst(root), true);
}

#[test]
fn test_is_valid_bst_2() {
    let root = new_tree(
        5,
        new_tree(1, None, None),
        new_tree(4, new_tree(3, None, None), new_tree(6, None, None)),
    );
    assert_eq!(is_valid_bst(root), false);
}

#[test]
fn test_is_valid_bst_3() {
    let root = new_tree(
        5,
        new_tree(4, None, None),
        new_tree(6, new_tree(3, None, None), new_tree(7, None, None)),
    );
    assert_eq!(is_valid_bst(root), false);
}
