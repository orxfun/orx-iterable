use crate::Iterable;

/// An iterable whose iterators yield elements which are clones of references
/// that a wrapped iterable yields.
pub struct Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    pub(crate) it: I,
}

impl<'a, T, I> Iterable for Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type Iter = core::iter::Cloned<I::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().cloned()
    }
}
