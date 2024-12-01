pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut ans = high - low;
        if low % 2 == 0 && high % 2 == 0 {
            ans = ans / 2;
        } else {
            ans = (ans / 2) + 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::count_odds(3, 7));
    }
}