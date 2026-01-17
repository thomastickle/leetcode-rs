use crate::lc_lib::solution::Solution;

/// LeetCode 724: Find Pivot Index

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        
        for (i, v) in nums.iter().enumerate() {
            if left_sum == sum - left_sum - v {
                return i as i32;
            }
            left_sum += v
            
        }
        
       -1
    }
}

#[test]
fn test_pivot_index() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let result = Solution::pivot_index(nums);
    assert_eq!(result, 3);

    let nums = vec![1, 2, 3];
    let result = Solution::pivot_index(nums);
    assert_eq!(result, -1);

    let nums = vec![2, 1, -1];
    let result = Solution::pivot_index(nums);
    assert_eq!(result, 0);
}
