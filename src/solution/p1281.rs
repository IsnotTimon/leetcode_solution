pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut sum = 0;
        let mut multiply = 1 ;
        while n >= 10 {
            sum += n % 10;
            multiply *= n % 10;
            n = n / 10;
        }
        sum += n;
        multiply *= n;
        multiply - sum
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
    }
}