fn go(memo: &mut Vec<Vec<i32>>, mask: usize, hat_idx: usize, hats: &Vec<Vec<i32>>) -> i32 {
  let n = hats.len();

  let moduu = 1000000007;

  if (1usize << n) - 1 == mask {
    return 1;
  }

  if hat_idx > 40 {
    return 0;
  }

  let mut ret = memo[hat_idx][mask];
  if ret != -1 {
    return ret;
  }

  ret = go(memo, mask, hat_idx + 1, hats);

  for (i, e) in hats.iter().enumerate() {
    if (1usize << i) & mask != 0 {
      continue;
    }

    let can_do = e.iter().any(|hat| *hat as usize == hat_idx);

    if can_do {
      ret = (ret + go(memo, mask | (1usize << i), hat_idx + 1, hats)) % moduu;
    }
  }

  memo[hat_idx][mask] = ret;
  ret
}


impl Solution {
  pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    let mut memo = vec![vec![-1; 1usize << hats.len()]; 41];

    go(&mut memo, 0, 1, &hats)
  }
}
