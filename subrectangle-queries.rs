#[derive(Clone)]
struct SegmentTree {
    n: usize,
    delta: Vec<i32>,
    value: Vec<i32>,
}

impl SegmentTree {
    fn new(nn: usize) -> Self {
        SegmentTree {
            n: nn,
            delta: vec![-1; 4 * nn + 1],
            value: vec![-1; 4 * nn + 1],
        }
    }
    
    fn prop(&mut self, i: usize) {
        if self.delta[i] == -1 {
            return;
        }    
        self.set(2 * i + 1, self.delta[i]);
        self.set(2 * i + 2, self.delta[i]);
        self.delta[i] = -1;
    }

    fn set(&mut self, i: usize, val: i32) {
        self.delta[i] = val;
        self.value[i] = val;
    }
    
    fn upd_inner(&mut self, i: usize, rl: usize, rr: usize, left: usize, right: usize, val: i32) {
        if rl == left && rr == right {
            return self.set(i, val);
        }
        
        self.prop(i);
        
        let mid = (rl + rr) / 2;
        if left <= mid {
            self.upd_inner(2 * i + 1, rl, mid, left, std::cmp::min(mid, right), val);
        }
        
        if mid < right {
            self.upd_inner(2 * i + 2, mid+1, rr, std::cmp::max(left, mid+1), right, val);
        }
    }

    fn upd(&mut self, left: usize, right: usize, val: i32) {
        self.upd_inner(0, 0, self.n-1, left, right, val)
    }

    fn qry(&mut self, idx: usize) -> i32 {
        let mut lo = 0;
        let mut hi = self.n - 1;
        let mut i = 0;

        while lo < hi {
            self.prop(i);
            let mid = (lo + hi) / 2;
            if idx <= mid {
                i = 2 * i + 1;
                hi = mid;
            } else {
                i = 2 * i + 2;
                lo = mid+1;
            }
        }

        self.value[i]
    }
}

// impl std::clone::Clone for SegmentTree {
    
// }

struct SubrectangleQueries {
    segs: Vec<SegmentTree>,
}

impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let cols = rectangle[0].len();
        let mut ret = SubrectangleQueries {
            segs: std::iter::repeat(SegmentTree::new(cols)).take(rectangle.len()).collect()
        };
        for (i, row) in rectangle.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                ret.segs[i].upd(j, j, *cell);
            }
        }
        ret
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            self.segs[i as usize].upd(col1 as usize, col2 as usize, new_value);
        }
    }
    
    fn get_value(&mut self, row: i32, col: i32) -> i32 {
        self.segs[row as usize].qry(col as usize)
    }
}

// fn main() {
//     let mut s: SubrectangleQueries = SubrectangleQueries::new(std::iter::repeat((1..6).collect()).take(2).collect());

//     assert_eq!(1, s.get_value(0, 0));
//     assert_eq!(2, s.get_value(0, 1));

//     s.update_subrectangle(1, 0, 1, 4, 1000);
    
//     assert_eq!(1, s.get_value(0, 0));
//     assert_eq!(2, s.get_value(0, 1));

//     assert_eq!(1000, s.get_value(1, 0));
// }
