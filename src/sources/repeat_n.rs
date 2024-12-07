use crate::Iterable;

/// An iterable which yields the same value `n` times.
pub struct RepeatN<T>
where
    T: Clone,
{
    value: T,
    count: usize,
}

impl<T> Iterable for RepeatN<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::RepeatN<T>;

    fn iter(&self) -> Self::Iter {
        core::iter::repeat_n(self.value.clone(), self.count)
    }
}

impl<T> Iterable for core::iter::RepeatN<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::RepeatN<T>;

    fn iter(&self) -> Self::Iter {
        self.clone()
    }
}

/// Creates an iterable which yields the same value `n` times.
pub fn repeat_n<T>(value: T, count: usize) -> RepeatN<T>
where
    T: Clone,
{
    RepeatN { value, count }
}
