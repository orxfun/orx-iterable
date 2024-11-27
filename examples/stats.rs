use orx_iterable::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Stats {
    mean: u32,
    std_dev: u32,
}

impl Stats {
    fn new(mean: u32, std_dev: u32) -> Self {
        Self { mean, std_dev }
    }
}

fn calc_stats(values: impl Iterable<Item = u32>) -> Stats {
    let count = values.iter().count() as u32;

    let sum: u32 = values.iter().sum();

    let mean = match count {
        0 => 0,
        n => sum / n,
    };

    let sum_sq_errors: u32 = values
        .iter()
        .map(|x| {
            let diff = u32::abs_diff(x, mean);
            diff * diff
        })
        .sum();
    let std_dev_sq = match count {
        0 | 1 => 0,
        n => sum_sq_errors / (n - 1),
    };
    let std_dev = f32::sqrt(std_dev_sq as f32) as u32;

    Stats { mean, std_dev }
}

struct FibUpTo(u32);

impl<'a> IntoIterator for &'a FibUpTo {
    type Item = u32;

    type IntoIter = FibonacciIterator;

    fn into_iter(self) -> Self::IntoIter {
        FibonacciIterator {
            curr: 0,
            next: 1,
            up_to: self.0,
        }
    }
}

#[derive(Clone)]
struct FibonacciIterator {
    curr: u32,
    next: u32,
    up_to: u32,
}

impl Iterator for FibonacciIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        match current > self.up_to {
            false => Some(current),
            true => None,
        }
    }
}

fn main() {
    // # Vec

    let values = vec![30, 10, 70, 50];

    // - use &Vec<T> as iterable
    let it = values.copied();
    assert_eq!(calc_stats(it), Stats::new(40, 25));

    // - use &[T] as iterable
    let it = values.as_slice().copied();
    assert_eq!(calc_stats(it), Stats::new(40, 25));

    // - convert an iterator into iterable
    let it = values.iter().filter(|x| **x < 40).copied().into_iterable();
    assert_eq!(calc_stats(it), Stats::new(20, 14));

    // - move values, but make it multiple times iterable
    let it = values.move_into_iterable().cloned();
    assert_eq!(calc_stats(it), Stats::new(40, 25));

    // # Range
    let it = (1..=100).into_iterable();
    assert_eq!(calc_stats(it), Stats::new(50, 29));

    // # Set

    let values: HashSet<u32> = [30, 70, 50, 10].into_iter().collect();

    // - use &HashSet<T> as iterable
    let it = values.copied();
    assert_eq!(calc_stats(it), Stats::new(40, 25));

    // - convert an iterator into iterable
    let it = values.iter().filter(|x| **x < 40).copied().into_iterable();
    assert_eq!(calc_stats(it), Stats::new(20, 14));

    // - move values, but make it multiple times iterable
    let it = values.move_into_iterable().cloned();
    assert_eq!(calc_stats(it), Stats::new(40, 25));

    // # Closure & Iterator

    let exponent = Box::new(3);
    let pow = |x: u32| u32::pow(x, *exponent);
    let range = 2..7;
    let it = range.map(pow).into_iterable();
    assert_eq!(calc_stats(it), Stats::new(88, 84));

    // # My IntoIterator

    let values = FibUpTo(100);
    assert_eq!(calc_stats(&values), Stats::new(19, 27));
}
