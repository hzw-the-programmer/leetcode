use super::majority_element;

#[test]
fn t1() {
    let tests = [(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(majority_element(test.0.clone()), test.1, "{i}");
    }
}
