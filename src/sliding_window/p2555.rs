// 2555. Maximize Win From Two Segments

use crate::Solution;

impl Solution {
    /*
       prizePositions = [1,1,2,2,3,3,5], k = 2
       0 1(0,1) 2(2,3) 3(4,5) 4 5(6)
       重叠解  ：[1, 3]，[3, 5]
       不重叠解：[0, 2], [3, 5]
       题目没要求一定要重叠解，所以只需求其等价的不重叠解即可

       假设遍历到奖品4，其位置在3上，
       以其为左端点的长度为2的线段的右端点在奖品6上，位置5，覆盖总奖品个数：6-4+1=3,
       这是线段2覆盖奖品的数量，还要加上线段1覆盖的奖品数量。
       线段1覆盖的奖品数量是，以奖品4之前的奖品(3,2,1,0)为右端点得到的线段的覆盖奖品数的最大值
    */
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let n = prize_positions.len();
        if 2 * k + 1 >= prize_positions[n - 1] - prize_positions[0] {
            return n as _;
        }
        let mut left = 0;
        let mut right = 0;
        let mut mx = 0;
        let mut ans = 0;
        for (mid, &pos) in prize_positions.iter().enumerate() {
            while right < n && prize_positions[right] - pos <= k {
                right += 1;
            }
            let covered = right - mid;
            // println!("({mid},{right}): {}", covered);
            ans = ans.max(covered + mx);

            while pos - prize_positions[left] > k {
                left += 1;
            }
            mx = mx.max(mid - left + 1);
        }
        ans as _
    }

    pub fn maximize_win_v1(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut ans = 0;
        let mut dp = vec![0; prize_positions.len() + 1];
        for (right, &pos) in prize_positions.iter().enumerate() {
            while pos - prize_positions[left] > k {
                // println!("({right},{left}): {pos} - {} > {k}", prize_positions[left]);
                left += 1;
            }
            let covered = right - left + 1;
            // println!("({right},{left}): {}", covered);
            ans = ans.max(covered + dp[left]);
            dp[right + 1] = dp[right].max(covered);
        }
        ans as _
    }

    pub fn maximize_win_v2(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; prize_positions.len() + 1];
        let mut ans = 0;
        for (right, &p) in prize_positions.iter().enumerate() {
            let left = prize_positions.partition_point(|&x| x < (p - k));
            let covered = right - left + 1;
            ans = ans.max(covered + dp[left]);
            dp[right + 1] = dp[right].max(covered);
            println!(
                "({right}, {left}), covered={covered}, dp[{left}]={}, dp[{right}]={}, dp[{}]={}",
                dp[left],
                dp[right],
                right + 1,
                dp[right + 1]
            );
        }
        ans as _
    }

    pub fn maximize_win_one_line(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut ans = 0;
        for (right, &pos) in prize_positions.iter().enumerate() {
            while pos - prize_positions[left] > k {
                // println!("({right},{left}): {pos} - {} > {k}", prize_positions[left]);
                left += 1;
            }
            // println!("({right},{left}): {}", right - left + 1);
            ans = ans.max(right - left + 1);
        }
        ans as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximize_win() {
        //   0 1 2   3 4 5 6
        // 0 1 2 3 4 5 6 7 8
        assert_eq!(6, Solution::maximize_win(vec![1, 2, 3, 5, 6, 7, 8], 2));

        //   0 1 2     3   4
        // 0 1 2 3 4 5 6 7 8
        assert_eq!(5, Solution::maximize_win(vec![1, 2, 3, 6, 8], 2));

        // 0 1(0,1) 2(2,3) 3(4,5) 4 5(6)
        assert_eq!(7, Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2));

        // 0 1(0) 2(1) 3(2) 4 5 6(3,4,5) 7 8(6)
        assert_eq!(7, Solution::maximize_win(vec![1, 2, 3, 6, 6, 6, 8], 2));

        //   0 1 3 4
        // 0 1 2 3 4
        assert_eq!(2, Solution::maximize_win(vec![1, 2, 3, 4], 0));

        assert_eq!(4, Solution::maximize_win(vec![1, 2, 3, 4], 1));
    }

    #[test]
    fn test_maximize_win_one_line() {
        //   0 1 2   3 4 5 6
        // 0 1 2 3 4 5 6 7 8
        assert_eq!(
            3,
            Solution::maximize_win_one_line(vec![1, 2, 3, 5, 6, 7, 8], 2)
        );

        //   0 1 2     3   4
        // 0 1 2 3 4 5 6 7 8
        assert_eq!(3, Solution::maximize_win_one_line(vec![1, 2, 3, 6, 8], 2));

        // 0 1(0,1) 2(2,3) 3(4,5) 4 5(6)
        assert_eq!(
            6,
            Solution::maximize_win_one_line(vec![1, 1, 2, 2, 3, 3, 5], 2)
        );

        // 0 1(0) 2(1) 3(2) 4 5 6(3,4,5) 7 8(6)
        assert_eq!(
            4,
            Solution::maximize_win_one_line(vec![1, 2, 3, 6, 6, 6, 8], 2)
        );
    }
}
