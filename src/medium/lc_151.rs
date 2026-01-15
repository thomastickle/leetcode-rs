
struct Solution;


impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::lc_151::Solution;

    #[test]
    fn test_reverse_words() {
        let s = "the sky is blue".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "blue is sky the".to_string());

        let s = "  hello world  ".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "world hello".to_string());

        let s = "a good   example".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "example good a".to_string());
    }
}
