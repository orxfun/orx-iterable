#![allow(dead_code)]

use orx_iterable::*;

pub fn test_it<'a>(values: Vec<usize>, col: impl Iterable<Item = &'a usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().copied().sum::<usize>(), sum);
}

pub fn test_it_val(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().sum::<usize>(), sum);
}

pub fn test_col(values: Vec<usize>, col: impl Collection<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);
}
