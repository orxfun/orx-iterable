use crate::Iterable;

/// Wraps an `Iterable` and creates a new `Iterable` which yields the element indices
/// together with the elements.
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

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().enumerate()
    }
}
