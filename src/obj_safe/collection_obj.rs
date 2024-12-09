use std::boxed::Box;

/// A collection providing the [`boxed_iter`] method which returns a boxed iterator over shared references
/// of elements of the collection.
///
/// It is the object safe variant of [`Collection`] trait which can conveniently be made a trait object.
///
/// [`boxed_iter`]: crate::CollectionObj::boxed_iter
/// [`Collection`]: orx_iterable::Collection
///
/// # Auto Implementations
///
/// Consider a collection type `X` storing elements of type `T`. Provided that the following implementations are provided:
///
/// * `X: IntoIterator<Item = T>`
/// * `&X: IntoIterator<Item = &T>`
///
/// Then, `X` implements `CollectionObj<Item = T>`.
/// Further, `&X` implements `IterableObj<Item = &T>`.
///
/// # Examples
///
/// ```
/// use orx_iterable::obj_safe::*;
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
/// fn statistics(numbers: &dyn CollectionObj<Item = i64>) -> Stats {
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
/// // example collections that automatically implement Collection
///
/// statistics(&[3, 5, 7]);
/// statistics(&vec![3, 5, 7]);
/// statistics(&LinkedList::from_iter([3, 5, 7]));
/// statistics(&VecDeque::from_iter([3, 5, 7]));
/// statistics(&HashSet::<_>::from_iter([3, 5, 7]));
/// statistics(&BTreeSet::<_>::from_iter([3, 5, 7]));
/// statistics(&BinaryHeap::<_>::from_iter([3, 5, 7]));
///
/// let x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
/// statistics(&x);
///
/// let mut x = ArrayVec::<_, 16>::new();
/// x.extend([3, 5, 7]);
/// statistics(&x);
/// ```
pub trait CollectionObj {
    /// Type of elements stored by the collection.
    type Item;

    /// Creates a new iterator in a box yielding references to the elements of the collection; i.e.,
    /// type of elements is `&Item`.
    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
}

impl<X> CollectionObj for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(<&X as IntoIterator>::into_iter(self))
    }
}
