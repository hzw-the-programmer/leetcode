use super::zigzag_level_order;
use crate::utils::binary_tree::btree;
use crate::utils::macros::vec_2d;

#[test]
fn t1() {
    let tests = [
        (
            btree![3, 9, 20, null, null, 15, 7],
            vec_2d![[3], [20, 9], [15, 7]],
        ),
        (btree![1], vec_2d![[1]]),
        (btree![], vec_2d![]),
    ];

    for (i, test) in tests.iter().cloned().enumerate() {
        assert_eq!(zigzag_level_order(test.0), test.1, "{} failed", i);
    }
}
