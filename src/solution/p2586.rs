pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        let target = vec![b'a',b'e',b'i',b'o',b'u'];
        for index in left as usize ..= right as usize {
            let first = words[index].as_bytes().first().unwrap();
            let last = words[index].as_bytes().last().unwrap();
            if target.contains(first) && target.contains(last) {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_1 = vec!["hey".to_string(),"aeo".to_string(),"mu".to_string(),"ooo".to_string(),"artro".to_string()];
        assert_eq!(3, Solution::vowel_strings(vec_1, 1, 4));
    }
}