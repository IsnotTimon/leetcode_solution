pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            for j in 0..(nums.len() - i - 1) {
                if nums[j] == 0 {
                    nums.swap(j, j+1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let mut test_vec = vec![0,1,0,3,12];
        Solution::move_zeroes(& mut test_vec);
        assert_eq!(vec![1,3,12,0,0], test_vec);
    }
}