use crate::lc_lib::solution::Solution;
use itertools::Itertools;

/// LeetCode 1207: Unique Number of Occurrences

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        arr.iter()
            .counts()
            .iter()
            .map(|(_num, &count)| count)
            .counts()
            .iter()
            .fold(true, |acc, (_num, &count)| acc && count == 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;

    #[test]
    fn test_unique_occurrences() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, true);

        let arr = vec![1, 2];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, false);

        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, true);
    }
}
