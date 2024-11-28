use crate::Iterable;

pub struct Skipped<I> {
    skip: usize,
    iterable: I,
}

impl<I> Iterable for Skipped<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter<'a> = core::iter::Skip<I::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().skip(self.skip)
    }
}

// into

pub trait IntoSkipped
where
    Self: Iterable,
{
    fn skipped(self, num_items_to_skip: usize) -> Skipped<Self>
    where
        Self: Sized,
    {
        Skipped {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkipped for I where I: Iterable {}
