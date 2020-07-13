// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_some() != q.is_some() {
            return false;
        }
        
        if p.is_none() {
            return true;
        }
        
        let pn = p.unwrap();
        let qn = q.unwrap();
        
        let pb = pn.borrow();
        let qb = qn.borrow();
        
        if pb.val == qb.val {
            Self::is_same_tree(pb.left.clone(), qb.left.clone()) && Self::is_same_tree(pb.right.clone(), qb.right.clone())
        } else {
            false
        }
    }
}
