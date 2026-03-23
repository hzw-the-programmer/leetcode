// 42. Trapping Rain Water

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();

    let mut left = vec![0; n];
    left[0] = height[0];
    for i in 1..n {
        left[i] = left[i - 1].max(height[i]);
    }

    let mut right = vec![0; n];
    right[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right[i] = right[i + 1].max(height[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += left[i].min(right[i]) - height[i];
    }

    ans
}

#[cfg(test)]
mod tests;
