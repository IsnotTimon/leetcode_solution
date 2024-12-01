pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let vec_s:Vec<&str> = s.trim().split_whitespace().map(|char_from_s|char_from_s).collect();
        let ans = vec_s.last().unwrap().len();
        ans as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, Solution::length_of_last_word("   fly me   to   the moon  ".to_string()));
    }
}