use std::collections::HashMap;

fn encode(cells: &Vec<i32>) -> i32 {
    let mut ret = 0;
    for (i, e) in cells.iter().enumerate() {
        ret |= (e << i);
    }
    ret
}

fn proceed(cells: &Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; cells.len()];

    for i in 1..=(ans.len()-2) {
        if (cells[i-1] == cells[i+1]) {
            ans[i] = 1;
        }
    }

    ans
}

fn go(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut cur = cells.clone();
    let mut last_rank = 0;
    loop {
        let next_vec = proceed(&cur);
        let next_rank = 1 + last_rank;
        
        if next_rank == n {
            break next_vec;
        }

        cur = next_vec;
        last_rank = next_rank;
    }
}

impl Solution {
    
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut next: HashMap<i32, i32> = HashMap::new();
        let mut rank: HashMap<i32, i32> = HashMap::new();
        
        rank.insert(encode(&cells), 0);
        
        let (ra, rb) = {
            let mut cur = cells.clone();
            loop {
                let cur_code = &encode(&cur);
                let next_vec = proceed(&cur);
                let next_code = &encode(&next_vec);
                
                if next.get(cur_code).is_some() {
                    break (rank.get(cur_code).unwrap(), rank.get(next_code).unwrap());
                }
                
                let new_rank = 1 + rank.get(cur_code).unwrap();
                
                next.insert(*cur_code, *next_code);
                rank.insert(*next_code, new_rank);
                
                cur = next_vec;
            }
        };
        
        let cycle_len = (ra - rb).abs() + 1;
        
        let start = std::cmp::min(ra, rb);
        
        if n < *start {
            go(cells.clone(), n)
        } else {
            let except_initial = n - start;
            let after_start = except_initial % cycle_len;
            go(cells.clone(), start + after_start)
        }
    }
}
