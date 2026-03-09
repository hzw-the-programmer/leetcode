// 45. Jump Game II

#[cfg(test)]
mod tests;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    let (mut max, mut end) = (0, 0);

    for i in 0..nums.len() - 1 {
        max = max.max(i + nums[i] as usize);
        if i == end {
            end = max;
            jumps += 1;
        }
    }

    jumps
}
