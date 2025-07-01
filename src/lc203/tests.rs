use super::remove_elements;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 6, 3, 4, 5, 6], 6, list![1, 2, 3, 4, 5]),
        (list![], 1, list![]),
        (list![7, 7, 7, 7], 7, list![]),
        (list![1, 2, 2, 1], 2, list![1, 1]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(remove_elements(test.0.clone(), test.1), test.2, "{}", i);
    }
}
