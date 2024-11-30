use crate::Iterable;

pub struct Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    it: I,
}

impl<'a, T, I> Iterable for Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type Iter = std::iter::Cloned<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().cloned()
    }
}

pub trait IntoCloned<'a, T>
where
    Self: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    fn cloned(self) -> Cloned<'a, T, Self>
    where
        Self: Sized,
    {
        Cloned { it: self }
    }
}

impl<'a, T, I> IntoCloned<'a, T> for I
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
}
