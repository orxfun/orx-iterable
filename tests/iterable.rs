use orx_iterable::*;
use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};

fn test_sum_ref<'a>(iter: impl Iterable<Item = &'a usize>, sum: usize) {
    assert_eq!(iter.iter().sum::<usize>(), sum);
    assert_eq!(iter.iter().sum::<usize>(), sum);
}

fn test_sum_val(iter: impl Iterable<Item = usize>, sum: usize) {
    assert_eq!(iter.iter().sum::<usize>(), sum);
    assert_eq!(iter.iter().sum::<usize>(), sum);
}

#[test]
fn iterable_array() {
    let data = [3, 2, 6, 1, 0, 7];

    test_sum_ref(&data, 19);
    test_sum_val(data.mapped(|x| 2 * x), 2 * 19);
    test_sum_ref(data.filtered(|x| **x < 7), 12);
    test_sum_ref(data.chained(&data), 2 * 19);
    test_sum_ref(data.chained(data.filtered(|x| **x < 7)), 19 + 12);
    test_sum_val(data.zipped(&data).mapped(|(a, b)| a + b), 2 * 19);

    test_sum_val(data.cloned(), 19);
    test_sum_val(data.copied(), 19);
    test_sum_val(data.copied().mapped(|x| 2 * x), 2 * 19);
    test_sum_val(data.copied().filtered(|x| *x < 7), 12);
}

#[test]
fn iterable_slice() {
    let vec = vec![3, 2, 6, 1, 0, 7];

    let data = vec.as_slice();
    test_sum_ref(data, 19);
    test_sum_val(data.mapped(|x| 2 * x), 2 * 19);
    test_sum_ref(data.filtered(|x| **x < 7), 12);
    test_sum_ref(data.chained(data), 2 * 19);
    test_sum_ref(data.chained(data.filtered(|x| **x < 7)), 19 + 12);
    test_sum_val(data.zipped(data).mapped(|(a, b)| a + b), 2 * 19);

    test_sum_val(data.cloned(), 19);
    test_sum_val(data.copied(), 19);
}

#[test]
fn iterable_std_owned_collections() {
    macro_rules! test_std_collection {
        ($V:ty) => {
            let data: $V = [3, 2, 6, 1, 0, 7].into_iter().collect();
            test_sum_ref(&data, 19);
            test_sum_val(data.mapped(|x| 2 * x), 2 * 19);
            test_sum_ref(data.filtered(|x| **x < 7), 12);
            test_sum_ref(data.chained(&data), 2 * 19);
            test_sum_ref(data.chained(data.filtered(|x| **x < 7)), 19 + 12);
            test_sum_val(data.zipped(&data).mapped(|(a, b)| a + b), 2 * 19);

            test_sum_val(data.cloned(), 19);
            test_sum_val(data.copied(), 19);
            test_sum_val(data.copied().mapped(|x| 2 * x), 2 * 19);
            test_sum_val(data.copied().filtered(|x| *x < 7), 12);

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
    test_sum_val(iter().into_iterable().mapped(|x| 2 * x), 2 * 19);
    test_sum_ref(iter().into_iterable().filtered(|x| **x < 7), 12);
    test_sum_ref(
        iter().into_iterable().chained(iter().into_iterable()),
        2 * 19,
    );
    test_sum_ref(
        iter()
            .into_iterable()
            .chained(iter().into_iterable().filtered(|x| **x < 7)),
        19 + 12,
    );
    test_sum_val(
        iter()
            .into_iterable()
            .zipped(iter().into_iterable())
            .mapped(|(a, b)| a + b),
        2 * 19,
    );

    test_sum_val(iter().into_iterable().cloned(), 19);
    test_sum_val(iter().into_iterable().copied(), 19);

    test_sum_val(iter().copied().into_iterable(), 19);
    test_sum_val(iter().cloned().into_iterable(), 19);
    test_sum_val(iter().map(|x| 2 * x).into_iterable(), 2 * 19);
    test_sum_ref(iter().filter(|x| **x < 7).into_iterable(), 12);
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

#[test]
fn iterable_flattened() {
    let data = vec![vec![1, 2], vec![6, 0, 7], vec![3]];
    test_sum_ref(data.flattened(), 19);
    test_sum_val(data.flattened().copied(), 19);
}

#[test]
fn iterable_flat_mapped() {
    let data = vec![2, 4, 3];
    test_sum_val(data.flat_mapped(|&i| 0..i), 10);

    let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
    let indices = vec![0, 2, 3];
    test_sum_ref(indices.flat_mapped(|idx| &data[*idx]), 26);
}

#[test]
fn iterable_mapped_while() {
    let data = vec![2, 4, 12, 3, 8, 4];
    test_sum_ref(data.mapped_while(|x| (x % 2 == 0).then_some(x)), 18);
    test_sum_val(data.mapped_while(|x| (x % 2 == 0).then_some(*x)), 18);
}

#[test]
fn iterable_skipped() {
    let data = vec![2, 4, 12, 3, 8, 4];
    test_sum_ref(data.skipped(0), 33);
    test_sum_ref(data.skipped(2), 27);
    test_sum_ref(data.skipped(20), 0);
}

#[test]
fn iterable_taken() {
    let data = vec![2, 4, 12, 3, 8, 4];
    test_sum_ref(data.taken(0), 0);
    test_sum_ref(data.taken(4), 21);
    test_sum_ref(data.taken(20), 33);
}

#[test]
fn iterable_taken_while() {
    let data = vec![2, 4, 12, 3, 8, 4];
    test_sum_ref(data.taken_while(|x| *x % 2 == 0), 18);
    test_sum_ref(data.taken_while(|x| *x % 2 == 1), 0);
    test_sum_ref(data.taken_while(|x| **x < 100), 33);
}

#[test]
fn iterable_chained() {
    let a = vec![3, 2, 1];
    let b = vec![33, 44];
    let c = vec![100];
    test_sum_ref(a.chained(&b), 83);
    test_sum_ref(a.chained(&b).chained(&c), 183);
}
