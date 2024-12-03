pub struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let mut first_num_chars = coordinate1.chars();
        let mut first_tar = 0;
        let mut second_num_chars = coordinate2.chars();
        let mut second_tar = 0;
        if let (Some(first_0),Some(first_1)) = (first_num_chars.next(),first_num_chars.next()) {
            first_tar = first_0 as u8+ first_1 as u8;
        }
        if let (Some(first_0),Some(first_1)) = (second_num_chars.next(),second_num_chars.next()) {
            second_tar = first_0 as u8+ first_1 as u8;
        }
        if first_tar % 2 == second_tar % 2 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::check_two_chessboards("a1".to_string(), "c3".to_string()));
        assert_eq!(false, Solution::check_two_chessboards("a1".to_string(), "h3".to_string()));
    }
}