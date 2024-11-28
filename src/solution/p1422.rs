pub struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ans = 0;
        let mut tem = 0;
        let mut left_vec = Vec::new();
        let mut right_vec = Vec::new();
        let mut s_vec = Vec::new();
        s.bytes().into_iter().for_each(|char_s|{
            if char_s == b'0' {
                tem += 1;
                left_vec.push(tem);
            } else {
                left_vec.push(tem);
            }
            s_vec.push(char_s);
        });
        tem = 0;
        s_vec.reverse();
        let len = s_vec.len();
        if len == 2 {
            ans = left_vec[0] + if s_vec[0] == b'1' { 1 } else { 0 };
            return ans;
        }
        for index in 0..len-1 {
            if s_vec[index] == b'1' {
                tem += 1;
                right_vec.push(tem);
            } else {
                right_vec.push(tem);
            }
            if right_vec[index] + left_vec[len - 2 - index] >= ans {
                ans = right_vec[index] + left_vec[len - 2 - index];
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
        assert_eq!(5, Solution::max_score("011101".to_string()));
        assert_eq!(3, Solution::max_score("1111".to_string()));
        assert_eq!(1, Solution::max_score("00".to_string()));
        assert_eq!(2, Solution::max_score("0100".to_string()));
    }
}