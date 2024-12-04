pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x_line = 0;
        let mut y_line = 0;

        for char_move in moves.chars() {
            match char_move {
                'U' => y_line += 1,
                'D' => y_line -= 1,
                'L' => x_line += 1,
                'R' => x_line -= 1,
                _ =>(),
            }
        }
        x_line == 0 && y_line == 0
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let moves = "UD".to_string();
        assert_eq!(true, Solution::judge_circle(moves));
    }

    #[test]
    fn test2() {
        let moves = "LL".to_string();
        assert_eq!(false, Solution::judge_circle(moves));
    }
}