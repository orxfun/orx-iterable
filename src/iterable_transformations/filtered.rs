use crate::Iterable;
use std::marker::PhantomData;

// iterable

pub struct Filtered<I, T, F>
where
    F: Fn(&T) -> bool,
{
    iterable: I,
    filter: F,
    phantom: PhantomData<T>,
}

// impl<I, T, F> IterableOnce for Filtered<I, T, F>
// where
//     F: Fn(&T) -> bool,
//     I: IterableOnce<Item = T>,
// {
//     type Item = T;

//     fn it_once(self) -> impl Iterator<Item = Self::Item> {
//         self.iterable.it_once().filter(self.filter)
//     }
// }

impl<I, T, F> Iterable for Filtered<I, T, F>
where
    F: Fn(&T) -> bool,
    I: Iterable<Item = T>,
{
    type Item = T;

    type Iter<'a> = FilteredIter<'a, I, T, F> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        FilteredIter {
            iter: self.iterable.iter(),
            filter: &self.filter,
        }
    }
}

// iter

pub struct FilteredIter<'a, I, T, F>
where
    F: Fn(&T) -> bool,
    I: Iterable<Item = T> + 'a,
{
    iter: I::Iter<'a>,
    filter: &'a F,
}

impl<'a, I, T, F> Iterator for FilteredIter<'a, I, T, F>
where
    F: Fn(&T) -> bool,
    I: Iterable<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => match (self.filter)(&x) {
                true => Some(x),
                false => self.next(),
            },
            None => None,
        }
    }
}

// into

pub trait IntoFiltered<T>
where
    Self: Iterable<Item = T>,
{
    fn filtered<F>(self, filter: F) -> Filtered<Self, T, F>
    where
        F: Fn(&T) -> bool,
        Self: Sized,
    {
        Filtered {
            iterable: self,
            filter,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoFiltered<T> for I where I: Iterable<Item = T> {}