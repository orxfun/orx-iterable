use crate::obj_safe::collection_obj::CollectionObj;
use std::boxed::Box;

/// In addition to  [`boxed_iter`], a `CollectionMutObj` provides the [`boxed_iter_mut`] method which returns a boxed
/// iterator over mutable references of elements of the collection.
///
/// It is the object safe counterpart of [`CollectionMut`] trait which can conveniently be made into a trait object.
///
/// Note that for collections, `CollectionMutObj` is implicitly implemented and readily available.
/// Please refer to [`CollectionMut`] documentation for details of automatic implementations.
///
/// In order to use object safe iterables and collections please add `--features std` and use
/// `use orx_iterable::{*, obj_safe::*}` to import dependencies rather than `use orx_iterable::*`.
///
/// [`Iterable`]: crate::Iterable
/// [`Item`]: crate::obj_safe::CollectionMutObj::Item
/// [`boxed_iter`]: crate::obj_safe::CollectionObj::boxed_iter
/// [`boxed_iter_mut`]: crate::obj_safe::CollectionMutObj::boxed_iter_mut
/// [`CollectionMut`]: crate::CollectionMut
///
/// # Examples
///
/// ```
/// use orx_iterable::obj_safe::*;
/// use arrayvec::ArrayVec;
/// use smallvec::{smallvec, SmallVec};
/// use std::collections::{LinkedList, VecDeque};
///
/// /// first computes sum, and then adds it to each of the elements
/// fn increment_by_sum(numbers: &mut dyn CollectionMutObj<Item = i32>) {
///     let sum: i32 = numbers.boxed_iter().sum();
///
///     for x in numbers.boxed_iter_mut() {
///         *x += sum;
///     }
/// }
///
/// // example collections that automatically implement CollectionMut
///
/// let mut x = [1, 2, 3];
/// increment_by_sum(&mut x);
/// assert_eq!(x, [7, 8, 9]);
///
/// let mut x = vec![1, 2, 3];
/// increment_by_sum(&mut x);
///
/// let mut x = LinkedList::from_iter([1, 2, 3]);
/// increment_by_sum(&mut x);
///
/// let mut x = VecDeque::from_iter([1, 2, 3]);
/// increment_by_sum(&mut x);
///
/// let mut x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
/// increment_by_sum(&mut x);
///
/// let mut x = ArrayVec::<_, 16>::new();
/// x.extend([3, 5, 7]);
/// increment_by_sum(&mut x);
/// ```
pub trait CollectionMutObj: CollectionObj {
    /// Creates a new iterator in a box yielding mutable references to the elements of the collection; i.e.,
    /// type of elements is `&mut Item`.
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_>;
}

impl<X> CollectionMutObj for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(<&mut X as IntoIterator>::into_iter(self))
    }
}
