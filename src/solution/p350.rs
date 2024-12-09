pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        for n in &nums1 {
            *map.entry(n).or_insert(0) += 1;
        }
        for n in nums2 {
            if let Some(cnt) = map.get_mut(&n) {
                if *cnt > 0 {
                    *cnt -= 1;
                    ans.push(n);
                }
            }
        }
        ans
    }
}
