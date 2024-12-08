use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set_1 = nums1.into_iter().collect::<HashSet<i32>>();
        let set_2 = nums2.into_iter().collect::<HashSet<i32>>();
        let ans = set_1.intersection(&set_2).cloned().collect::<Vec<i32>>();
        ans
    }
}