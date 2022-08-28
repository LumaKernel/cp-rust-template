use std::cmp::{max, min, Ordering};

pub trait SwapMax {
    fn smax(&mut self, v: Self);
}

pub trait SwapMin {
    fn smin(&mut self, v: Self);
}

impl<T: Ord> SwapMax for T {
    fn smax(&mut self, v: Self) {
        if (*self).cmp(&v) == Ordering::Less {
            *self = v;
        }
    }
}

impl<T: Ord + Clone> SwapMin for T {
    fn smin(&mut self, v: Self) {
        if (*self).cmp(&v) == Ordering::Greater {
            *self = v;
        }
    }
}
