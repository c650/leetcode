impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut roll = if n < 0 { 1.0 / x } else { x };
        let mut res: f64 = 1.0;
        
        let mut nn = (n as i64).abs();
        while nn > 0 {
           if nn % 2 == 0 {
               roll = roll * roll;
               nn /= 2;
            } else {
                res = res * roll;
                nn -= 1;
            }
        }
        res
    }
}
