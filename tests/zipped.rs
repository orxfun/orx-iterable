use orx_iterable::*;

fn test_it<'a>(values: Vec<(usize, bool)>, col: impl Iterable<Item = (&'a usize, &'a bool)>) {
    let sum = values.iter().map(|x| x.0).sum::<usize>();
    let num_true = values.iter().filter(|x| x.1).count();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().map(|x| x.0).sum::<usize>(), sum);
    assert_eq!(col.iterate().filter(|x| *x.1).count(), num_true);
}

#[test]
fn zipped() {
    let a = vec![1, 2, 3, 4];
    let b = vec![false, true, false];
    let values = vec![(1, false), (2, true), (3, false)];

    test_it(values, a.zipped(&b));
}
