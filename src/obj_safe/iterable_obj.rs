use std::boxed::Box;

/// An `IterableObj` is any type which can return a new boxed iterator that yields
/// elements of the associated type [`Item`] every time [`boxed_iter`] method is called.
///
/// It is the object safe counterpart of [`Iterable`] trait which can conveniently be made into a trait object.
///
/// Instead of `iter`, it implements `boxed_iter` which returns the same iterator in a box.
///
/// Note that for collections and cloneable iterators, `IterableObj` is implicitly implemented and readily available.
/// Please refer to [`Iterable`] documentation for details of automatic implementations.
///
/// In order to use object safe iterables and collections please add `--features std` and use
/// `use orx_iterable::{*, obj_safe::*}` to import dependencies rather than `use orx_iterable::{*}`.
///
/// [`Item`]: crate::obj_safe::IterableObj::Item
/// [`boxed_iter`]: crate::obj_safe::IterableObj::boxed_iter
/// [`Iterable`]: crate::Iterable
///
/// # Examples
///
/// ```
/// use orx_iterable::{*, obj_safe::*};
/// use arrayvec::ArrayVec;
/// use smallvec::{smallvec, SmallVec};
/// use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};
///
/// struct Stats {
///     count: usize,
///     mean: i64,
///     std_dev: i64,
/// }
///
/// /// we need multiple iterations over numbers to compute the stats
/// fn statistics<'a>(numbers: Box<dyn IterableObj<Item = i64> + 'a>) -> Stats {
///     let count = numbers.boxed_iter().count() as i64;
///     let sum = numbers.boxed_iter().sum::<i64>();
///     let mean = sum / count;
///     let sum_sq_errors: i64 = numbers.boxed_iter().map(|x| (x - mean) * (x - mean)).sum();
///     let std_dev = f64::sqrt(sum_sq_errors as f64 / (count - 1) as f64) as i64;
///     Stats {
///         count: count as usize,
///         mean,
///         std_dev,
///     }
/// }
///
/// // collections as IterableObj
///
/// let x = [3, 5, 7];
/// statistics(Box::new(x.copied()));
/// // see IterableObj's transformation methods such as copied, mapped, etc.
///
/// let x = vec![3, 5, 7];
/// statistics(Box::new(x.copied()));
///
/// let x = LinkedList::from_iter([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// let x = VecDeque::from_iter([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// let x = HashSet::<_>::from_iter([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// let x = BTreeSet::from_iter([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// let x = BinaryHeap::from_iter([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// let x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
/// statistics(Box::new(x.copied()));
///
/// let mut x = ArrayVec::<_, 16>::new();
/// x.extend([3, 5, 7]);
/// statistics(Box::new(x.copied()));
///
/// // cloneable iterators as IterableObj
///
/// let x = Box::new((0..10).map(|x| x * 2).into_iterable());
/// statistics(x);
///
/// let x = vec![1, 2, 3];
/// let y = Box::new(x
///     .iter()
///     .copied()
///     .filter(|x| x % 2 == 1)
///     .flat_map(|x| [-x, x])
///     .into_iterable());
/// statistics(y);
///
/// // lazy generators as IterableObj
///
/// statistics(Box::new(7..21i64));
/// ```
///
/// The following example represents an explicit implementation of the Iterable
/// trait for a lazy generator, which generates a sequence of Fibonacci numbers
/// up to a set bound.
///
/// ```
/// use orx_iterable::*;
/// use orx_iterable::obj_safe::*;
///
/// struct FibUntilIter {
///     curr: u32,
///     next: u32,
///     until: u32,
/// }
///
/// impl Iterator for FibUntilIter {
///     type Item = u32;
///
///     fn next(&mut self) -> Option<Self::Item> {
///         let current = self.curr;
///         self.curr = self.next;
///         self.next = current + self.next;
///         match current > self.until {
///             false => Some(current),
///             true => None,
///         }
///     }
/// }
///
/// struct FibUntil(u32);
///
/// impl IterableObj for FibUntil {
///     type Item = u32;
///
///     fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
///         Box::new(FibUntilIter { curr: 0, next: 1, until: self.0 })
///     }
/// }
///
/// let fib = FibUntil(10); // IterableObj
///
/// assert_eq!(fib.boxed_iter().count(), 7);
/// assert_eq!(fib.boxed_iter().max(), Some(8));
/// assert_eq!(fib.boxed_iter().collect::<Vec<_>>(), [0, 1, 1, 2, 3, 5, 8]);
/// ```
pub trait IterableObj {
    /// Type of the item that the iterators created by the [`boxed_iter`] method yields.
    ///
    /// [`boxed_iter`]: crate::obj_safe::IterableObj::boxed_iter
    type Item;

    /// Creates a new iterator in a box from this iterable yielding elements of type `IterableObj::Item`.
    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_>;
}

// impl

impl<'a, X> IterableObj for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.into_iter())
    }
}

impl<'a, X> IterableObj for &'a [X] {
    type Item = &'a X;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(<[X]>::iter(self))
    }
}
