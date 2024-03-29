use crate::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

test! {}

#[derive(Default)]
struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn is_odd(&self) -> bool {
        self.small.len() / 2 == 1
    }

    fn add_num(&mut self, num: i32) {
        if self.is_odd() {
            self.small.push(num);
            if let Some(max) = self.small.pop() {
                self.large.push(Reverse(max));
            }
        } else {
            self.large.push(Reverse(num));
            if let Some(Reverse(min)) = self.large.pop() {
                self.small.push(min);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.is_odd() {
            *self.small.peek().unwrap() as f64
        } else {
            (self.small.peek().unwrap() + self.large.peek().unwrap().0) as f64 / 2.0
        }
    }
}
