pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let len = colors.len();
        let mut ans = 0;
        for i in 0..len {
            if i != 0 && i != len - 1 {
                if colors[i] != colors[i-1] && colors[i] != colors[i+1] {
                    ans += 1;
                }
            } else {
                if i == 0 {
                    if colors[i] != colors[i+1] && colors[i] != colors[len-1] {
                        ans += 1;
                    }
                } else {
                    if colors[i] != colors[i-1] && colors[i] != colors[0] {
                        ans += 1;
                    }
                }
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
        assert_eq!(3, Solution::number_of_alternating_groups(vec![0,1,0,0,1]));
    }
}