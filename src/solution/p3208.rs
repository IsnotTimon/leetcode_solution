pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let mut al = 1;
        let mut ans = 0;
        for i in 1..(n + k as usize -1) {
            if colors[i % n] != colors[(i-1) % n] {
                al += 1;
            } else {
                al = 1;
            }
            if al >= k {
                ans += 1;
            }
        }
        ans
    }
}