use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct OwningIterable<'a, X> {
    x: X,
    phantom: PhantomData<&'a ()>,
}

impl<'a, X> OwningIterable<'a, X> {
    pub fn new(x: X) -> Self {
        Self {
            x,
            phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> X {
        self.x
    }
}

impl<'a, X: 'a> IterableOnce for OwningIterable<'a, X>
where
    &'a X: IntoIterator + IterableOnce + 'a,
{
    type Item = <&'a X as IntoIterator>::Item;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        unsafe { into_ref(&self.x) }.into_iter()
    }
}

impl<'a, X: 'a> Iterable for OwningIterable<'a, X>
where
    &'a X: IntoIterator + Iterable + 'a,
{
    type Item = <&'a X as IntoIterator>::Item;

    fn iter(&self) -> impl Iterator<Item = Self::Item> + '_ {
        unsafe { into_ref(&self.x) }.into_iter()
    }
}

unsafe fn into_ref<'a, T>(reference: &T) -> &'a T {
    unsafe { &*(reference as *const T) }
}

// into

pub trait IntoOwningIterable<'a>
where
    &'a Self: IntoIterator + Iterable + 'a,
{
    fn move_into_iterable(self) -> OwningIterable<'a, Self>
    where
        Self: Sized;
}

impl<'a, X> IntoOwningIterable<'a> for X
where
    &'a X: IntoIterator + Iterable + 'a,
{
    fn move_into_iterable(self) -> OwningIterable<'a, Self>
    where
        Self: Sized,
    {
        OwningIterable::new(self)
    }
}
