use super::middle_node;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4, 5], list![3, 4, 5]),
        (list![1, 2, 3, 4, 5, 6], list![4, 5, 6]),
        (list![], list![]),
        (list![1], list![1]),
        (list![1, 2], list![2]),
        (list![1, 2, 3], list![2, 3]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(middle_node(test.0.clone()), test.1, "{}", i);
    }
}
