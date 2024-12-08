pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // 左括号数
        let mut l = 0;
        // \*转（数
        let mut cnt = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else if c == ')' {
                if l > 0 {
                    l -= 1;
                } else if cnt > 0 {
                    cnt -= 1;
                } else {
                    return false;
                }
            } else {
                cnt += 1;
                // 左括号优先与星号结合，并补偿一个只与右括号结合的星号
                if l > 0 {
                    l -= 1;
                    cnt += 1;
                }
            }
        }
        l == 0
    }
}