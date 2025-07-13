use super::swap_pairs;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4], list![2, 1, 4, 3]),
        (list![], list![]),
        (list![1], list![1]),
        (list![1, 2, 3], list![2, 1, 3]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(swap_pairs(test.0.clone()), test.1, "{}", i);
    }
}
