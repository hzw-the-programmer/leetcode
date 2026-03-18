// 238. Product of Array Except Self

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![0; nums.len()];

    answer[0] = 1;
    for i in 1..nums.len() {
        answer[i] = nums[i - 1] * answer[i - 1];
    }

    let mut r = 1;
    for i in (0..nums.len()).rev() {
        answer[i] *= r;
        r *= nums[i];
    }

    answer
}

#[cfg(test)]
mod tests;
