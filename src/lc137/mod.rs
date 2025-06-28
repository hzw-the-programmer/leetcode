// 137. Single Number II

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..32 {
        let total = nums.iter().fold(0, |acc, &n| acc + (n as u32 >> i & 1));
        if total % 3 != 0 {
            res |= 1 << i;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn t1() {
        let tests = [(vec![2, 2, 3, 2], 3), (vec![0, 1, 0, 1, 0, 1, 99], 99)];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(single_number(test.0.clone()), test.1, "{}", i);
        }
    }
}
