pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut product = 1;
        nums.iter().for_each(|num|{
            if num > &0 {
                product *= 1;
            } else if num < &0 {
                product *= -1;
            } else {
                product *= num;
            }
        });
        if product == 0 {
            return 0;
        } else if product > 0 {
            return 1;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::array_sign(vec![-1,-2,-3,-4,3,2,1]));
        assert_eq!(0, Solution::array_sign(vec![1,5,0,2,-3]));
        assert_eq!(-1, Solution::array_sign(vec![-1,1,-1,1,-1]));
        assert_eq!(-1, Solution::array_sign(vec![41,65,14,80,20,10,55,58,24,56,28,86,96,10,3,84,4,41,13,32,42,43,83,78,82,70,15,-41]));
    }
}