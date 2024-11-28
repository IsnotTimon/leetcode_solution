pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let len_h = haystack.len();
        let len_n = needle.len();
        if len_n > len_h {
            return -1;
        } else {
            let h:Vec<&u8> = haystack.as_bytes().iter().collect();
            let n:Vec<&u8> = needle.as_bytes().iter().collect();
            for i in 0..len_h - len_n + 1{
                let mut pre_i = i;
                let mut j = 0;
                while h[pre_i] == n[j]{
                    pre_i += 1;
                    j += 1;
                    if j >= len_n {
                        break;
                    }
                }
                if j == len_n {
                    return i as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(-1, Solution::str_str("leetcode".to_string(), "leeto".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(0, Solution::str_str("sadbutsad".to_string(), "sad".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
    }

    #[test]
    fn test4() {
        assert_eq!(0, Solution::str_str("a".to_string(), "a".to_string()));
    }
}