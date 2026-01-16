use crate::lc_lib::solution::Solution;

/// LC 11: Container With Most Water

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let w = (right - left) as i32;
            let h = height[left].min(height[right]);
            let area = w * h;
            if area > max_area {
                max_area = area;
            }
            
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0::lc_11::Solution;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);

        let height = vec![1, 1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}
