pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let differ = arr[1] - arr[0];
        
        for window in arr.windows(2) {
            if window[1]-window[0] != differ {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::can_make_arithmetic_progression(vec![3,5,1]));
        assert_eq!(false, Solution::can_make_arithmetic_progression(vec![1,2,4]));
    }
}