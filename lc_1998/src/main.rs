struct Solution {}

fn gcd(a: i32, b:i32) -> i32 {
    let (a, b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    if b == 0 {
        return 0;
    }
    let c =  a % b;
    if c == 0 {
        return b;
    }
    return gcd(b, c);
}

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
