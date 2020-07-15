impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut it = s.split_whitespace().rev();
                
        let first = match it.nth(0) {
            Some(s) => s,
            None => ""
        };
        
        it.fold(first.to_string(), |acc, s| acc + " " + s)
    }
}
