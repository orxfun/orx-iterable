use crate::Iterable;
use std::marker::PhantomData;

pub struct Copied<'a, T, I>
where
    I: Iterable<'a, Item = &'a T>,
    T: Copy + 'a,
{
    iterable: I,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I> Iterable<'a> for Copied<'a, T, I>
where
    I: Iterable<'a, Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    type Iter = std::iter::Copied<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.iterable.iter().copied()
    }
}

pub trait IntoCopied<'a, T>
where
    Self: Iterable<'a, Item = &'a T>,
    T: Copy + 'a,
{
    fn copied(self) -> Copied<'a, T, Self>
    where
        Self: Sized,
    {
        Copied {
            iterable: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I> IntoCopied<'a, T> for I
where
    I: Iterable<'a, Item = &'a T>,
    T: Copy + 'a,
{
}
