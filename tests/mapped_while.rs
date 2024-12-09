use orx_iterable::{obj_safe::*, *};

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn mapped() {
    test_it(
        vec![2, 6, 14],
        vec![1, 3, 7, 4, 9].mapped_while(|x| (*x % 2 == 1).then_some(*x as usize * 2)),
    );

    test_it(
        vec![],
        vec![1, 3, 7, 4, 9].mapped_while(|x| (*x % 2 == 0).then_some(*x as usize * 2)),
    );

    test_it(
        vec![2, 6, 14, 8, 18],
        vec![1, 3, 7, 4, 9].mapped_while(|x| (*x < 10).then_some(*x as usize * 2)),
    );
}

// obj

fn obj_test_it(values: Vec<usize>, col: &dyn IterableObj<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.boxed_iter().count(), count);
    assert_eq!(col.boxed_iter().sum::<usize>(), sum);
}

#[test]
fn obj_mapped() {
    obj_test_it(
        vec![2, 6, 14],
        &vec![1, 3, 7, 4, 9].mapped_while(|x| (*x % 2 == 1).then_some(*x as usize * 2)),
    );

    obj_test_it(
        vec![],
        &vec![1, 3, 7, 4, 9].mapped_while(|x| (*x % 2 == 0).then_some(*x as usize * 2)),
    );

    obj_test_it(
        vec![2, 6, 14, 8, 18],
        &vec![1, 3, 7, 4, 9].mapped_while(|x| (*x < 10).then_some(*x as usize * 2)),
    );
}
