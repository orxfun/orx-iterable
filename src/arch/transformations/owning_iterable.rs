use crate::Iterable;
use std::marker::PhantomData;

pub struct OwningIterable<'a, I>
where
    I: 'a,
    &'a I: Iterable<'a>,
{
    iter: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> OwningIterable<'a, I>
where
    I: 'a,
    &'a I: Iterable<'a>,
{
    pub fn into_inner(self) -> I {
        self.iter
    }
}

impl<'a, I> Iterable<'a> for OwningIterable<'a, I>
where
    I: 'a,
    &'a I: Iterable<'a>,
{
    type Item = <&'a I as Iterable<'a>>::Item;

    type Iter = <&'a I as Iterable<'a>>::Iter;

    fn iter(&self) -> Self::Iter {
        unsafe fn into_ref<'b, U>(reference: &U) -> &'b U {
            unsafe { &*(reference as *const U) }
        }

        let x = unsafe { into_ref(&self.iter) };
        x.iter()
    }
}

pub trait IntoOwningIterable<'a>
where
    Self: Sized + 'a,
    &'a Self: Iterable<'a>,
{
    fn move_into_iterable(self) -> OwningIterable<'a, Self> {
        OwningIterable {
            iter: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoOwningIterable<'a> for I
where
    I: 'a,
    &'a I: Iterable<'a>,
{
}
