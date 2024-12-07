use crate::Iterable;

/// An iterable which yields the same value infinitely many times.
pub struct Repeat<T>
where
    T: Clone,
{
    value: T,
}

impl<T> Iterable for Repeat<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::Repeat<T>;

    fn iter(&self) -> Self::Iter {
        core::iter::repeat(self.value.clone())
    }
}

impl<T> Iterable for core::iter::Repeat<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::Repeat<T>;

    fn iter(&self) -> Self::Iter {
        self.clone()
    }
}

/// Creates an iterable which yields the same value infinitely many times.
pub fn repeat<T>(value: T) -> Repeat<T>
where
    T: Clone,
{
    Repeat { value }
}
