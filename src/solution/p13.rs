use std::char;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut pre_char = Solution::get_value(*s.as_bytes().get(0).unwrap());
        s.as_bytes().iter().skip(1).for_each(|char|{
            let cur_char = Solution::get_value(*char);
            // println!("{} {}",pre_char,cur_char);
            if cur_char > pre_char {
                ans -= pre_char;
            } else {
                ans += pre_char;
            }
            pre_char = cur_char;
        });
        ans += pre_char;
        ans
    }

    pub fn get_value(char: u8) -> i32 {
        match char {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(9, Solution::roman_to_int("IX".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}