use super::add_two_numbers;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![2, 4, 3], list![5, 6, 4], list![7, 0, 8]),
        (list![0], list![0], list![0]),
        (
            list![9, 9, 9, 9, 9, 9, 9],
            list![9, 9, 9, 9],
            list![8, 9, 9, 9, 0, 0, 0, 1],
        ),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            add_two_numbers(test.0.clone(), test.1.clone()),
            test.2,
            "{}",
            i
        );
    }
}
