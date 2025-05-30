use crate::utils::binary_tree::*;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Tree {
    recursive(&nums, 0, nums.len() - 1)
}

fn recursive(nums: &[i32], lower: usize, upper: usize) -> Tree {
    if lower > upper {
        return None;
    }

    let mid = (lower + upper) / 2;
    new(
        nums[mid],
        if mid < 1 {
            None
        } else {
            recursive(nums, lower, mid - 1)
        },
        recursive(nums, mid + 1, upper),
    )
}
