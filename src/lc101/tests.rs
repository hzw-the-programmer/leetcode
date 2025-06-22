use super::is_symmetric;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![1, 2, 2, 3, 4, 4, 3], true),
        (btree![1, 2, 2, null, 3, null, 3], false),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(is_symmetric(test.0.clone()), test.1, "{}", i);
    }
}
