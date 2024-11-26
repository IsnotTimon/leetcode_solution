pub struct Solution;

impl Solution {
    pub fn num_identical_pairs_sample(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] == nums[j] {
                    ans += 1;
                }
            }
        }
        ans
    }

    pub fn num_identical_pairs(_nums: Vec<i32>) -> i32 {
        let mut _ans = 0;
        // todo
        _ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_1 = vec![1,2,3,1,1,3];
        assert_eq!(4, Solution::num_identical_pairs(vec_1));
    }

    #[test]
    fn test2() {
        let vec_1 = vec![1,1,1,1];
        assert_eq!(6, Solution::num_identical_pairs(vec_1));
    }
}