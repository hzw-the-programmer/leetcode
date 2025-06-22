use super::kth_smallest;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![3, 1, 4, null, 2], 1, 1),
        (btree![5, 3, 6, 2, 4, null, null, 1], 3, 3),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(kth_smallest(test.0.clone(), test.1), test.2, "{}", i);
    }
}
