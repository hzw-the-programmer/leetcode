use super::sorted_list_to_bst;
use crate::utils::binary_tree::btree;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![-10, -3, 0, 5, 9], btree![0, -3, 9, -10, null, 5]),
        (list![], btree![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(sorted_list_to_bst(test.0.clone()), test.1, "{}", i);
    }
}
