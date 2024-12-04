use orx_iterable::*;

fn test_it<'a>(values: Vec<(usize, usize)>, col: impl Iterable<Item = (usize, &'a usize)>) {
    let sum_values = values.iter().map(|x| x.1).sum::<usize>();
    let sum_indices = values.iter().map(|x| x.0).sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().map(|x| x.0).sum::<usize>(), sum_indices);
    assert_eq!(col.iter().map(|x| x.1).sum::<usize>(), sum_values);
}

#[test]
fn enumerated() {
    let a = vec![1, 2, 3, 4];
    test_it(
        a.clone().into_iter().enumerate().collect::<Vec<_>>(),
        a.enumerated(),
    );
}
