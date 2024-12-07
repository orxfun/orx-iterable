use crate::Iterable;

/// Wraps an `Iterable` and creates a new `Iterable` which maps the elements of
/// the original iterable.
pub struct Mapped<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> U + Copy,
{
    pub(crate) it: I,
    pub(crate) map: M,
}

impl<I, M, U> Iterable for Mapped<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> U + Copy,
{
    type Item = U;

    type Iter = core::iter::Map<I::Iter, M>;

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().map(self.map)
    }
}
