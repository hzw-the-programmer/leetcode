// 260. Single Number III

pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mask = nums.iter().fold(0, |acc, n| acc ^ n);
    let mask = 1 << mask.trailing_zeros();
    nums.iter().fold(vec![0, 0], |mut acc, n| {
        if n & mask == 0 {
            acc[0] ^= n;
        } else {
            acc[1] ^= n;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn t1() {
        let tests = [
            (vec![1, 2, 1, 3, 2, 5], [5, 3]),
            (vec![-1, 0], [0, -1]),
            (vec![0, 1], [0, 1]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(single_number(test.0.clone()), test.1, "{}", i);
        }
    }
}
