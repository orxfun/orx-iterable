use crate::{IterableMut, IterableRef};

pub trait Iterable {
    type ItemVal;

    type IterVal: Iterator<Item = Self::ItemVal>;

    fn iter_val(self) -> Self::IterVal;
}

pub struct Copied<'a, I>
where
    I: IterableRef,
    I::Item: Copy,
{
    iterable: &'a I,
}

impl<'a, I> Iterable for Copied<'a, I>
where
    I: IterableRef,
    I::Item: Copy,
{
    type ItemVal = I::Item;

    type IterVal = core::iter::Copied<<I as IterableRef>::Iter<'a>>;

    fn iter_val(self) -> Self::IterVal {
        self.iterable.iter().copied()
    }
}

pub struct Mapped<'a, I, M, U>
where
    I: IterableRef,
    M: Fn(&'a I::Item) -> U,
{
    iterable: &'a I,
    map: M,
}

impl<'a, I, M, U> Iterable for Mapped<'a, I, M, U>
where
    I: IterableRef,
    M: Fn(&'a I::Item) -> U,
{
    type ItemVal = U;

    type IterVal = core::iter::Map<<I as IterableRef>::Iter<'a>, M>;

    fn iter_val(self) -> Self::IterVal {
        todo!()
    }
}

impl<'a, X> Iterable for &'a X
where
    X: IterableRef,
{
    type ItemVal = &'a X::Item;

    type IterVal = X::Iter<'a>;

    fn iter_val(self) -> Self::IterVal {
        self.iter()
    }
}

fn abc() {
    use crate::*;

    let vec = vec![1.to_string(), 3.to_string(), 2.to_string()];

    let x = Mapped {
        iterable: &vec,
        map: |x: &String| x.len(),
    };

    let y = x.iter_val().next().unwrap();

    let x = Mapped {
        iterable: &vec,
        map: |x: &String| x.as_str(),
    };

    let y = x.iter_val().next().unwrap();
}

// AAAAAAAAAA

pub trait ItMut<'a> {
    type ItItemMut;

    type ItIterMut: Iterator<Item = Self::ItItemMut>;

    fn it_mut(self) -> Self::ItIterMut;
}

impl<'a, T> ItMut<'a> for &'a mut Vec<T> {
    type ItItemMut = &'a mut T;

    type ItIterMut = core::slice::IterMut<'a, T>;

    fn it_mut(self) -> Self::ItIterMut {
        self.as_mut_slice().iter_mut()
    }
}

fn xyz() {
    let mut v = vec![1, 3, 4];
    let x = v.it_mut();
    let x = v.it_mut();
    let x = v.it_mut();
    let x = v.it_mut();
    let x = v.it_mut();
}
