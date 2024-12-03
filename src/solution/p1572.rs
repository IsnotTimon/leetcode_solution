pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans =0;
        let len = mat.len();
        for i in 0..len {
            ans += mat[i][i];
            ans += mat[i][len - i -1];
        }
        if len % 2 == 1 {
            ans -= mat[len/2][len/2];
        }
        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_a = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        assert_eq!(25, Solution::diagonal_sum(vec_a));
    }
}