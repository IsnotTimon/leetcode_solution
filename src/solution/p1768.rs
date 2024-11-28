pub struct Solution;

impl Solution {
    pub fn merge_alternately(mut word1: String, mut word2: String) -> String {
        let mut ans = String::new();
        while !word1.is_empty() && !word2.is_empty() {
            ans.push(word1.chars().next().unwrap());
            word1.remove(0);
            ans.push(word2.chars().next().unwrap());
            word2.remove(0);
        }
        if word1.is_empty() {
            ans.push_str(&word2);
        } else {
            ans.push_str(&word1);
        }
        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("apbqrs".to_string(), Solution::merge_alternately("ab".to_string(), "pqrs".to_string()));
    }
}