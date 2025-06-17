use super::binary_tree_paths;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    assert_eq!(
        binary_tree_paths(btree![1, 2, 3, null, 5]),
        vec!["1->2->5", "1->3"]
    );
    assert_eq!(binary_tree_paths(btree![1]), vec!["1"]);
}
