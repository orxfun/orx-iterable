use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

// iterable

pub struct Flattened<I1, T> {
    it1: I1,
    phantom: PhantomData<T>,
}

impl<I1, T> IterableOnce for Flattened<I1, T>
where
    I1: IterableOnce,
    I1::Item: IterableOnce<Item = T>,
{
    type Item = T;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.it1.it_once().flat_map(|it2| it2.it_once())
    }
}

impl<I1, T> Iterable for Flattened<I1, T>
where
    I1: Iterable,
    I1::Item: Iterable<Item = T>,
{
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        // self.it1.iter().flat_map(|it2| it2.iter())
        std::iter::empty()
    }
}

pub struct FlattenedIter<'a, I1, T>
where
    I1: Iterable,
    I1::Item: Iterable<Item = T>,
{
    it1: &'a I1,
    it2: I1::Item,
}

impl<'a, I1, T> Iterator for FlattenedIter<'a, I1, T>
where
    I1: Iterable,
    I1::Item: Iterable<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn abc() {
    let x = vec![vec![1, 2, 3]];
    let y = x.iter();
    let mut z = y.flatten();

    let w = z.next().unwrap();

    let o = Flattened {
        it1: &x,
        phantom: PhantomData::<&usize>,
    };
    let mut w = o.iter();
    let zz = w.next().unwrap();

    // let o = Flattened {
    //     it1: x,
    //     phantom: PhantomData::<usize>,
    // };
    // let mut w = o.it_once();
    // let zz = w.next().unwrap();
}
