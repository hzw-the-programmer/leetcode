// 55. Jump Game

#[cfg(test)]
mod tests;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut max = 0;

    for i in 0..n {
        if i > max {
            return false;
        }

        max = max.max(i + nums[i] as usize);

        if max >= n - 1 {
            return true;
        }
    }

    true
}
