use super::*;
use crate::utils::binary_tree as bt;
use crate::utils::singly_linked_list as sll;

#[test]
fn test_1() {
    let head = sll::new_from(&[-10, -3, 0, 5, 9]);
    let wanted = bt::btree![0, -3, 9, -10, null, 5];
    assert_eq!(sorted_list_to_bst(head), wanted);
}

#[test]
fn test_2() {
    let head = sll::new_from(&[]);
    let wanted = bt::btree![];
    assert_eq!(sorted_list_to_bst(head), wanted);
}
