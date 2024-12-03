pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
        .iter() // 遍历每个客户的账户
        .map(|customer| customer.iter().sum()) // 计算每个客户的总财富
        .max() // 找到最大的财富值
        .unwrap_or(0) // 如果为空，返回 0
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_a = vec![vec![1,2,3],vec![3,2,1]];
        assert_eq!(6, Solution::maximum_wealth(vec_a));
    }
}