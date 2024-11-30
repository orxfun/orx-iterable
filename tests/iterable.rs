mod common_testers;
use common_testers::test_it;
mod custom_iterables;
use std::collections::{LinkedList, VecDeque};

#[test]
fn std_collections() {
    let values = || vec![1, 3, 7];

    test_it(values(), &[1, 3, 7]);
    test_it(values(), &vec![1, 3, 7]);
    test_it(values(), &VecDeque::from_iter([1, 3, 7].into_iter()));
    test_it(values(), &LinkedList::from_iter([1, 3, 7].into_iter()));
}

#[test]
fn custom_collection() {
    let col = custom_iterables::EvensThenOdds {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test_it(vec![4, 12, 8, 2, 1, 7], &col);
}
