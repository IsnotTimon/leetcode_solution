pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for char_s in s.chars() {
            match char_s {
                '(' | '[' | '{' => stack.push(char_s),
                ')' => if stack.is_empty() || stack.pop().unwrap() != '(' { return false },
                ']' => if stack.is_empty() || stack.pop().unwrap() != '[' { return false },
                '}' => if stack.is_empty() || stack.pop().unwrap() != '{' { return false },
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
        assert_eq!(true, Solution::is_valid("([])".to_string()));
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
        assert_eq!(false, Solution::is_valid("]".to_string()));
    }
}