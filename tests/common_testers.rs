#![allow(dead_code)]

use orx_iterable::*;

pub fn test_it<'a>(values: Vec<usize>, col: impl Iterable<Item = &'a usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);
}

pub fn test_col(values: Vec<usize>, mut col: impl IterableCol<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);

    // tests - mut
    for x in col.iter_mut() {
        *x += 1;
    }

    for x in col.iter_mut() {
        *x += 2;
    }

    let new_sum = sum + 3 * count;
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), new_sum);

    let new_values = values.iter().map(|x| x + 3).collect::<Vec<_>>();
    assert_eq!(col.iter().copied().collect::<Vec<_>>(), new_values);
}
