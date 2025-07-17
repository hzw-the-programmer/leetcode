// 295. Find Median from Data Stream

pub struct MedianFinder {
    nums: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self { nums: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        let pos = self.nums.binary_search(&num).unwrap_or_else(|pos| pos);
        self.nums.insert(pos, num);
    }

    fn find_median(&mut self) -> f64 {
        let len = self.nums.len();
        let idx = len >> 1;
        if len & 1 == 1 {
            self.nums[idx] as f64
        } else {
            (self.nums[idx - 1] + self.nums[idx]) as f64 / 2 as f64
        }
    }
}

#[cfg(test)]
mod tests;
