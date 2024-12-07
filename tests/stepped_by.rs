use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().sum::<usize>(), sum);
}

#[test]
fn stepped_by() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.stepped_by(1).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 7, 8], a.stepped_by(2).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1], a.stepped_by(10).copied());
}

#[test]
fn stepped_by_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.stepped_by_mut(1).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.stepped_by_mut(2).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 3, 17, 2, 18], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.stepped_by_mut(10).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 3, 7, 2, 8], a.copied());
}

#[test]
fn into_stepped_by() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_stepped_by(1);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_stepped_by(2);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 17, 18], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_stepped_by(10);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11], a.copied());
}
