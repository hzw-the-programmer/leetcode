use super::merge_two_lists;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 4], list![1, 3, 4], list![1, 1, 2, 3, 4, 4]),
        (list![], list![], list![]),
        (list![], list![0], list![0]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            merge_two_lists(test.0.clone(), test.1.clone()),
            test.2,
            "{}",
            i
        );
    }
}
