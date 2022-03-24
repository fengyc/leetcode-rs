struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = (nums.len() - 1) as i32;
        loop {
            if a > b {
                if b < 0 {
                    return 0;
                }
                if a >= nums.len() as i32 {
                    return nums.len() as i32;
                }
                return a;
            }

            let m = (a + b) / 2;
            let mid = nums[m as usize];
            if mid == target {
                return m;
            }
            if mid > target {
                b = m - 1;
            }
            if mid < target {
                a = m + 1;
            }
        }
    }
}

fn main() {
    let _r = Solution::search_insert(vec![1, 2, 3, 5, 6, 7], 4);
    let _r = Solution::search_insert(vec![1, 2, 3, 5, 6, 7], 0);
    let _r = Solution::search_insert(vec![1, 2, 3, 5, 6, 7], 8);
    println!("Hello, world!");
}
