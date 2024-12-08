use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate_hashset(nums: Vec<i32>) -> bool {
        nums.len() != nums.into_iter().collect::<HashSet<i32>>().len()
    }
}