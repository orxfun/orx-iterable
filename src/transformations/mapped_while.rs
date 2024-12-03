use crate::Iterable;

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

    fn it(&self) -> Self::Iter {
        self.it.it().map_while(self.map_while)
    }
}
