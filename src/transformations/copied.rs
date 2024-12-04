use crate::Iterable;

pub struct Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    it: I,
}

impl<'a, T, I> Iterable for Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    type Iter = core::iter::Copied<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().copied()
    }
}

pub trait IntoCopied<'a, T>
where
    Self: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    fn copied(self) -> Copied<'a, T, Self>
    where
        Self: Sized,
    {
        Copied { it: self }
    }
}

impl<'a, T, I> IntoCopied<'a, T> for I
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
}
