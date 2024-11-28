use crate::Iterable;

pub struct Taken<I> {
    take: usize,
    iterable: I,
}

impl<I> Iterable for Taken<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter<'a> = core::iter::Take<I::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().take(self.take)
    }
}

// into

pub trait IntoTaken
where
    Self: Iterable,
{
    fn taken(self, num_items_to_take: usize) -> Taken<Self>
    where
        Self: Sized,
    {
        Taken {
            iterable: self,
            take: num_items_to_take,
        }
    }
}

impl<I> IntoTaken for I where I: Iterable {}
