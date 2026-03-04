// 88. Merge Sorted Array

/// 合并两个有序数组（原地合并，最优解）
/// - nums1: 第一个有序数组，长度为 m+n，前 m 个为有效元素
/// - m: nums1 有效元素个数
/// - nums2: 第二个有序数组，长度为 n
/// - n: nums2 有效元素个数
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
    // 转换为 usize 避免索引越界（Rust 索引必须是非负整数）
    let mut p1 = m as usize;
    let mut p2 = n as usize;
    let mut p = (m + n) as usize;

    // 从后往前合并，直到其中一个数组遍历完
    while p1 > 0 && p2 > 0 {
        if nums1[p1 - 1] > nums2[p2 - 1] {
            nums1[p - 1] = nums1[p1 - 1];
            p1 -= 1;
        } else {
            nums1[p - 1] = nums2[p2 - 1];
            p2 -= 1;
        }
        p -= 1;
    }

    // 若 nums2 还有剩余元素，拷贝到 nums1 开头（nums1 剩余位置已都是 0）
    nums1[0..p2].copy_from_slice(&nums2[0..p2]);
}

#[cfg(test)]
mod tests;
