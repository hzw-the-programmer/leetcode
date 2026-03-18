use super::product_except_self;

#[test]
fn t1() {
    let tests = [
        (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
        (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(product_except_self(test.0.clone()), test.1, "{}", i);
    }
}
