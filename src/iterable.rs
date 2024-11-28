use std::ops::Range;

pub trait Iterable {
    type Item;

    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

// impl

impl<'a, X> Iterable for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    type Iter<'b> = <&'a X as IntoIterator>::IntoIter where Self: 'b;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}

// impl - special

impl<'a, T> Iterable for &'a [T] {
    type Item = &'a T;

    type Iter<'b> = std::slice::Iter<'a, T> where Self: 'b;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}

impl Iterable for Range<usize> {
    type Item = usize;

    type Iter<'a> = Range<usize> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.clone()
    }
}

// impl<X> Iterable for X
// where
//     X: IntoIterator,
//     X::Item: Copy,
// {
//     type Item = usize;

//     type Iter<'a> = Range<usize> where Self: 'a;

//     fn iter(&self) -> Self::Iter<'_> {
//         todo!()
//     }
// }
