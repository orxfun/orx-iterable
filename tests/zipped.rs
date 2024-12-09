use orx_iterable::{obj_safe::*, *};

fn test_it<'a>(values: Vec<(usize, bool)>, col: impl Iterable<Item = (&'a usize, &'a bool)>) {
    let sum = values.iter().map(|x| x.0).sum::<usize>();
    let num_true = values.iter().filter(|x| x.1).count();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().map(|x| x.0).sum::<usize>(), sum);
    assert_eq!(col.iter().filter(|x| *x.1).count(), num_true);
}

#[test]
fn zipped() {
    let a = vec![1, 2, 3, 4];
    let b = vec![false, true, false];
    let values = vec![(1, false), (2, true), (3, false)];

    test_it(values, a.zipped(&b));
}

// obj

fn obj_test_it<'a>(
    values: Vec<(usize, bool)>,
    col: &dyn IterableObj<Item = (&'a usize, &'a bool)>,
) {
    let sum = values.iter().map(|x| x.0).sum::<usize>();
    let num_true = values.iter().filter(|x| x.1).count();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().map(|x| x.0).sum::<usize>(), sum);
    assert_eq!(col.boxed_iter().filter(|x| *x.1).count(), num_true);
}

#[test]
fn obj_zipped() {
    let a = vec![1, 2, 3, 4];
    let b = vec![false, true, false];
    let values = vec![(1, false), (2, true), (3, false)];

    obj_test_it(values, &a.zipped(&b));
}
