pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut ans = vec!["".to_string(); score.len()];
        let mut map = score.iter().enumerate().collect::<Vec<_>>();
        map.sort_unstable_by(|a, b| b.1.cmp(a.1));
        map.iter().enumerate().for_each(|i| {
            ans[i.1 .0] = match i.0 {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i.0 + 1).to_string(),
            }
        });
        ans
    }
}
