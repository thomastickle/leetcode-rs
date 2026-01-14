
struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() < str2.len() {
            return Solution::gcd_of_strings(str2, str1);
        }
        if str2.is_empty() {
            return str1;
        }
        if !str1.starts_with(&str2) {
            return String::new();
        }
        Solution::gcd_of_strings(str1[str2.len()..].to_string(), str2)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_1071::Solution;

    #[test]
    fn test_gcd_of_strings() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "ABC".to_string());

        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "AB".to_string());

        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "".to_string());
    }
}
