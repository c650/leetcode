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

struct Pair {
  node: Rc<RefCell<TreeNode>>,
  height: i32,
}

fn pop_and_add(stk: &mut Vec<Pair>, val: i32, height: i32) {
  let new_pair = Pair {
    node: Rc::new(RefCell::new(TreeNode::new(val))),
    height,
  };

  {
    let parent: &mut Pair = loop {
      if stk.is_empty() {
        panic!("aaaaah");
      }

      if stk.last().unwrap().height >= height {
        stk.pop();
        continue;
      }

      break stk.last_mut().unwrap();
    };

    let mut parent_node = parent.node.borrow_mut();

    if parent_node.left.is_none() {
      parent_node.left = Some(new_pair.node.clone());
    } else {
      parent_node.right = Some(new_pair.node.clone());
    }
  }

  stk.push(new_pair);
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sitr = s.chars();

    let mut stk: Vec<Pair> = vec![];

    stk.push(Pair {
      node: Rc::new(RefCell::new(TreeNode::new(-1))),
      height: -1,
    });

    let mut reading_height: bool = true;
    let mut val = 0;
    let mut height = 0;
    while let Some(c) = sitr.next() {
      match c {
        n @ '0'..='9' => {
          reading_height = false;
          val = val * 10 + (n as i32) - ('0' as i32);
        }
        _ => {
          if reading_height {
            height += 1
          } else {
            pop_and_add(&mut stk, val, height);

            reading_height = true;
            height = 1;
            val = 0;
          }
        }
      }
    }

    pop_and_add(&mut stk, val, height);

    stk.reverse();
    stk.pop();

    Some(stk.pop().unwrap().node)
  }
}
