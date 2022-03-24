use std::cmp::Ordering;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::default();
    let mut sums = [[0; 200]; 200];
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            sums[i][j] = nums[i] + nums[j];
        }
    }

    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            for m in j + 1..nums.len() - 1 {
                for n in m + 1..nums.len() {
                    if sums[i][j] + sums[m][n] == target {
                        let mut v = vec![nums[i], nums[j], nums[m], nums[n]];
                        v.sort();

                        let mut dup = false;
                        for r in &result {
                            if v.cmp(r) == Ordering::Less {
                                break;
                            }
                            if v.cmp(r) == Ordering::Equal {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            result.push(v);
                            result.sort();
                        }
                    }
                }
            }
        }
    }

    result
}

fn main() {
    // let _r = four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    // let _r = four_sum(vec![2, 2, 2, 2, 2], 8);
    let _r = four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0);
    println!("Hello, world");
}
