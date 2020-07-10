impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut lo = 0 as i64;
        let mut hi = 2e9 as i64;
        
        let check = |mid: i64| mid * (mid+1) / 2;
        
        while (lo < hi) {
            let mid: i64 = (lo + (hi - lo) / 2);
            if (check(mid) > n.into()) {
                hi = mid-1;
            } else {
                lo = mid+1;
                if (check(lo) > n.into()) {
                    lo = mid;
                    hi = mid;
                }
            }
        }
        
        return lo as i32;
    }
}
