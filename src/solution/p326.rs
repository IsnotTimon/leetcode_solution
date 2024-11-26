pub struct Solution;

impl Solution {
    // 递归
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n < 3 || n % 3 != 0 {
            return false;
        }
        Solution::is_power_of_three(n/3)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_power_of_three(27));
        assert_eq!(false, Solution::is_power_of_three(26));
        assert_eq!(false, Solution::is_power_of_three(-1));
        assert_eq!(false, Solution::is_power_of_three(45));
    }
}