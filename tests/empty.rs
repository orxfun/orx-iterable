use orx_iterable::*;

#[test]
fn empty() {
    let it = orx_iterable::empty::<usize>();

    assert_eq!(it.iter().count(), 0);
    assert_eq!(it.iter().sum::<usize>(), 0);

    let it = core::iter::empty::<usize>();

    assert_eq!(it.iter().count(), 0);
    assert_eq!(it.iter().sum::<usize>(), 0);
}

#[test]
fn empty_col() {
    let mut it = orx_iterable::empty_col::<usize>();

    assert_eq!(it.iter().count(), 0);
    assert_eq!(it.iter().sum::<usize>(), 0);

    for x in it.iter_mut() {
        *x += 10;
    }

    assert_eq!(it.iter().count(), 0);
    assert_eq!(it.iter().sum::<usize>(), 0);
}

// obj

#[cfg(feature = "std")]
#[test]
fn obj_empty() {
    use orx_iterable::obj_safe::*;

    let it = orx_iterable::empty::<usize>();

    assert_eq!(it.boxed_iter().count(), 0);
    assert_eq!(it.boxed_iter().sum::<usize>(), 0);

    let it = core::iter::empty::<usize>();

    assert_eq!(it.boxed_iter().count(), 0);
    assert_eq!(it.boxed_iter().sum::<usize>(), 0);
}

#[cfg(feature = "std")]
#[test]
fn obj_empty_col() {
    use orx_iterable::obj_safe::*;

    let mut it = orx_iterable::empty_col::<usize>();

    assert_eq!(it.boxed_iter().count(), 0);
    assert_eq!(it.boxed_iter().sum::<usize>(), 0);

    for x in it.iter_mut() {
        *x += 10;
    }

    assert_eq!(it.boxed_iter().count(), 0);
    assert_eq!(it.boxed_iter().sum::<usize>(), 0);
}
