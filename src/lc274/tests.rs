use super::h_index;

#[test]
fn t1() {
    let tests = [(vec![3, 0, 6, 1, 5], 3), (vec![1, 3, 1], 1)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(h_index(test.0.clone()), test.1, "{}", i);
    }
}
