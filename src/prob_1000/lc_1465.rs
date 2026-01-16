use crate::lc_lib::solution::Solution;

/// LeetCode 1465: Maximum Number of Vowels In a Substring of Given Length


impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let chars = s.chars().collect::<Vec<char>>();
        let mut max_vowels = chars[..k].iter().filter(|x| vowels.contains(x)).count() as i32;
        let mut current_vowels = max_vowels;

        for i in k..chars.len() {
            if vowels.contains(&chars[i]) {
                current_vowels += 1;
            }
            
            if vowels.contains(&chars[i - k]) {
                current_vowels -= 1;
            }
            
            max_vowels = max_vowels.max(current_vowels);
        }
        max_vowels
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_1000::lc_1465::Solution;

    #[test]
    fn max_vowels_test() {
        let string = "abciiidef".to_string();
        let k = 3;
        assert_eq!(Solution::max_vowels(string, k), 3);

        let string = "aeiou".to_string();
        let k = 2;
        assert_eq!(Solution::max_vowels(string, k), 2);

        let string = "leetcode".to_string();
        let k = 3;
        assert_eq!(Solution::max_vowels(string, k), 2);
    }
}
