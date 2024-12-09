use crate::Iterable;

/// An iterable created from an [`Iterator`] which can be [`Clone`]d.
///
/// Every time the `iter()` method of the iterable is called, it simply returns a clone
/// of the wrapped iterator, allowing for multiple iterations over the data.
///
/// # Example
///
/// ```
/// use orx_iterable::*;
///
/// let numbers = vec![1, 10, 7, 6, 3, 8, 2];
///
/// // evens is an `Iterator` which can be iterated over only once
/// let evens = numbers.iter().filter(|x| *x % 2 == 0);
///
/// // evens below is an `Iterable` which can be iterated over many times
/// let evens = numbers.iter().filter(|x| *x % 2 == 0).into_iterable();
///
/// assert_eq!(4, evens.iter().count());
/// assert_eq!(26, evens.iter().sum());
/// assert_eq!(Some(&2), evens.iter().min());
/// assert_eq!(Some(&10), evens.iter().max());
/// ```
pub struct CloningIterable<I>(I)
where
    I: Iterator + Clone;

impl<I> Iterable for CloningIterable<I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;

    type Iter = I;

    fn iter(&self) -> Self::Iter {
        self.0.clone()
    }
}

/// Trait to transform types implementing `Iterator + Clone` into an `Iterable`.
///
/// Resulting iterable is of type [`CloningIterable`].
pub trait IntoCloningIterable: Iterator + Clone {
    /// Transforms this type implementing `Iterator + Clone` into an `Iterable`.
    ///
    /// Resulting iterable is of type [`CloningIterable`].
    fn into_iterable(self) -> CloningIterable<Self> {
        CloningIterable(self)
    }
}

impl<I> IntoCloningIterable for I where I: Iterator + Clone {}
