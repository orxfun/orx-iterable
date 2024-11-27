use orx_iterable::*;
use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};

fn test_sum_ref<'a>(iter: impl Iterable<Item = &'a usize>, sum: usize) {
    assert_eq!(iter.it().sum::<usize>(), sum);
    assert_eq!(iter.it().sum::<usize>(), sum);
}

fn test_sum_val(iter: impl Iterable<Item = usize>, sum: usize) {
    assert_eq!(iter.it().sum::<usize>(), sum);
    assert_eq!(iter.it().sum::<usize>(), sum);
}

#[test]
fn iterable_array() {
    let data = [3, 2, 6, 1, 0, 7];
    test_sum_ref(&data, 19);

    test_sum_val(data.cloned(), 19);
    test_sum_val(data.copied(), 19);
}

#[test]
fn iterable_slice_vec() {
    let vec = vec![3, 2, 6, 1, 0, 7];

    let data = vec.as_slice();
    test_sum_ref(data, 19);
    test_sum_val(data.cloned(), 19);
    test_sum_val(data.copied(), 19);

    test_sum_ref(&vec, 19);
    test_sum_val(vec.cloned(), 19);
    test_sum_val(vec.copied(), 19);
}

#[test]
fn iterable_std_owned_collections() {
    macro_rules! test_std_collection {
        ($V:ty) => {
            let data: $V = [3, 2, 6, 1, 0, 7].into_iter().collect();
            test_sum_ref(&data, 19);

            test_sum_val(data.cloned(), 19);
            test_sum_val(data.copied(), 19);

            // cloned() does not consume data
            test_sum_ref(&data, 19);
        };
    }

    test_std_collection!(Vec<_>);
    test_std_collection!(VecDeque<_>);
    test_std_collection!(LinkedList<_>);
    test_std_collection!(HashSet<_>);
    test_std_collection!(BTreeSet<_>);
}

#[test]
fn iterable_cloned_iter() {
    let vec = vec![3, 2, 6, 1, 0, 7, 33];
    let iter = || vec.iter().filter(|x| **x < 33);

    test_sum_ref(iter().into_iterable(), 19);
    test_sum_val(iter().into_iterable().cloned(), 19);
    test_sum_val(iter().into_iterable().copied(), 19);
    test_sum_val(iter().copied().into_iterable(), 19);
    test_sum_val(iter().cloned().into_iterable(), 19);
}

#[test]
fn iterable_type_members() {
    struct Graph {
        edges: Vec<Vec<usize>>,
    }

    impl Graph {
        fn out_edges(&self, i: usize) -> &[usize] {
            &self.edges[i]
        }

        fn out_edges_iter(&self, i: usize) -> impl Iterator<Item = &usize> + Clone {
            self.edges[i].iter()
        }
    }

    let graph = Graph {
        edges: vec![vec![3, 2], vec![3, 2, 6, 1, 0, 7], vec![]],
    };

    let i = 1;
    test_sum_ref(&graph.edges[i], 19);
    test_sum_ref(graph.out_edges(i), 19);
    test_sum_ref(graph.out_edges_iter(i).into_iterable(), 19);

    test_sum_val(graph.edges[i].cloned(), 19);
    test_sum_val(graph.out_edges(i).cloned(), 19);
    test_sum_val(graph.out_edges_iter(i).into_iterable().cloned(), 19);

    test_sum_val(graph.edges[i].copied(), 19);
    test_sum_val(graph.out_edges(i).copied(), 19);
    test_sum_val(graph.out_edges_iter(i).into_iterable().copied(), 19);
}
