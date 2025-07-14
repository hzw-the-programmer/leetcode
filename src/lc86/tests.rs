use super::partition;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 4, 3, 2, 5, 2], 3, list![1, 2, 2, 4, 3, 5]),
        (list![2, 1], 2, list![1, 2]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(partition(test.0.clone(), test.1), test.2, "{}", i);
    }
}
