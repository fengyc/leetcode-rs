fn main() {
    let v = vec![1, 2, 2, 3, 4, 5, 5, 5, 5];
    let r = search_range(v, 5);
    println!("{:?}", r);
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn search_inner(nums: &Vec<i32>, l: i32, r: i32, target: i32) -> Vec<i32> {
        if nums.len() == 0 || l > r {
            return vec![-1, -1];
        }
        let mid = (l + r) / 2;
        let m = nums[mid as usize];
        if m < target {
            return search_inner(nums, mid + 1, r, target);
        } else if m > target {
            return search_inner(nums, l, mid - 1, target);
        } else {
            let l = search_inner(nums, l, mid - 1, target);
            let r = search_inner(nums, mid + 1, r, target);
            let l = if l[0] == -1 { mid as i32 } else { l[0] };
            let r = if r[1] == -1 { mid as i32 } else { r[1] };
            return vec![l, r];
        }
    }
    search_inner(&nums, 0, nums.len() as i32 - 1, target)
}