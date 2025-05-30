use crate::utils::binary_tree::*;

pub fn sorted_array_to_bst_2(nums: Vec<i32>) -> Tree {
    recursive(&nums)
}

fn recursive(nums: &[i32]) -> Tree {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    new(
        nums[mid],
        recursive(&nums[..mid]),
        recursive(&nums[mid + 1..]),
    )
}
