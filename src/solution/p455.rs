pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut i = 0;
        for x in s {
            if i < g.len() && g[i] <= x {
                i += 1;
            }
        }
        i as _
    }
}
