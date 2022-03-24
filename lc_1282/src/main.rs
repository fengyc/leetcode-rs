struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut r = vec![];
        if group_sizes.is_empty() {
            return r;
        }

        let mut s = Vec::with_capacity(group_sizes.len());
        for i in 0..group_sizes.len() {
            s.push([group_sizes[i], i as i32]);
        }
        s.sort();

        let mut size = s[0][0];
        let mut v = Vec::with_capacity(size as usize);
        let mut i = 0;
        while size > 0 && i < s.len() {
            v.push(s[i][1]);
            size -= 1;
            i += 1;
            if size == 0 {
                r.push(v);
                if i >= s.len() {
                    break;
                }
                size = s[i][0];
                v = Vec::with_capacity(size as usize);
            }
        }

        r
    }
}

fn main() {
    let _r = Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]);
    println!("Hello, world!");
}
