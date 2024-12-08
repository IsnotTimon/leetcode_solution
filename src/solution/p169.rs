pub struct Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len()/2]
    }
}