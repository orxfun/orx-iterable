use crate::{Iterable, IterableMut, IterableOnce};

pub struct Chained<I1, I2> {
    it1: I1,
    it2: I2,
}

impl<I1, I2> IterableOnce for Chained<I1, I2>
where
    I1: IterableOnce,
    I2: IterableOnce<Item = I1::Item>,
{
    type Item = I1::Item;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.it1.it_once().chain(self.it2.it_once())
    }
}

impl<I1, I2> Iterable for Chained<I1, I2>
where
    I1: Iterable,
    I2: Iterable<Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter<'a> = std::iter::Chain<I1::Iter<'a>, I2::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.iter().chain(self.it2.iter())
    }
}

// into

pub trait IntoChained
where
    Self: Iterable + Sized,
{
    fn chained<I>(self, other: I) -> Chained<Self, I> {
        Chained {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChained for I where I: Iterable {}

#[test]
fn abc() {
    let mut a = vec![1, 2];
    let mut b = vec![3];
    let c = a.iter_mut().chain(b.iter_mut());
    for x in c {
        *x += 1;
    }

    assert_eq!(a, vec![2, 3]);
    assert_eq!(b, vec![4]);
}
