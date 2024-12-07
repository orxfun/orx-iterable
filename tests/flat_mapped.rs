use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().sum::<usize>(), sum);
}

#[test]
fn flat_mapped() {
    test_it(
        vec![1, 4, 7, 11, 3, 8],
        vec![vec![0, 3], vec![6], vec![10, 2, 7]].flat_mapped(|x| x.iter().map(|x| x + 1)),
    );

    let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
    let indices = vec![0, 2, 3];
    assert_eq!(
        indices
            .flat_mapped(|idx| &data[*idx])
            .iterate()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 4, 2, 8, 8, 3]
    );
    assert_eq!(
        indices
            .flat_mapped(|idx| &data[*idx])
            .copied()
            .iterate()
            .collect::<Vec<_>>(),
        vec![1, 4, 2, 8, 8, 3]
    );
}
