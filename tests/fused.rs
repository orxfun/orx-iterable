use orx_iterable::*;

// TODO: does not really test fused behavior, but only makes sure that it is callable.
fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn fused() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.fused().copied());
}

#[test]
fn fused_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.fused_mut().iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.fused().copied());
}

#[test]
fn into_fused() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_fused();
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());
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
    fn obj_fused() {
        let a = vec![1, 3, 7, 2, 8];
        obj_test_it(vec![1, 3, 7, 2, 8], &a.fused().copied());
    }

    #[test]
    fn obj_fused_mut() {
        let mut a = vec![1, 3, 7, 2, 8];
        for x in a.fused_mut().iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![11, 13, 17, 12, 18], &a.fused().copied());
    }

    #[test]
    fn obj_into_fused() {
        let a = vec![1, 3, 7, 2, 8];
        let mut a = a.into_fused();
        for x in a.iter_mut() {
            *x += 10;
        }
        obj_test_it(vec![11, 13, 17, 12, 18], &a.copied());
    }
}
