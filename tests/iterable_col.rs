mod common_testers;
mod custom_iterables;
use common_testers::test_col;
use std::collections::{LinkedList, VecDeque};

#[test]
fn std_collections() {
    let values = || vec![1, 3, 7];

    test_col(values(), [1, 3, 7]);
    test_col(values(), vec![1, 3, 7]);
    test_col(values(), VecDeque::from_iter([1, 3, 7].into_iter()));
    test_col(values(), LinkedList::from_iter([1, 3, 7].into_iter()));
}

#[test]
fn custom_collection() {
    let col = custom_iterables::EvensThenOddsCol {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test_col(vec![4, 12, 8, 2, 1, 7], col);
}
