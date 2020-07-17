use std::collections::HashMap;
use std::vec::Vec;

impl Solution {
  pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for e in nums.iter() {
      let val = match map.get(e) {
        None => 0,
        Some(&freq) => freq.clone(),
      } + 1;

      map.insert(*e, val);
    }

    let mut inv: HashMap<i32, Vec<i32>> = HashMap::new();

    map.iter().for_each(|(_, &v)| {
      inv.insert(v, Vec::new());
    });
    map.iter().for_each(|(&k, v)| {
      inv.get_mut(v).unwrap().push(k.clone());
    });

    let mut ret = vec![];
    for i in 1..=nums.len() {
      if let Some(e) = inv.get(&(i as i32)) {
        for x in e.iter() {
          ret.push(*x);
        }
      }
    }

    ret.reverse();

    while ret.len() > (k as usize) {
      ret.pop();
    }

    ret
  }
}
