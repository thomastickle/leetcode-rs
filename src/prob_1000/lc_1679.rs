/// LeetCode 1679: Max Number of K-Sum Pairs
use std::collections::HashMap;
use crate::lc_lib::solution::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut compliment_map = HashMap::new();
        let mut pairs_found = 0;

        for n in nums.into_iter() {
            let compliment = k - n;
            let possible_compliment = compliment_map.get(&compliment);
            match possible_compliment {
                None => {
                    let x = compliment_map.entry(n).or_insert(0);
                    *x += 1;
                }
                Some(count) => {
                    if *count > 0 {
                        pairs_found += 1;
                        let x = compliment_map.entry(compliment).or_insert(0);
                        *x -= 1;
                    } else {
                        let x = compliment_map.entry(n).or_insert(0);
                        *x += 1;
                    }
                }
            }
        }
        pairs_found
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_1000::lc_1679::Solution;

    #[test]
    fn test_max_operations() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, 2);

        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, 1);

        let nums = vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4];
        let k = 2;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, 2);
    }
}
