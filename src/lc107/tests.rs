use super::*;
use crate::utils::binary_tree::btree;
use crate::utils::macros::vec_2d;

#[test]
fn t1() {
    let tests = [
        (
            btree![3, 9, 20, null, null, 15, 7],
            vec_2d![[15, 7], [9, 20], [3]],
        ),
        (btree![1], vec_2d![[1]]),
        (btree![], vec_2d![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(level_order_bottom(test.0.clone()), test.1, "{}", i);
    }
}
