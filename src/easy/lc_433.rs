
struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut read_idx = 0;
        let mut write_idx = 0;

        while read_idx < n {
            let current_char = chars[read_idx];
            let mut count = 0;

            // Count occurrences of the current character
            while read_idx < n && chars[read_idx] == current_char {
                read_idx += 1;
                count += 1;
            }

            // Write the character and its count (if greater than 1)
            chars[write_idx] = current_char;
            write_idx += 1;
            if count > 1 {
                for digit in count.to_string().chars() {
                    chars[write_idx] = digit;
                    write_idx += 1;
                }
            }
        }

        write_idx as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_433::Solution;

    #[test]
    fn test_compress() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 6);
        assert_eq!(chars[..6], ['a', '2', 'b', '2', 'c', '3']);

        let mut chars = vec!['a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 1);
        assert_eq!(chars[..1], ['a']);

        let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 4);
    }
}
