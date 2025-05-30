use crate::utils::binary_tree::*;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Tree {
    // recursive(&nums, 0, nums.len() - 1)
    recursive(&nums)
}

// fn recursive(nums: &[i32], lower: usize, upper: usize) -> Tree {
//     if lower > upper {
//         return None;
//     }

//     let mid = (lower + upper) / 2;
//     new(
//         nums[mid],
//         if mid < 1 {
//             None
//         } else {
//             recursive(nums, lower, mid - 1)
//         },
//         recursive(nums, mid + 1, upper),
//     )
// }

fn recursive(nums: &[i32]) -> Tree {
    if nums.is_empty() {
        None
    } else {
        let mid = (nums.len() - 1) / 2;
        new(
            nums[mid],
            recursive(&nums[..mid]),
            recursive(&nums[mid + 1..]),
        )
    }
}
