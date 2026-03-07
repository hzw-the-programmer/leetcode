// 189. Rotate Array

#[cfg(test)]
mod tests;

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = k as usize % n;
    let count = gcd(n, k);
    for start in 0..count {
        let mut current = start;
        let mut prev = nums[start];
        loop {
            let next = (current + k) % n;
            let t = nums[next];
            nums[next] = prev;
            prev = t;
            current = next;
            if current == start {
                break;
            }
        }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
