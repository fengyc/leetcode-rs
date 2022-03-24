fn main() {
    let mut t0 = TreeNode::new(0);
    let mut t1 = TreeNode::new(1);
    let mut t2 = TreeNode::new(2);

    t0.left = Some(Rc::new(RefCell::new(t1)));
    t0.right = Some(Rc::new(RefCell::new(t2)));

    let v = preorder_traversal(Some(Rc::new(RefCell::new(t0))));

    println!("{:?}", v);
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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v = Vec::with_capacity(1);
    if let Some(root) = root {
        v.push(root.borrow().val);
        let mut left = preorder_traversal(root.borrow().left.clone());
        let mut right = preorder_traversal(root.borrow().right.clone());
        v.append(&mut left);
        v.append(&mut right);
    }
    v
}
