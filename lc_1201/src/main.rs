struct Solution {}

fn gcd(a: i32, b: i32) -> i32 {
    let (a, b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    let c = a % b;
    if c == 0 {
        return b;
    }
    gcd(b, c)
}

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let mut n = n;
        let mut v = vec![a, b, c];
        v.sort();

        let gcd_01 = gcd(v[0], v[1]);
        let lcd_01 = v[1] / gcd_01 * v[0];

        let gcd_02 = gcd(v[0], v[2]);
        let lcd_02 = v[2] / gcd_02 * v[0];

        let gcd_12 = gcd(v[1], v[2]);
        let lcd_12 = v[2] / gcd_12 * v[1];

        let gcd_3 = gcd(lcd_01, v[2]);
        let lcd_012 = if lcd_01 > v[2] {
            lcd_01 / gcd_3 * v[2]
        } else {
            v[2] / gcd_3 * lcd_01
        };

        let mut m = n * v[0];
        loop {
            let k = m / v[0] + m / v[1] + m / v[2] - m / lcd_01 - m / lcd_02 - m / lcd_12 + m / lcd_012;
            if k == n {
                return k;
            }

            let a = m % v[0];
            let b = m % v[1];
            let c = m % v[2];

            let mut r = vec![a, b, c];
            r.sort();
            if r[1] != 0 && r[1] < v[0] {
                m -= r[1];
                continue;
            }
            if r[2] != 0 && r[2] < v[0] {
                m -= r[2];
                continue;
            }
            m -= v[0]
        }
    }
}

fn main() {
    let _r = Solution::nth_ugly_number(3, 2, 3, 5);
    let _r = Solution::nth_ugly_number(4, 2, 3, 4);
    let _r = Solution::nth_ugly_number(1000000000, 2, 217983653, 336916467);
    println!("Hello, world!");
}


