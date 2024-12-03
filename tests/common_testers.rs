#![allow(dead_code)]

use orx_iterable::*;

pub fn test_it<'a>(values: Vec<usize>, col: impl Iterable<Item = &'a usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.it().count(), count);
    assert_eq!(col.it().copied().sum::<usize>(), sum);
}

pub fn test_col(values: Vec<usize>, col: impl IterableCol<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);
}
