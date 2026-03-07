// 169. Majority Element

#[cfg(test)]
mod tests;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = nums[0];
    let mut count = 0;
    for num in nums {
        if count == 0 {
            candidate = num;
        }
        count += if candidate == num { 1 } else { -1 };
    }
    candidate as i32
}

// [2,2,1,1,1,2,2]
// candidate = 2, count = 1
// candidate = 2, count = 2
// candidate = 2, count = 1
// candidate = 2, count = 0
// candidate = 1, count = 1
// candidate = 1, count = 0
// candidate = 2, count = 1
