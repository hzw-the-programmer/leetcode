use super::binary_tree_paths;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![1, 2, 3, null, 5], vec!["1->2->5", "1->3"]),
        (btree![1], vec!["1"]),
    ];
    for (i, test) in tests.iter().enumerate() {
        let mut res = binary_tree_paths(test.0.clone());
        res.sort();
        assert_eq!(res, test.1, "{}", i);
    }
}
