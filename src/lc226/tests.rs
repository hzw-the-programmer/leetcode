use super::invert_tree;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![4, 2, 7, 1, 3, 6, 9], btree![4, 7, 2, 9, 6, 3, 1]),
        (btree![2, 1, 3], btree![2, 3, 1]),
        (btree![], btree![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(invert_tree(test.0.clone()), test.1, "{}", i);
    }

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(invert_tree(test.1.clone()), test.0, "{}", i);
    }
}
