mod custom_iterables;
use orx_iterable::*;
use std::collections::{LinkedList, VecDeque};

#[test]
fn copied_cloned() {
    fn test(values: Vec<usize>, col: impl Iterable<Item = usize>) {
        let sum = values.iter().sum::<usize>();
        let count = values.len();

        // tests
        assert_eq!(col.iter().count(), count);
        assert_eq!(col.iter().sum::<usize>(), sum);
    }

    let values = || vec![1, 3, 7];

    test(values(), [1, 3, 7].copied());
    test(values(), vec![1, 3, 7].copied());
    test(
        values(),
        VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test(
        values(),
        LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    test(values(), [1, 3, 7].cloned());
    test(values(), vec![1, 3, 7].cloned());
    test(
        values(),
        VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test(
        values(),
        LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    let col = custom_iterables::EvensThenOdds {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test(vec![4, 12, 8, 2, 1, 7], col.copied());
    test(vec![4, 12, 8, 2, 1, 7], col.cloned());
}

#[cfg(feature = "std")]
#[test]
fn obj_copied_cloned() {
    fn test(values: Vec<usize>, col: &dyn orx_iterable::obj_safe::IterableObj<Item = usize>) {
        let sum = values.iter().sum::<usize>();
        let count = values.len();

        // tests
        assert_eq!(col.boxed_iter().count(), count);
        assert_eq!(col.boxed_iter().sum::<usize>(), sum);
    }

    let values = || vec![1, 3, 7];

    test(values(), &[1, 3, 7].copied());
    test(values(), &vec![1, 3, 7].copied());
    test(
        values(),
        &VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test(
        values(),
        &LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    test(values(), &[1, 3, 7].cloned());
    test(values(), &vec![1, 3, 7].cloned());
    test(
        values(),
        &VecDeque::from_iter([1, 3, 7].into_iter()).copied(),
    );
    test(
        values(),
        &LinkedList::from_iter([1, 3, 7].into_iter()).copied(),
    );

    let col = custom_iterables::EvensThenOdds {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test(vec![4, 12, 8, 2, 1, 7], &col.copied());
    test(vec![4, 12, 8, 2, 1, 7], &col.cloned());
}
