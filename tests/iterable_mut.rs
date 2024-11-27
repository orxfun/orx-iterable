use orx_iterable::*;
use std::collections::{LinkedList, VecDeque};

fn take_owned(mut iter: impl IterableMut<Item = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.it_mut() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.it_mut() {
        *y += 1;
    }

    let count = iter.it_mut().count();
    let mut x = 0;
    for y in iter.it_mut() {
        x += *y;
    }
    assert_eq!(x, sum + count);
}

fn take_mut_ref(iter: &mut impl IterableMut<Item = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.it_mut() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.it_mut() {
        *y += 1;
    }

    let count = iter.it_mut().count();
    let mut x = 0;
    for y in iter.it_mut() {
        x += *y;
    }
    assert_eq!(x, sum + count);
}

#[test]
fn iterable_mut_array() {
    let mut data = [3, 2, 6, 1, 0, 7];
    take_mut_ref(&mut data, 19);

    let data = [3, 2, 6, 1, 0, 7];
    take_owned(data, 19);
}

#[test]
fn iterable_mut_slice() {
    let mut vec = vec![3, 2, 6, 1, 0, 7];

    let slice = vec.as_mut_slice();
    take_owned(slice.into_iterable_mut(), 19);
}

#[test]
fn iterable_mut_std_owned_collections() {
    macro_rules! test_std_collection {
        ($V:ty) => {
            let mut data: $V = [3, 2, 6, 1, 0, 7].into_iter().collect();
            take_mut_ref(&mut data, 19);

            let data: $V = [3, 2, 6, 1, 0, 7].into_iter().collect();
            take_owned(data, 19);
        };
    }

    test_std_collection!(Vec<_>);
    test_std_collection!(VecDeque<_>);
    test_std_collection!(LinkedList<_>);
}
