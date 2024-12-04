pub struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for ope in operations {
            match ope.as_str() {
                "C" => { stack.pop(); }
                "D" => { stack.push(stack[stack.len()-1] * 2); },
                "+" => { stack.push(stack[stack.len()-1] + stack[stack.len() - 2]); },
                _ => { stack.push(ope.parse::<i32>().unwrap()); }
            }
        }
        stack.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_a = vec!["5".to_string(),"2".to_string(),"C".to_string(),"D".to_string(),"+".to_string()];
        assert_eq!(30, Solution::cal_points(vec_a));
    }

    #[test]
    fn test2() {
        let vec_a = vec!["5".to_string(),"-2".to_string(),"4".to_string(),"C".to_string(),"D".to_string(),"9".to_string(),"+".to_string(),"+".to_string()];
        assert_eq!(27, Solution::cal_points(vec_a));
    }

    #[test]
    fn test3() {
        let vec_a = vec!["1".to_string()];
        assert_eq!(27, Solution::cal_points(vec_a));
    }
}