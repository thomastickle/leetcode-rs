struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {        
        const VOWELS: &str = "aeiouAEIOU";

        let mut vowels = s.chars().filter(|&c| VOWELS.contains(c)).rev();
        s.chars()
         .map(|c| match VOWELS.contains(c) {
             true => vowels.next().unwrap(),
             false => c,
         })
         .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_345::Solution;

    #[test]
    fn test_reverse_vowels() {
        let s = "IceCreAm".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "AceCreIm".to_string());

        let s = "leetcode".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "leotcede".to_string());
    }
}
