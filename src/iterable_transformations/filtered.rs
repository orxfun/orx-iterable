use crate::Iterable;

// iterable

pub struct Filtered<I, F> {
    iterable: I,
    filter: F,
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

impl<I, F> Iterable for Filtered<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter<'a> = FilteredIter<'a, I, F> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        FilteredIter {
            iter: self.iterable.iter(),
            filter: &self.filter,
        }
    }
}

// iter

pub struct FilteredIter<'a, I, F>
where
    I: Iterable + 'a,
    F: Fn(&I::Item) -> bool,
{
    iter: I::Iter<'a>,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredIter<'a, I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

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

pub trait IntoFiltered
where
    Self: Sized + Iterable,
{
    fn filtered<F>(self, filter: F) -> Filtered<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFiltered for I where I: Iterable {}
