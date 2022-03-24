//! Given an unsorted integer array nums, return the smallest missing positive integer.
//! You must implement an algorithm that runs in O(n) time and uses constant extra space.
//! 1 <= nums.length <= 5 * 10^5
//! -2^31 <= nums[i] <= 2^31 - 1

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut bits = [0_u8; 500000 / 8 + 1];
        for n in nums {
            if n >= 0 && n <= 500000 {
                let i = n / 8;
                let j = n % 8;
                bits[i as usize] |= (1 << j) as u8;
            }
        }
        for i in 0..bits.len() {
            for j in 0..8 {
                if bits[i] & 1 << j == 0 {
                    let n = (i * 8 + j) as i32;
                    if n > 0 {
                        return n;
                    }
                }
            }
        }
        0
    }
}

fn main() {
    let _r = Solution::first_missing_positive(vec![3, 4, -1, 1]);
    let _r = Solution::first_missing_positive(vec![7, 8, 9, 11, 12]);
    println!("Hello, world!");
}
