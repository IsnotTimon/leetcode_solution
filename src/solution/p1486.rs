pub struct Solution;

impl Solution {
    pub fn xor_operation_complex(n: i32, start: i32) -> i32 {
        (0..n).map(|i| start + i * 2).fold(0, std::ops::BitXor::bitxor)
    }

    pub fn xor_operation_sample(n: i32, start: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n {
            ans = ans ^ (2*i + start);
        }
        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::xor_operation_complex(4, 3), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::xor_operation_sample(10, 5), 2);
    }
}