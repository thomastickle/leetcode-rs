mod lc_1768;

use itertools::Itertools;


pub fn merge_alternatively(word1: String, word2: String) -> String {
    word1.chars().interleave(word2.chars()).collect()
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_1768::merge_alternatively;

    #[test]
    fn test_merge_alternatively() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let result = merge_alternatively(word1, word2);
        assert_eq!(result, "apbqcr".to_string());

        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let result = merge_alternatively(word1, word2);
        assert_eq!(result, "apbqrs".to_string());

        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let result = merge_alternatively(word1, word2);
        assert_eq!(result, "apbqcd".to_string());
    }
}
