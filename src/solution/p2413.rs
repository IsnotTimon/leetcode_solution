// This is my third solution, it is becoming more and more hard, and there is two methods for this problem.

pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 1  {
            return 2*n;
        } else {
            return n;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}