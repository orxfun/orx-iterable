use crate::{
    transformations::{
        ChainedCol, FilteredCol, FlattenedCol, FusedCol, ReversedCol, SkippedCol, SkippedWhileCol,
        SteppedByCol, TakenCol, TakenWhileCol,
    },
    Iterable,
};

/// A collection providing the `iter` method which returns an iterator over shared references
/// of elements of the collection.
///
/// # Auto Implementations
///
/// Consider a collection type `X` storing elements of type `T`. Provided that the following implementations are provided:
///
/// * `X: IntoIterator<Item = T>`
/// * `&X: IntoIterator<Item = &T>`
///
/// Then, `X` implements `Collection<Item = T>`.
/// Further, `&X` implements `Iterable<Item = &T>`.
///
/// # Examples
///
/// ```
/// use orx_iterable::*;
/// use arrayvec::ArrayVec;
/// use smallvec::{smallvec, SmallVec};
/// use std::collections::{BinaryHeap, BTreeSet, HashSet, LinkedList, VecDeque};
///
/// struct Stats {
///     count: usize,
///     mean: i64,
///     std_dev: i64,
/// }
///
/// /// we need multiple iterations over numbers to compute the stats
/// fn statistics(numbers: &impl Collection<Item = i64>) -> Stats {
///     let count = numbers.iter().count() as i64;
///     let sum = numbers.iter().sum::<i64>();
///     let mean = sum / count;
///     let sum_sq_errors: i64 = numbers.iter().map(|x| (x - mean) * (x - mean)).sum();
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
pub trait Collection {
    /// Type of elements stored by the collection.
    type Item;

    /// Related type implementing `Iterable` trait that the `as_iterable` method returns.
    /// If the type of the `Collection` is `X`, the corresponding `Iterable` type is almost
    /// always `&X` due to the following relation among the both traits.
    ///
    /// Practically, these definitions correspond to the following relations:
    /// * if a collection `X` implements [`Collection<Item = T>`], then `&X` implements [`Iterable<Item = &T>`];
    /// * on the other hand, a type implementing [`Iterable`] may not be a collection at all, such as [`Range<usize>`],
    ///   and hence, does not necessarily implement [`Collection`].
    ///
    /// [`Range<usize>`]: core::ops::Range
    type Iterable<'i>: Iterable<Item = &'i Self::Item>
    where
        Self: 'i;

    /// Creates a new iterator yielding references to the elements of the collection; i.e.,
    /// type of elements is `&Collection::Item`.
    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        self.as_iterable().iter()
    }

    /// Returns the corresponding `Iterable` type of this collection, which is often nothing but `&Self`.
    fn as_iterable(&self) -> Self::Iterable<'_>;

    // provided

    /// Consumes this collection and `other`; creates an iterable collection which is a chain of these two
    /// collections.
    ///
    /// Note that this method does not change the memory locations of the elements; i.e.,
    /// the elements still live in two separate collections; however, now chained together.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = vec!['a', 'b'];
    /// let b = ['c', 'd', 'e'];
    ///
    /// let mut it = a.into_chained(b);
    ///
    /// *it.iter_mut().last().unwrap() = 'x';
    ///
    /// assert_eq!(it.iter().count(), 5);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), vec![&'a', &'b', &'c', &'d', &'x']);
    /// ```
    fn into_chained<I>(self, other: I) -> ChainedCol<Self, I, Self, I>
    where
        Self: Sized,
        I: Collection<Item = Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is a filtered version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [0i32, 1, 2];
    ///
    /// let mut it = a.into_filtered(|x| x.is_positive());
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 2;
    /// }
    ///
    /// assert_eq!(it.iter().count(), 2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&2, &4]);
    /// ```
    fn into_filtered<P>(self, filter: P) -> FilteredCol<Self, Self, P>
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

    /// Consumes this collection and creates an iterable collection which is a flattened version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    ///
    /// let mut it = data.into_flattened();
    ///
    /// for x in it.iter_mut() {
    ///     *x *= 2;
    /// }
    ///
    /// assert_eq!(it.iter().count(), 6);
    /// assert_eq!(it.iter().sum::<u32>(), 2 * 21);
    /// ```
    fn into_flattened(self) -> FlattenedCol<Self, Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is a fused version of this collection.
    ///
    /// See [`core::iter::Fuse`] for details on fused iterators.
    fn into_fused(self) -> FusedCol<Self, Self>
    where
        Self: Sized,
    {
        FusedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is a reversed version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    ///
    /// let a = [1, 2, 3];
    ///
    /// let it = a.into_reversed();
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&3, &2, &1]);
    ///
    /// let it = it.into_reversed();
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    /// ```
    fn into_reversed(self) -> ReversedCol<Self, Self>
    where
        Self: Sized,
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
    {
        ReversedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is skipped-by-`n` version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 2, 3, 4, 5];
    ///
    /// let it = a.into_skipped(2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&3, &4, &5]);
    ///
    /// let it = it.into_skipped(1);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&4, &5]);
    /// ```
    fn into_skipped(self, n: usize) -> SkippedCol<Self, Self>
    where
        Self: Sized,
    {
        SkippedCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is skipped-while version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [-1i32, 0, 1];
    ///
    /// let it = a.into_skipped_while(|x| x.is_negative());
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&0, &1]);
    /// ```
    fn into_skipped_while<P>(self, skip_while: P) -> SkippedWhileCol<Self, Self, P>
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

    /// Consumes this collection and creates an iterable collection which is stepped-by-`step` version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [0, 1, 2, 3, 4, 5];
    ///
    /// let it = a.into_stepped_by(2);
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&0, &2, &4]);
    /// ```
    fn into_stepped_by(self, step: usize) -> SteppedByCol<Self, Self>
    where
        Self: Sized,
    {
        SteppedByCol {
            it: self,
            step,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is taken-`n` version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 2, 3, 4, 5];
    ///
    /// let it = a.into_taken(3);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    ///
    /// let it = it.into_taken(2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2]);
    /// ```
    fn into_taken(self, n: usize) -> TakenCol<Self, Self>
    where
        Self: Sized,
    {
        TakenCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is taken-while version of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [-1i32, 0, 1];
    ///
    /// let it = a.into_taken_while(|x| x.is_negative());
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&-1]);
    /// ```
    fn into_taken_while<P>(self, take_while: P) -> TakenWhileCol<Self, Self, P>
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

impl<X> Collection for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    type Iterable<'i>
        = &'i X
    where
        Self: 'i;

    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        <&X as IntoIterator>::into_iter(self)
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
