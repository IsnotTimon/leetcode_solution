pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & n - 1 == 0
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_power_of_two(16));
    }

    #[test]
    fn test2() {
        assert_eq!(false, Solution::is_power_of_two(15));
    }
}