pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut window_check = (&nums).windows(3).rev();
        while let Some(&[a,b,c]) = window_check.next() {
            if a+b>c {return a+b+c;}
        }
        0
    }
}