use super::level_order;
use crate::utils::binary_tree::btree;
use crate::utils::macros::vec_2d;

#[test]
fn t1() {
    let tests = [
        (btree![3, 9, 20, 15, 7], vec_2d![[3], [9, 20], [15, 7]]),
        (btree![1], vec_2d![[1]]),
        (btree![], vec_2d![]),
        (btree![1, null, 2], vec_2d![[1], [2]]),
        (btree![1, 2, 3, 4, 5], vec_2d![[1], [2, 3], [4, 5]]),
        (
            btree![1, 2, 3, 4, null, null, 5],
            vec_2d![[1], [2, 3], [4, 5]],
        ),
        (
            btree![1, 2, 3, 4, 5, 6, 7],
            vec_2d![[1], [2, 3], [4, 5, 6, 7]],
        ),
    ];

    for (i, test) in tests.iter().cloned().enumerate() {
        assert_eq!(level_order(test.0), test.1, "{} failed", i);
    }
}
