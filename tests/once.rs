use orx_iterable::*;

#[test]
fn once() {
    let it = orx_iterable::once::<usize>(42);

    assert_eq!(it.iter().count(), 1);
    assert_eq!(it.iter().sum::<usize>(), 42);

    let it = core::iter::once::<usize>(42);

    assert_eq!(it.iter().count(), 1);
    assert_eq!(it.iter().sum::<usize>(), 42);
}

#[test]
fn once_col() {
    let mut it = orx_iterable::once_col::<usize>(42);

    assert_eq!(it.iter().count(), 1);
    assert_eq!(it.iter().sum::<usize>(), 42);

    for x in it.iter_mut() {
        *x += 10;
    }

    assert_eq!(it.iter().count(), 1);
    assert_eq!(it.iter().sum::<usize>(), 52);
}

#[cfg(feature = "std")]
mod object_safe {
    use orx_iterable::{obj_safe::*, *};

    #[test]
    fn obj_once() {
        let it = orx_iterable::once::<usize>(42);

        assert_eq!(it.boxed_iter().count(), 1);
        assert_eq!(it.boxed_iter().sum::<usize>(), 42);

        let it = core::iter::once::<usize>(42);

        assert_eq!(it.boxed_iter().count(), 1);
        assert_eq!(it.boxed_iter().sum::<usize>(), 42);
    }

    #[test]
    fn obj_once_col() {
        let mut it = orx_iterable::once_col::<usize>(42);

        assert_eq!(it.boxed_iter().count(), 1);
        assert_eq!(it.boxed_iter().sum::<usize>(), 42);

        for x in it.iter_mut() {
            *x += 10;
        }

        assert_eq!(it.boxed_iter().count(), 1);
        assert_eq!(it.boxed_iter().sum::<usize>(), 52);
    }
}
