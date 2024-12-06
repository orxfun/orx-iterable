use crate::Iterable;

/// Wraps an `Iterable` and creates a new `Iterable` which maps the elements of
/// the original iterable as long as the map-while condition is satisfied.
pub struct MappedWhile<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U> + Copy,
{
    pub(crate) it: I,
    pub(crate) map_while: M,
}

impl<I, M, U> Iterable for MappedWhile<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U> + Copy,
{
    type Item = U;

    type Iter = core::iter::MapWhile<I::Iter, M>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().map_while(self.map_while)
    }
}
