// 55. Jump Game

#[cfg(test)]
mod tests;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max = 0 + nums[0] as usize;

    for i in 1..nums.len() {
        if i > max {
            return false;
        }
        max = max.max(i + nums[i] as usize);
    }

    true
}
