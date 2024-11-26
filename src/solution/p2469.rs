// Here is my second solution, no happy, a little confuse, don't know  if is useful for me.

pub struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.80 + 32.00]
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_temperature(36.50),vec![309.65, 97.70]);
    }
}