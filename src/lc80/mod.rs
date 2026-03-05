// 80. Remove Duplicates from Sorted Array II

#[cfg(test)]
mod tests;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len < 3 {
        return len as i32;
    }

    let mut slow = 2;
    for fast in 2..len {
        if nums[fast] != nums[slow - 2] {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    slow as i32
}

// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let len = nums.len();
//     if len < 3 {
//         return len as i32;
//     }

//     let mut count = 0;
//     let mut slow = 1;
//     for fast in 1..len {
//         if nums[fast] == nums[fast - 1] {
//             count += 1;
//             if count == 1 {
//                 nums[slow] = nums[fast];
//                 slow += 1;
//             }
//         } else {
//             count = 0;
//             nums[slow] = nums[fast];
//             slow += 1;
//         }
//     }

//     slow as i32
// }
