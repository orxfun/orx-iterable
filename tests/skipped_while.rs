use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn skipped_while() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.skipped_while(|x| **x > 100).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![7, 2, 8], a.skipped_while(|x| **x < 5).copied());

    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![], a.skipped_while(|x| **x < 10).copied());
}

#[test]
fn skipped_while_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.skipped_while_mut(|x| *x > 100).iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.skipped_while_mut(|x| *x < 5).iter_mut() {
        *x += 10;
    }
    test_it(vec![1, 3, 17, 12, 18], a.copied());

    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.skipped_while_mut(|x| *x < 100).iter_mut() {
        *x += 10;
    }
    test_it(vec![1, 3, 7, 2, 8], a.copied());
}

#[test]
fn into_skipped_while() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_skipped_while(|x| *x > 100);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_skipped_while(|x| *x < 5);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![17, 12, 18], a.copied());

    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_skipped_while(|x| *x < 100);
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![], a.copied());
}

#[cfg(feature = "std")]
mod object_safe {
    use orx_iterable::{obj_safe::*, *};

    fn obj_test_it(values: Vec<usize>, col: &dyn IterableObj<Item = usize>) {
        let sum = values.iter().sum::<usize>();
        let count = values.len();

        // tests
        assert_eq!(col.boxed_iter().count(), count);
        assert_eq!(col.boxed_iter().sum::<usize>(), sum);
    }

    #[test]
    fn obj_skipped_while() {
        let a = vec![1, 3, 7, 2, 8];
        obj_test_it(
            vec![1, 3, 7, 2, 8],
            &a.skipped_while(|x| **x > 100).copied(),
        );

        let a = vec![1, 3, 7, 2, 8];
        obj_test_it(vec![7, 2, 8], &a.skipped_while(|x| **x < 5).copied());

        let a = vec![1, 3, 7, 2, 8];
        obj_test_it(vec![], &a.skipped_while(|x| **x < 10).copied());
    }

    #[test]
    fn obj_skipped_while_mut() {
        let mut a = vec![1, 3, 7, 2, 8];
        for x in a.skipped_while_mut(|x| *x > 100).iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![11, 13, 17, 12, 18], &a.copied());

        let mut a = vec![1, 3, 7, 2, 8];
        for x in a.skipped_while_mut(|x| *x < 5).iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![1, 3, 17, 12, 18], &a.copied());

        let mut a = vec![1, 3, 7, 2, 8];
        for x in a.skipped_while_mut(|x| *x < 100).iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![1, 3, 7, 2, 8], &a.copied());
    }

    #[test]
    fn obj_into_skipped_while() {
        let a = vec![1, 3, 7, 2, 8];
        let mut a = a.into_skipped_while(|x| *x > 100);
        for x in a.iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![11, 13, 17, 12, 18], &a.copied());

        let a = vec![1, 3, 7, 2, 8];
        let mut a = a.into_skipped_while(|x| *x < 5);
        for x in a.iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![17, 12, 18], &a.copied());

        let a = vec![1, 3, 7, 2, 8];
        let mut a = a.into_skipped_while(|x| *x < 100);
        for x in a.iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![], &a.copied());
    }
}
