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
