pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_vec:Vec<&u8> = s.as_bytes().iter().collect();
        let mut t_vec:Vec<&u8> = t.as_bytes().iter().collect();
        s_vec.sort();
        t_vec.sort();
        s_vec == t_vec
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_anagram("abc".to_string(), "cba".to_string()));
        assert_eq!(true, Solution::is_anagram("dabc".to_string(), "dcba".to_string()));
    }
}