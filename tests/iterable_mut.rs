use orx_iterable::*;
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

fn take_owned<'a>(mut iter: impl IterableMut<ItemMut = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.xyz() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.xyz() {
        *y += 1;
    }

    let count = iter.xyz().count();
    let mut x = 0;
    for y in iter.xyz() {
        x += *y;
    }
    assert_eq!(x, sum + count);
}

fn take_mut_ref<'a>(iter: &mut impl IterableMut<ItemMut = usize>, sum: usize) {
    let mut x = 0;
    for y in iter.xyz() {
        x += *y;
    }
    assert_eq!(x, sum);

    for y in iter.xyz() {
        *y += 1;
    }

    let count = iter.xyz().count();
    let mut x = 0;
    for y in iter.xyz() {
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

// #[test]
// fn iterable_mut_std_pair_collections() {
//     // TODO! this doesn't work yet.
//     fn test<'a>(mut iter: impl IterableMut<ItemMut = u32>) {
//         for x in iter.xyz() {
//             *x += 1;
//         }
//         for x in iter.xyz() {
//             *x -= 1;
//         }
//         assert_eq!(iter.xyz().map(|x| *x).sum::<u32>(), 42);
//     }

//     let mut map: HashMap<u64, u32> = [(1, 40), (3, 2)].into_iter().collect();
//     // test(map.taken(10));
//     // test(map.taken_while(|x| x.1 % 2 == 0));

//     let map: BTreeMap<u64, u32> = [(1, 40), (3, 2)].into_iter().collect();
//     // test(&map);
//     // test(map.taken(10));
//     // test(map.taken_while(|x| x.1 % 2 == 0));
// }

#[test]
fn iterable_mut_chained() {
    fn add_two(mut iter: impl IterableMut<ItemMut = usize>, original_sum: usize) {
        for x in iter.xyz() {
            *x += 1;
        }
        for x in iter.xyz() {
            *x += 1;
        }

        let sum = iter.xyz().count() * 2 + original_sum;
        assert_eq!(iter.xyz().map(|x| *x).sum::<usize>(), sum);
    }

    let mut a: Vec<usize> = vec![3, 2, 1];
    let mut b: Vec<usize> = vec![33, 44];
    add_two(a.chained_mut(&mut b), 83);

    let mut a: Vec<usize> = vec![3, 2, 1];
    let mut b: Vec<usize> = vec![33, 44];
    let mut c: Vec<usize> = vec![100];
    add_two(a.chained_mut(&mut b).chained_mut(&mut c), 183);
}

#[test]
fn iterable_mut_filtered() {
    let mut vec = vec![3, 2, 6, 1, 0, 7, 33];

    let mut iterable = vec.filtered_mut(|x| *x > 30);
    for x in iterable.xyz() {
        *x += 100;
    }
    for x in iterable.xyz() {
        *x += 100;
    }

    assert_eq!(vec, vec![3, 2, 6, 1, 0, 7, 233]);
}

#[test]
fn iterable_mut_flattened() {
    let mut data = vec![vec![1, 2], vec![6, 0, 7], vec![3]];

    let mut iter = data.flattened_mut();
    for x in iter.xyz() {
        *x += 10;
    }
    for x in iter.xyz() {
        *x += 100;
    }

    assert_eq!(data, vec![vec![111, 112], vec![116, 110, 117], vec![113]]);
}

#[test]
fn iterable_mut_skipped() {
    let mut data = vec![2, 4, 12, 3, 8, 4];

    let mut iter = data.skipped_mut(4);
    for x in iter.xyz() {
        *x += 10;
    }
    for x in iter.xyz() {
        *x += 10;
    }

    assert_eq!(data, vec![2, 4, 12, 3, 28, 24]);
}
