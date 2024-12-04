pub struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 2 {
            return true;
        } 
        let x1 = coordinates[1][0] - coordinates[0][0];
        let y1 = coordinates[1][1] - coordinates[0][1];

        for i in 2..coordinates.len() { 
            let x2 = coordinates[i][0] - coordinates[0][0];
            let y2 = coordinates[i][1] - coordinates[0][1];
            if x1 * y2 != x2 * y1 {
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
        let vec_a = vec![vec![1,2],vec![2,3],vec![4,5],vec![6,7],vec![7,8]];
        assert_eq!(true, Solution::check_straight_line(vec_a));
    }

    #[test]
    fn test2() {
        let vec_a = vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]];
        assert_eq!(false, Solution::check_straight_line(vec_a));
    }
}