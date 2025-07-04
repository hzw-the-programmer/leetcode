// 136. Single Number

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, n| acc ^ n)
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn t1() {
        let tests = [(vec![2, 2, 1], 1), (vec![4, 1, 2, 1, 2], 4), (vec![1], 1)];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(single_number(test.0.clone()), test.1, "{}", i);
        }
    }
}
