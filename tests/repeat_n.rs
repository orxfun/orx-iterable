use orx_iterable::*;

#[test]
fn repeat_n() {
    let it = orx_iterable::repeat_n(42, 3);

    assert_eq!(it.iterate().count(), 3);
    assert_eq!(it.iterate().sum::<usize>(), 3 * 42);

    let it = core::iter::repeat_n(42, 3);

    assert_eq!(it.iterate().count(), 3);
    assert_eq!(it.iterate().sum::<usize>(), 3 * 42);
}