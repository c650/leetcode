use std::collections::HashMap;

fn go(
  memo: &mut Vec<i32>,
  letters: &Vec<char>,
  words: &Vec<String>,
  score: &Vec<i32>,
  mask: usize,
) -> i32 {
  let mut ret: i32 = memo[mask];
  if ret != -1 {
    return ret;
  }

  let mut hm: HashMap<char, i32> = HashMap::new();

  letters
    .iter()
    .for_each(|letter| *hm.entry(*letter).or_insert(0) += 1);

  words
    .iter()
    .enumerate()
    .filter(|(i, _)| ((1usize << i) & mask) != 0)
    .for_each(|(_, e)| e.chars().for_each(|c| *hm.entry(c).or_insert(0) -= 1));

  ret = 0;
  for (i, e) in words.iter().enumerate() {
    if ((1 << i) & mask) != 0 {
      continue;
    }

    let mut my_hm: HashMap<char, i32> = HashMap::new();
    e.chars().for_each(|c| *my_hm.entry(c).or_insert(0) += 1);

    let can_do = my_hm
      .iter()
      .fold(true, |acc, (k, v)| acc && v <= hm.get(k).unwrap_or(&0));

    if can_do {
      let reward = e.chars().fold(0, |acc, c| {
        acc + score[((c as i32) - ('a' as i32)) as usize]
      });
      ret = std::cmp::max(
        ret,
        reward + go(memo, &letters, &words, &score, mask | (1 << i)),
      );
    }
  }

  memo[mask] = ret;

  ret
}

impl Solution {
  pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut memo = vec![-1; 1 << 14];

    let mut letters_hm: HashMap<char, i32> = HashMap::new();
    for c in letters.iter() {
      letters_hm.insert(*c, 0);
    }
    for c in letters.iter() {
      let opt = letters_hm.get_mut(c);
      *opt.unwrap() += 1;
    }

    go(&mut memo, &letters, &words, &score, 0)
  }
}

