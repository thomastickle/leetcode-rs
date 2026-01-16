use std::collections::HashMap;
use crate::lc_lib::solution::Solution;

/// Leet Code 1: Two Sum

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliments: HashMap<i32, usize> = HashMap::new();
        
        for (idx, i) in nums.iter().enumerate() {
            let compliment = target - *i;
            let compliment_candidate = compliments.get(&compliment);
            match compliment_candidate {
                None => {
                    compliments.insert(*i, idx);
                },
                Some(idx_2) => {
                    return Vec::from([*idx_2 as i32, idx as i32]);
                }
            }
        }
        
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0::lc_1::Solution;

    #[test]
    fn test_find_max_average() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
