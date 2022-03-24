struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev = -1;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                prev = i as i32;
                break;
            }
        }
        if prev >= 0 {
            for i in (prev + 1) ..nums.len() as i32 {
                if nums[i as usize] == 1 {
                    if i - prev - 1 < k {
                        return false;
                    }
                    prev = i;
                }
            }
        }
        true
    }
}

fn main() {
    let _r = Solution::k_length_apart(vec![1,0,0,1,0,1], 2);
    let _r = Solution::k_length_apart(vec![1,1,1,1,1], 0);
    let _r = Solution::k_length_apart(vec![0,1,0,1], 1);
    println!("Hello, world!");
}
