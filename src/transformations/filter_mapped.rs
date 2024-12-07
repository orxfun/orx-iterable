use crate::Iterable;

/// Wraps an `Iterable` and creates a new `Iterable` which filters-and-maps the elements
/// of the original iterable.
pub struct FilterMapped<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U> + Copy,
{
    pub(crate) it: I,
    pub(crate) filter_map: M,
}

impl<I, M, U> Iterable for FilterMapped<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U> + Copy,
{
    type Item = U;

    type Iter = core::iter::FilterMap<I::Iter, M>;

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().filter_map(self.filter_map)
    }
}
