use crate::{Collection, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

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

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().step_by(self.step)
    }
}

// col

/// Wraps an `Collection` and creates a new `Collection` which yields elements of
/// the original iterable by stepping by a given step size.
pub struct SteppedByCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    pub(crate) it: E,
    pub(crate) step: usize,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a SteppedByCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::StepBy<<I::Iterable<'a> as Iterable>::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.it.get_ref().iter().step_by(self.step)
    }
}

impl<I, E> Collection for SteppedByCol<I, E>
where
    I: Collection,
    E: SoM<I>,
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
