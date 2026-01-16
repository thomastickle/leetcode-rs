use crate::lc_lib::solution::Solution;

/// LeetCode 334: Increasing Triplet Subsequence


impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        for num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                second = num;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0::lc_334::Solution;

    #[test]
    fn test_increasing_triplet() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, true);

        let nums = vec![5, 4, 3, 2, 1];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, false);

        let nums = vec![2, 1, 5, 0, 4, 6];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, true);

        let nums = vec![1, 2, 1, 3];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, true);

        let nums = vec![2, 4, -2, -3];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, false);
    }
}
