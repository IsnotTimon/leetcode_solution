pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits[0] == 0 {
            return vec![1];
        }

        let mut carry = 1;

        digits
            .iter_mut()
            .rev()
            .for_each(|num| {
                if carry == 1 {
                    let sum = *num + carry;
                    *num = sum % 10;
                    carry = sum / 10;
                }
                println!("{} {}",num,carry);
            });
        if carry == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![1,2,4], Solution::plus_one(vec![1,2,3]));
        assert_eq!(vec![1,3,0], Solution::plus_one(vec![1,2,9]));
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
    }
}