use crate::{Iterable, IterableCol};
use core::marker::PhantomData;
use orx_exclusive::Exclusive;

/// Wraps an `Iterable` and creates a new `Iterable` which yields elements of
/// the original iterable by stepping by a given step size.
pub struct SteppedBy<I>
where
    I: Iterable,
{
    pub(crate) it: I,
    pub(crate) step: usize,
}

impl<I> Iterable for SteppedBy<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter = core::iter::StepBy<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().step_by(self.step)
    }
}

// col

/// Wraps an `IterableCol` and creates a new `IterableCol` which yields elements of
/// the original iterable by stepping by a given step size.
pub struct SteppedByCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    pub(crate) it: E,
    pub(crate) step: usize,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a SteppedByCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::StepBy<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().step_by(self.step)
    }
}

impl<I, E> IterableCol for SteppedByCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::StepBy<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().step_by(self.step)
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
