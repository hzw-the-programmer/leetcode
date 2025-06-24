use super::replace_value_in_tree;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (
            btree![5, 4, 9, 1, 10, null, 7],
            btree![0, 0, 0, 7, 7, null, 11],
        ),
        (btree![3, 1, 2], btree![0, 0, 0]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(replace_value_in_tree(test.0.clone()), test.1, "{}", i);
    }
}
