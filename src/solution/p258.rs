pub struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut count = 0;
        while num > 0 {
            count += num % 10;
            num /= 10;
        }
        Self::add_digits(count)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::add_digits(38));
    }

}