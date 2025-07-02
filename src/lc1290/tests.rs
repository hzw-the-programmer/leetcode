use super::get_decimal_value;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [(list![1, 0, 1], 5), (list![0], 0)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(get_decimal_value(test.0.clone()), test.1, "{}", i);
    }
}
