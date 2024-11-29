pub struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut greater = true;
        let mut less = true;
        nums.windows(2).all(|w| {
            if w[0] > w[1] { greater = false }
            if w[0] < w[1] { less = false }
            greater || less
        })
    }
    // unstable
    pub fn is_monotonic_unstable(nums: Vec<i32>) -> bool {
        nums.is_sorted() || nums.is_sorted_by(|a,b| a >= b)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_monotonic(vec![1,2,2,3]));
        assert_eq!(true, Solution::is_monotonic(vec![6,5,4,4]));
        assert_eq!(false, Solution::is_monotonic(vec![1,3,2]));
    }
}