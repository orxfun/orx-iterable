use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.it().count(), count);
    assert_eq!(col.it().sum::<usize>(), sum);
}

#[test]
fn taken() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![], a.taken(0).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3], a.taken(2).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.taken(5).copied());
}

#[test]
fn taken_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.taken_mut(0).iter_mut() {
        *x += 10;
    }
    test_it(vec![1, 3, 7, 2, 8], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.taken_mut(2).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 7, 2, 8], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.taken_mut(10).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());
}

#[test]
fn into_taken() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_taken(0);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_taken(2);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_taken(10);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());
}
