use crate::Iterable;
use std::marker::PhantomData;

pub struct OwningIterable<'a, I, T>
where
    I: IntoIterator<Item = T>,
    for<'b> &'b I: IntoIterator<Item = &'b <I as IntoIterator>::Item>,
{
    iter: I,
    phantom: PhantomData<&'a T>,
}

// impl<'a, I> OwningIterable<'a, I>
// where
//     I: IntoIterator + 'a,
//     for<'b> &'b I: IntoIterator<Item = &'b <I as IntoIterator>::Item>,
// {
//     pub fn into_inner(self) -> I {
//         self.iter
//     }
// }

// impl<'a, I> Iterable for OwningIterable<'a, I>
// where
//     I: IntoIterator + 'a,
//     for<'b> &'b I: IntoIterator<Item = &'b <I as IntoIterator>::Item>,
// {
//     type Item = <&'a I as IntoIterator>::Item;

//     type Iter<'b> = <&'b I as IntoIterator>::IntoIter where Self: 'b;

//     fn iter(&self) -> Self::Iter<'_> {
//         let x = &self.iter;
//         x.into_iter()
//     }
// }

pub trait IntoOwningIterable
where
    Self: Iterable + Sized,
{
    // fn move_into_iterable(self) -> OwningIterable<Self> {
    //     OwningIterable(self)
    // }
}

impl<I: Iterable> IntoOwningIterable for I {}
