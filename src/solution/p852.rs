pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 2;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if arr[mid] > arr[mid + 1] {
                right = mid;
            } else {
                left = mid;
            }
        }
        right as _
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0,10,5,2]));
    }
}