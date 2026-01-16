use crate::lc_lib::solution::Solution;

/// LC 392: Is Subsequence

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars().into_iter();
        let mut s_char = s_chars.next();
        
        for t_char in t.chars() {
            if s_char == Some(t_char) {
                s_char = s_chars.next();
            }
        }
        
        s_char == None           
    }
}


#[cfg(test)]
mod tests {
    use crate::prob_0::lc_392::Solution;

    #[test]
    fn test_is_subsequence() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), true);

        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), false);
    }
}
