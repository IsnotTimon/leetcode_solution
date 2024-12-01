pub struct Solution;

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        salary.remove(salary.len() -1);
        salary.remove(0);
        let sum:f64 = salary.iter().map(|&num| num as f64).sum();
        sum / (salary.len() as f64)
    }
}

#[cfg(test)]
mod tests{
    use  super::*;

    #[test]
    fn test1() {
        assert_eq!(2500.0, Solution::average(vec![4000,3000,1000,2000]));
    }
}