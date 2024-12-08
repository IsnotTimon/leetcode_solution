use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let nums_set = nums.into_iter().collect::<HashSet<i32>>();
        for i in 0..=len {
            if !nums_set.contains(&(i as i32)) {
                return i as i32;
            }
        }
        0
    }
}