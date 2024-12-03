use crate::Iterable;

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

    fn iter(&self) -> Self::Iter {
        self.it.iter().map(self.map)
    }
}