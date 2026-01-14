struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut skip = false;

        // Insert a 0 into the beginning and end to handle edge cases. 
        // Now we can match on windows of 3 consecutive elements.
        for i in [&[0], &flowerbed[..], &[0]].concat().windows(3) {
            if skip {
                skip = false;
                continue
            }
            
            if i == [0, 0, 0] {
                count += 1;
                skip = true;
            }
        }

        count >= n
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_605::Solution;

    #[test]
    fn test_can_place_flowers() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, true);

        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, false);

        let flowerbed = vec![0, 0, 1, 0, 0];
        let n = 1;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, true);

        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, false);
    }
}
