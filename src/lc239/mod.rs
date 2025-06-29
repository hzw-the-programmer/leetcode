// 239. Sliding Window Maximum

use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut res = Vec::with_capacity(nums.len() - k + 1);
    let mut queue = VecDeque::with_capacity(k);
    for i in 0..nums.len() {
        if let Some(front) = queue.front() {
            if front + k <= i {
                queue.pop_front();
            }
        }

        while let Some(&back) = queue.back() {
            if nums[back] < nums[i] {
                queue.pop_back();
            } else {
                break;
            }
        }
        queue.push_back(i);

        if i + 1 >= k {
            if let Some(&front) = queue.front() {
                res.push(nums[front]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::max_sliding_window;

    #[test]
    fn t1() {
        let tests = [
            (vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7]),
            (vec![1], 1, vec![1]),
            (vec![10, 9, 8, 7, 6], 3, vec![10, 9, 8]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(max_sliding_window(test.0.clone(), test.1), test.2, "{}", i);
        }
    }
}
