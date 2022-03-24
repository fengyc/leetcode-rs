
struct Solution {}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut v = [0; 100];    // max box = 9*5 = 45
        for mut i in low_limit..high_limit + 1 {
            let mut b = 0_usize;
            while i >= 10 {
                b += i as usize % 10;
                i = i / 10;
            }
            b += i as usize;
            v[b] += 1;
        }
        let mut max_count = v[0];
        for n in v.iter() {
            if *n > max_count {
                max_count = *n
            }
        }
        max_count
    }
}

fn main() {
    let _r = Solution::count_balls(5, 15);
    println!("Hello, world!");
}
