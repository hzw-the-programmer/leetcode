// 26. Remove Duplicates from Sorted Array

#[cfg(test)]
mod tests;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut slow = 1;
    for fast in 1..nums.len() {
        if nums[fast] != nums[fast - 1] {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    slow as i32
}
