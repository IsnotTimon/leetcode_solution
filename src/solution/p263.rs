pub struct Solution;

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n<1 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_ugly(1));
        assert_eq!(true, Solution::is_ugly(6));
        assert_eq!(false, Solution::is_ugly(14));
    }
}
