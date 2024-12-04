pub struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let length = moves.len();
        let mut count = vec![0, 0, 0, 0, 0, 0, 0, 0];
        for i in (0..length).rev().step_by(2) {
            let row = &moves[i][0];
            let col = &moves[i][1];

            count[*row as usize] += 1;
            count[(*col + 3) as usize] += 1;

            if row == col {
                count[6] += 1;
            }

            if row + col == 2 {
                count[7] += 1;
            }

            if count[*row as usize] == 3 || count[(*col + 3) as usize] == 3 || count[6] == 3 || count[7] == 3 {
                if length % 2 == 0 {
                    return String::from("B");
                } else {
                    return String::from("A");
                }
            }
        }
        if length < 9 {
            return String::from("Pending");
        } else {
            return String::from("Draw");
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let vec_a = vec![vec![0,0],vec![2,0],vec![1,1],vec![2,1],vec![2,2]];
        assert_eq!("A".to_string(), Solution::tictactoe(vec_a));
    }

    #[test]
    fn test2() {
        let vec_a = vec![vec![0,0],vec![1,1],vec![0,1],vec![0,2],vec![1,0],vec![2,0]];
        assert_eq!("B".to_string(), Solution::tictactoe(vec_a));
    }

    #[test]
    fn test3() {
        let vec_a = vec![vec![0,0],vec![1,1],vec![2,0],vec![1,0],vec![1,2],vec![2,1],vec![0,1],vec![0,2],vec![2,2]];
        assert_eq!("Draw".to_string(), Solution::tictactoe(vec_a));
    }
}