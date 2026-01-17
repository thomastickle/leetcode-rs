/// LeetCode 2215: Find the Difference of Two Arrays

use crate::lc_lib::solution::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let hash_nums1: HashSet<i32> = HashSet::from_iter(nums1);
        let hash_nums2: HashSet<i32> = HashSet::from_iter(nums2);

        let difference1 = hash_nums1
            .difference(&hash_nums2)
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<i32>>();
        let difference2 = hash_nums2
            .difference(&hash_nums1)
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<i32>>();
        let result = vec![difference1, difference2];
        result
    }
}

#[cfg(test)]
mod tests {
    use hashbag::HashBag;
    use crate::lc_lib::solution::Solution;

    #[test]
    fn test_find_difference() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let results = Solution::find_difference(nums1, nums2);
        assert_eq!(results[0].iter().collect::<HashBag<_>>(), vec![1,3].iter().collect::<HashBag<_>>());
        assert_eq!(results[1].iter().collect::<HashBag<_>>(), vec![4,6].iter().collect::<HashBag<_>>());

        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let results = Solution::find_difference(nums1, nums2);
        assert_eq!(results[0].iter().collect::<HashBag<_>>(), vec![3].iter().collect::<HashBag<_>>());
        assert_eq!(results[1].iter().collect::<HashBag<_>>(), vec![].iter().collect::<HashBag<_>>());
    }
}
