use crate::utils::binary_tree::*;

pub fn sorted_array_to_bst_2(nums: Vec<i32>) -> Tree {
    recursive(&nums)
    // recursive(&nums, 0, nums.len() - 1)
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

// fn recursive(nums: &[i32], lower: usize, upper: usize) -> Tree {
//     if lower > upper {
//         None
//     } else {
//         let mid = (lower + upper + 1) / 2;
//         new(
//             nums[mid],
//             if mid < 1 {
//                 None
//             } else {
//                 recursive(nums, lower, mid - 1)
//             },
//             recursive(nums, mid + 1, upper),
//         )
//     }
// }
