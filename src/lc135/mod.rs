// 135. Candy

// origin: 1 3 5 2 3 3
// left  : 1 2 3 1 2 1
// right : 1 1 2 1 1 1
// result: 1 2 3 1 2 1

// origin: 1 3 5 3 2 2
// left  : 1 2 3 1 1 1
// right : 1 1 3 2 1 1
// result: 1 2 3 2 1 1

// origin: 1 3 5 3 2 1
// left  : 1 2 3 1 1 1
// right : 1 1 4 3 2 1
// result: 1 2 4 3 2 1
pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut left = vec![0; n];

    for i in 0..n {
        if i > 0 && ratings[i] > ratings[i - 1] {
            left[i] = left[i - 1] + 1;
        } else {
            left[i] = 1;
        }
    }

    let (mut res, mut right) = (0, 0);
    for i in (0..n).rev() {
        if i < n - 1 && ratings[i] > ratings[i + 1] {
            right += 1;
        } else {
            right = 1;
        }

        res += right.max(left[i]);
    }

    res
}

#[cfg(test)]
mod tests;
