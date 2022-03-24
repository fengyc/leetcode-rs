use std::alloc::Global;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut r = Vec::with_capacity(k as usize);
        let mut size = 0;
        if head.is_some() {}
        let c = size / k;
        let m = size % k;
        let mut p = head;
        for i in 0..k {
            let mut p2 = &p;
            for _ in 0..c {
                match p2 {
                    None => {}
                    Some(b) => {p2 = Some(b.clone())}
                }
            }
        }
        r
    }
}

fn main() {
    println!("Hello, world!");
}
