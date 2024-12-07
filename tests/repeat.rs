use orx_iterable::*;

#[test]
fn repeat() {
    let it = orx_iterable::repeat(42).taken(3);

    assert_eq!(it.iterate().count(), 3);
    assert_eq!(it.iterate().sum::<usize>(), 3 * 42);

    let it = core::iter::repeat(42).taken(3);

    assert_eq!(it.iterate().count(), 3);
    assert_eq!(it.iterate().sum::<usize>(), 3 * 42);
}
