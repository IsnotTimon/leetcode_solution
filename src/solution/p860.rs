pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut in_hand = (0,0,0);
        for &pay in &bills {
            match pay {
                5 => { in_hand.0 += 1},
                10 => { 
                    if in_hand.0 > 0 {
                        in_hand.0 -= 1;
                        in_hand.1 += 1;
                    } else {
                        return false;
                    }
                },
                20 => {
                    if in_hand.0 > 0 && in_hand.1 > 0 {
                        in_hand.0 -= 1;
                        in_hand.1 -= 1;
                        in_hand.2 += 1;
                    } else if in_hand.0 >= 3 {
                        in_hand.0 -= 3;
                        in_hand.2 += 1;
                    } else {
                        return false;
                    }
                },
                _ => { return false; },
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
        assert_eq!(true, Solution::lemonade_change(vec![5,5,5,10,20]));
        assert_eq!(false, Solution::lemonade_change(vec![5,5,10,10,20]));
    }
}