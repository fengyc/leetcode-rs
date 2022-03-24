fn main() {
    let mut t0 = make_node(3);
    let mut t1 = make_node(4);
    let mut t2 = make_node(5);

    t0.borrow_mut().left = Some(t1.clone());
    t0.borrow_mut().right = Some(t2.clone());

    let s = Solution::is_valid_bst(Some(t0));
    println!("{}", s);

    let t3 = make_node(5);
    t3.borrow_mut().left = Some(t1.clone());
    t3.borrow_mut().right = Some(t2.clone());

    let s = Solution::is_valid_bst(Some(t3));
    println!("{}", s);
}

fn make_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

pub struct Solution {}

impl Solution {

    pub fn valid(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, bool) {
        if let Some(root) = root {
            let root = root.borrow();

            let l = if let Some(l) = root.left.clone() {
                let lr = Self::valid(Some(l));
                if !lr.2 || root.val < lr.1 {
                    return (0, 0, false);
                }
                lr.0
            } else {
                root.val
            };

            let r = if let Some(r) = root.right.clone() {
                let rr = Self::valid(Some(r));
                if !rr.2 || root.val > rr.0 {
                    return (0, 0, false);
                }
                rr.1
            } else {
                root.val
            };

            return (l, r, true);
        }
        (0, 0, true)
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let r = Self::valid(root);
        r.2
    }
}