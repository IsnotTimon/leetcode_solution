pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let new_s = s.into_bytes().iter()
        .map(|& char|{
            if char >= 65 && char <= 90 {
                char + 32
            } else {
                char
            }
        })
        .collect();
        
        let new_s = String::from_utf8(new_s).unwrap();
        
        new_s
    }

    pub fn to_lower_case_new(s: String) -> String {
        s.to_lowercase()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("hello".to_string(), Solution::to_lower_case("Hello".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!("hello".to_string(), Solution::to_lower_case_new("Hello".to_string()));
    }
}