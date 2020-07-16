fn go(memo: &mut Vec<Vec<i32>>, s: &Vec<i32>, i: usize, j: usize) -> i32 {
    if i >= s.len() {
        return 0;
    }

    let mut ret = memo[i][j];
    if ret != -1 {
        return ret;
    }

    ret = std::cmp::max(0, s[i] * (j as i32 + 1) + go(memo, s, i+1, j+1));
    ret = std::cmp::max(ret, go(memo, s, i + 1, j));

    memo[i][j] = ret;
    
    ret
}

impl Solution {
        
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut s = satisfaction.clone();
        s.sort();
        
        let mut memo: Vec<Vec<i32>> = std::iter::repeat(std::iter::repeat(-1).take(501).collect()).take(501).collect();
        
        go(&mut memo, &s, 0, 0)
    }
}
