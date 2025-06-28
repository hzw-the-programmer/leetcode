// 137. Single Number II

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold(vec![0; 32], |mut acc, &n| {
            for (i, e) in acc.iter_mut().enumerate() {
                *e += (n as u32 & (1 << i)) >> i;
            }
            acc
        })
        .iter()
        .enumerate()
        .fold(0, |mut acc, (i, n)| {
            acc |= ((n % 3) << i) as i32;
            acc
        })
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
