use super::remove_element;

#[test]
fn t1() {
    let tests = [
        (vec![3, 2, 2, 3], 3, vec![2, 2]),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]),
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut nums = test.0.clone();
        let k = remove_element(&mut nums, test.1) as usize;
        assert_eq!(k, test.2.len(), "{}", i);
        nums[..k].sort();
        assert_eq!(nums[..k], test.2, "{}", i);
    }
}
