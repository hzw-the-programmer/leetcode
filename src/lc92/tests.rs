use super::reverse_between;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4, 5], 2, 4, list![1, 4, 3, 2, 5]),
        (list![5], 1, 1, list![5]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            reverse_between(test.0.clone(), test.1, test.2),
            test.3,
            "{}",
            i
        );
    }
}
