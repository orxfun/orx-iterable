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

macro_rules! impl_for_range {
    ($T:ty) => {
        impl Iterable for Range<$T> {
            type Item = $T;

            type Iter<'a> = Range<$T> where Self: 'a;

            fn iter(&self) -> Self::Iter<'_> {
                self.clone()
            }
        }
    };
}

impl_for_range!(usize);
impl_for_range!(u8);
impl_for_range!(u16);
impl_for_range!(u32);
impl_for_range!(u64);
impl_for_range!(u128);
impl_for_range!(i8);
impl_for_range!(i16);
impl_for_range!(i32);
impl_for_range!(i64);
impl_for_range!(i128);
