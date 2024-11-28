// use orx_iterable::*;
// use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};

// fn test_elements_sum_val(iter: impl IterableOnce<Item = usize>, sum: usize) {
//     assert_eq!(iter.it_once().sum::<usize>(), sum);
// }

// fn test_elements_sum_ref<'a>(iter: impl IterableOnce<Item = &'a usize>, sum: usize) {
//     assert_eq!(iter.it_once().sum::<usize>(), sum);
// }

// #[test]
// fn iterable_once_array() {
//     let data = [3, 2, 6, 1, 0, 7];
//     test_elements_sum_val(data, 19);
// }

// #[test]
// fn iterable_once_std_owned_collections() {
//     macro_rules! test_std_collection {
//         ($V:ty) => {
//             let data: $V = [3, 2, 6, 1, 0, 7].into_iter().collect();
//             test_elements_sum_val(data, 19);
//         };
//     }

//     test_std_collection!(Vec<_>);
//     test_std_collection!(VecDeque<_>);
//     test_std_collection!(LinkedList<_>);
//     test_std_collection!(HashSet<_>);
//     test_std_collection!(BTreeSet<_>);
// }

// #[test]
// fn iterable_once_iter() {
//     let vec = vec![3, 2, 6, 1, 0, 7];
//     test_elements_sum_ref(vec.iter(), 19);
//     test_elements_sum_val(vec.iter().copied(), 19);

//     let vec = vec![3, 2, 6, 1, 0, 7, 33];
//     test_elements_sum_ref(vec.iter().filter(|x| **x < 33), 19);
//     test_elements_sum_val(vec.iter().copied().filter(|x| *x < 33), 19);

//     let vec = vec![3, 2, 6, 1, 0, 7, 33];
//     test_elements_sum_ref(vec.iter().take(6), 19);
//     test_elements_sum_val(vec.iter().copied().take(6), 19);

//     let vec = vec![3, 2, 6, 1, 0, 7];
//     test_elements_sum_val(vec.iter().map(|x| x * 2), 38);
// }
