#![allow(dead_code)]

use orx_iterable::*;

pub fn test_it<'a>(values: Vec<usize>, col: impl Iterable<Item = &'a usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);
}

pub fn test_it_val(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

pub fn test_col(values: Vec<usize>, col: impl Collection<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().copied().sum::<usize>(), sum);
}

// obj

#[cfg(feature = "std")]
pub fn obj_test_it<'a>(
    values: Vec<usize>,
    col: &dyn orx_iterable::obj_safe::IterableObj<Item = &'a usize>,
) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().copied().sum::<usize>(), sum);
}

#[cfg(feature = "std")]
pub fn obj_test_it_val(
    values: Vec<usize>,
    col: &dyn orx_iterable::obj_safe::IterableObj<Item = usize>,
) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().sum::<usize>(), sum);
}

#[cfg(feature = "std")]
pub fn obj_test_col(
    values: Vec<usize>,
    col: &dyn orx_iterable::obj_safe::CollectionObj<Item = usize>,
) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().copied().sum::<usize>(), sum);
}
