// I must say something here, ice, you should do this, you can, go on, come on!

pub struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum(2, -1),1);
    }
}