use crate::Iterable;

pub struct Enumerated<I>
where
    I: Iterable,
{
    pub(crate) it: I,
}

impl<I> Iterable for Enumerated<I>
where
    I: Iterable,
{
    type Item = (usize, I::Item);

    type Iter = core::iter::Enumerate<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().enumerate()
    }
}
