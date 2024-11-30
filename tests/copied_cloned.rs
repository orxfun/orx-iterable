mod custom_iterables;
use orx_iterable::*;
use std::collections::{LinkedList, VecDeque};

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn copied_cloned() {
    let values = || vec![1, 3, 7];

    test_it(values(), [1, 3, 7].copied());
    test_it(values(), vec![1, 3, 7].copied());
    test_it(
        values(),
        VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test_it(
        values(),
        LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    test_it(values(), [1, 3, 7].cloned());
    test_it(values(), vec![1, 3, 7].cloned());
    test_it(
        values(),
        VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test_it(
        values(),
        LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    let col = custom_iterables::EvensThenOdds {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test_it(vec![4, 12, 8, 2, 1, 7], col.copied());
    test_it(vec![4, 12, 8, 2, 1, 7], col.cloned());
}
