pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut new_nums = Vec::new();
        for i in 0..n as usize {
            new_nums.push(nums[i]);
            new_nums.push(nums[n as usize + i]);
        }
        new_nums
    }

    pub fn shuffle_new(nums: Vec<i32>, n: i32) -> Vec<i32> {
        nums.iter()
            .skip(n as usize)
            .zip(nums.iter())
            .fold(Vec::new(), |mut axx,(&x,&y)|{
                axx.push(y);
                axx.push(x);
                axx
            })
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,4,3,2,1];
        let v_do = vec![1,4,2,3,3,2,4,1];
        assert_eq!(v_do, Solution::shuffle(v, 4))
    }
}