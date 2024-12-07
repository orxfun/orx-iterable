use crate::Iterable;

/// An iterable whose iterators yield elements which are copies of references
/// that a wrapped iterable yields.
pub struct Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    pub(crate) it: I,
}

impl<'a, T, I> Iterable for Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    type Iter = core::iter::Copied<I::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().copied()
    }
}
