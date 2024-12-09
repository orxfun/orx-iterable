use orx_iterable::{obj_safe::*, *};

fn test_it(mut values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);

    values.reverse();
    assert_eq!(values, col.iter().collect::<Vec<_>>());
}

#[test]
fn reversed() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.reversed().copied());
}

#[test]
fn reversed_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.reversed_mut().iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.reversed().copied());
}

#[test]
fn into_reversed() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_reversed();
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());
}

// obj

fn obj_test_it(mut values: Vec<usize>, col: &dyn IterableObj<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().sum::<usize>(), sum);

    values.reverse();
    assert_eq!(values, col.boxed_iter().collect::<Vec<_>>());
}

#[test]
fn obj_reversed() {
    let a = vec![1, 3, 7, 2, 8];
    obj_test_it(vec![1, 3, 7, 2, 8], &a.reversed().copied());
}

#[test]
fn obj_reversed_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.reversed_mut().iter_mut() {
        *x += 10;
    }
    obj_test_it(vec![11, 13, 17, 12, 18], &a.reversed().copied());
}

#[test]
fn obj_into_reversed() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_reversed();
    for x in a.iter_mut() {
        *x += 10;
    }
    obj_test_it(vec![11, 13, 17, 12, 18], &a.copied());
}
