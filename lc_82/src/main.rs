struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::search2(&nums, 0, nums.len() - 1, target);
    }

    pub fn search2(nums: &[i32], a: usize, b: usize, target: i32) -> i32 {
        if a > b {
            return -1;
        }

        if nums[a] <= nums[b] {
            if let Ok(i) = &nums[a..b+1].binary_search(&target) {
                return (a + i) as i32;
            }
            return -1;
        }

        if target < nums[a] && target > nums[b] {
            return -1;
        }

        let r1 = Solution::search2(nums, a, (a + b) / 2, target);
        if r1 != -1 {
            return r1;
        }
        let r2 = Solution::search2(nums, (a + b) / 2 + 1, b, target);
        return r2;
    }
}

fn main() {
    let _i = Solution::search(vec![3, 4, 5, 6, 7, 0, 1, 2], 0);
    println!("Hello, world!");
}
