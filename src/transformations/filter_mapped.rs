use crate::Iterable;

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

    fn iter(&self) -> Self::Iter {
        self.it.iter().filter_map(self.filter_map)
    }
}
