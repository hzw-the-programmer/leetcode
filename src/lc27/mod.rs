// 27. Remove Element

/// 移除数组中指定值的元素（原地修改）
/// - nums: 可变整数数组
/// - val: 需要移除的目标值
/// - 返回：移除后数组的新长度
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 慢指针：记录新数组的填充位置
    let mut slow = 0;
    // 快指针：遍历整个数组
    for fast in 0..nums.len() {
        // 若当前元素不是目标值，保留到慢指针位置
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    // 慢指针的最终位置就是新数组长度
    slow as i32
}

// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     let mut p = nums.len();
//     let mut i = 0;
//     while i < p {
//         if nums[i] == val {
//             nums.swap(i, p - 1);
//             p -= 1;
//         } else {
//             i += 1;
//         }
//     }
//     p as i32
// }

#[cfg(test)]
mod tests;
