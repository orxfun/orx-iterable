use crate::{
    transformations::{
        ChainedCol, FilteredCol, FlattenedCol, FusedCol, ReversedCol, SkippedCol, SkippedWhileCol,
        SteppedByCol, TakenCol, TakenWhileCol,
    },
    Collection, Iterable,
};

/// A mutable collection providing the `iter_mut` method which returns an iterator over mutable references
/// of elements of the collection.
///
/// Since it extends `Collection`, `iter` method is also available which returns an iterator over shared references
/// of elements.
///
/// # Auto Implementations
///
/// Consider a collection type `X` storing elements of type `T`. Provided that the following implementations are provided:
///
/// * `X: IntoIterator<Item = T>`
/// * `&X: IntoIterator<Item = &T>`
/// * `&mut X: IntoIterator<Item = &mut T>`
///
/// Then, `X` implements `Collection<Item = T>` and `CollectionMut<Item = T>`.
/// Further, `&X` implements `Iterable<Item = &T>`.
///
/// # Examples
///
/// ```
/// use orx_iterable::*;
/// use arrayvec::ArrayVec;
/// use smallvec::{smallvec, SmallVec};
/// use std::collections::{LinkedList, VecDeque};
///
/// /// first computes sum, and then adds it to each of the elements
/// fn increment_by_sum(numbers: &mut impl CollectionMut<Item = i32>) {
///     let sum: i32 = numbers.iter().sum();
///
///     for x in numbers.iter_mut() {
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
pub trait CollectionMut: Collection {
    /// Type of the iterator yielding mutable references created by the [`iter_mut`] method.
    ///
    /// [`iter_mut`]: crate::CollectionMut::iter_mut
    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    /// Creates a new iterator yielding mutable references to the elements of the collection; i.e.,
    /// type of elements is `&mut Collection::Item`.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    // provided

    /// Combines mutable references of this collection and `other`; and creates an iterable collection which
    /// is a chain of these two collections.
    ///
    /// Note that this method does not change the memory locations of the elements; i.e.,
    /// the elements still live in two separate collections; however, now chained together.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = vec!['a', 'b'];
    /// let mut b = ['c', 'd', 'e'];
    ///
    /// let mut it = a.chained_mut(&mut b);
    ///
    /// *it.iter_mut().last().unwrap() = 'x';
    ///
    /// assert_eq!(it.iter().count(), 5);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), vec![&'a', &'b', &'c', &'d', &'x']);
    ///
    /// // neither a nor b is consumed
    /// assert_eq!(a, ['a', 'b']);
    /// assert_eq!(b, ['c', 'd', 'x']);
    /// ```
    fn chained_mut<'a, I>(
        &'a mut self,
        other: &'a mut I,
    ) -> ChainedCol<Self, I, &'a mut Self, &'a mut I>
    where
        Self: Sized,
        I: CollectionMut<Item = Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is a filtered version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [0i32, 1, 2];
    ///
    /// let mut it = a.filtered_mut(|x| x.is_positive());
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 2;
    /// }
    ///
    /// assert_eq!(it.iter().count(), 2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&2, &4]);
    ///
    /// // a is not consumed
    /// assert_eq!(a, [0, 2, 4]);
    /// ```
    fn filtered_mut<P>(&mut self, filter: P) -> FilteredCol<Self, &mut Self, P>
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool + Copy,
    {
        FilteredCol {
            it: self,
            filter,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is a flattened version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    ///
    /// let mut it = data.flattened_mut();
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 2;
    /// }
    ///
    /// assert_eq!(it.iter().count(), 6);
    /// assert_eq!(it.iter().sum::<u32>(), 2 * 21);
    ///
    /// // data is not consumed
    /// assert_eq!(data, [vec![2, 4, 6, 8], vec![10, 12]]);
    /// ```
    fn flattened_mut(&mut self) -> FlattenedCol<Self, &mut Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
        for<'i> &'i mut Self::Item: IntoIterator<Item = &'i mut <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is a fused version of this collection from its mutable reference.
    ///
    /// See [`core::iter::Fuse`] for details on fused iterators.
    fn fused_mut(&mut self) -> FusedCol<Self, &mut Self>
    where
        Self: Sized,
    {
        FusedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is a reversed version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    ///
    /// let mut a = [1, 2, 3];
    ///
    /// let mut it = a.reversed_mut();
    /// *it.iter_mut().next().unwrap() += 10;
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&13, &2, &1]);
    /// ```
    fn reversed_mut(&mut self) -> ReversedCol<Self, &mut Self>
    where
        Self: Sized,
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
        for<'b> Self::IterMut<'b>: DoubleEndedIterator,
    {
        ReversedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is skipped-by-`n` version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [1, 2, 3, 4, 5];
    ///
    /// let mut it = a.skipped_mut(2);
    ///
    /// for x in it.iter_mut() {
    ///     *x += 10;
    /// }
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&13, &14, &15]);
    ///
    /// assert_eq!(a, [1, 2, 13, 14, 15]);
    /// ```
    fn skipped_mut(&mut self, n: usize) -> SkippedCol<Self, &mut Self>
    where
        Self: Sized,
    {
        SkippedCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is skipped-while version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [-1i32, 0, 1];
    ///
    /// let mut it = a.skipped_while_mut(|x| x.is_negative());
    ///
    /// for x in it.iter_mut() {
    ///     *x += 10;
    /// }
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&10, &11]);
    ///
    /// assert_eq!(a, [-1, 10, 11]);
    /// ```
    fn skipped_while_mut<P>(&mut self, skip_while: P) -> SkippedWhileCol<Self, &mut Self, P>
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool + Copy,
    {
        SkippedWhileCol {
            it: self,
            skip_while,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is stepped-by-`step` version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [0, 1, 2, 3, 4, 5];
    ///
    /// let mut it = a.stepped_by_mut(2);
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 10;
    /// }
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&0, &20, &40]);
    ///
    /// assert_eq!(a, [0, 1, 20, 3, 40, 5]);
    /// ```
    fn stepped_by_mut(&mut self, step: usize) -> SteppedByCol<Self, &mut Self>
    where
        Self: Sized,
    {
        SteppedByCol {
            it: self,
            step,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is taken-`n` version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [1, 2, 3, 4, 5];
    ///
    /// let mut it = a.taken_mut(3);
    ///
    /// for x in it.iter_mut() {
    ///     *x += 10;
    /// }
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&11, &12, &13]);
    ///
    /// assert_eq!(a, [11, 12, 13, 4, 5]);
    /// ```
    fn taken_mut(&mut self, n: usize) -> TakenCol<Self, &mut Self>
    where
        Self: Sized,
    {
        TakenCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is taken-while version of this collection from its mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let mut a = [-1i32, 0, 1];
    ///
    /// let mut it = a.taken_while_mut(|x| x.is_negative());
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 10;
    /// }
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&-10]);
    ///
    /// assert_eq!(a, [-10, 0, 1]);
    /// ```
    fn taken_while_mut<P>(&mut self, take_while: P) -> TakenWhileCol<Self, &mut Self, P>
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhileCol {
            it: self,
            take_while,
            phantom: Default::default(),
        }
    }
}

impl<X> CollectionMut for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut X as IntoIterator>::into_iter(self)
    }
}
