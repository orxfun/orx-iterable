use orx_iterable::*;
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

fn take_owned(mut iter: impl IterableMut<Item = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.iter_mut() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.iter_mut() {
        *y += 1;
    }

    let count = iter.iter_mut().count();
    let mut x = 0;
    for y in iter.iter_mut() {
        x += *y;
    }
    assert_eq!(x, sum + count);
}

fn take_mut_ref(iter: &mut impl IterableMut<Item = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.iter_mut() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.iter_mut() {
        *y += 1;
    }

    let count = iter.iter_mut().count();
    let mut x = 0;
    for y in iter.iter_mut() {
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
    take_owned(slice.iterable_mut(), 19);
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

#[test]
fn iterable_std_pair_collections() {
    // TODO! this doesn't work yet.
    fn test<'a>(mut iter: impl IterableMut<Item = u32>) {
        for x in iter.iter_mut() {
            *x += 1;
        }
        for x in iter.iter_mut() {
            *x -= 1;
        }
        assert_eq!(iter.iter_mut().map(|x| *x).sum::<u32>(), 42);
    }

    let mut map: HashMap<u64, u32> = [(1, 40), (3, 2)].into_iter().collect();
    // test(map.taken(10));
    // test(map.taken_while(|x| x.1 % 2 == 0));

    let map: BTreeMap<u64, u32> = [(1, 40), (3, 2)].into_iter().collect();
    // test(&map);
    // test(map.taken(10));
    // test(map.taken_while(|x| x.1 % 2 == 0));
}
