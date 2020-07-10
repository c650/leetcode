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
use std::collections::VecDeque;

struct Pair {
    pub first: Rc<RefCell<TreeNode>>,
    pub second: usize,
}

fn enq(cur: &Pair, q: &mut VecDeque<Pair>) {
    if let Some(left) = &(*cur).first.borrow().left {
        q.push_back(Pair{first: left.clone(), second: (*cur).second + 1});
    }
    
    if let Some(right) = &(*cur).first.borrow().right {
        q.push_back(Pair{first: right.clone(), second: (*cur).second + 1});
    }
}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q: VecDeque<Pair> = VecDeque::new();
        
        let mut ans: Vec<Vec<i32>> = Vec::new();
                
        if let Some(r) = root {
            q.push_back(Pair{first: r, second: 0})
        }
        
        while let Some(cur) = q.pop_front() {
            enq(&cur, &mut q);
            if (ans.len() <= cur.second) {
                ans.push(Vec::new());
            }
            ans[cur.second].push(cur.first.borrow().val);
        }
        
        ans.reverse();
        
        ans
    }
}
