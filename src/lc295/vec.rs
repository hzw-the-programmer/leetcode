pub struct MedianFinder {
    nums: Vec<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }

    pub fn add_num(&mut self, num: i32) {
        let pos = self.nums.binary_search(&num).unwrap_or_else(|pos| pos);
        self.nums.insert(pos, num);
    }

    pub fn find_median(&mut self) -> f64 {
        let len = self.nums.len();
        let idx = len >> 1;
        if len & 1 == 1 {
            self.nums[idx] as f64
        } else {
            (self.nums[idx - 1] + self.nums[idx]) as f64 / 2 as f64
        }
    }
}
