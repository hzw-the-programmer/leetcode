use super::remove_duplicates;

#[test]
fn t1() {
    let tests = [
        (vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3]),
        (vec![0, 0, 1, 1, 1, 1, 2, 3, 3], vec![0, 0, 1, 1, 2, 3, 3]),
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut nums = test.0.clone();
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..k], test.1, "{}", i);
    }
}
