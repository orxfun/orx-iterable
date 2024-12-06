use crate::{
    transformations::{
        ChainedCol, FilteredCol, FlattenedCol, FusedCol, ReversedCol, SkippedCol, SkippedWhileCol,
        SteppedByCol, TakenCol, TakenWhileCol,
    },
    Iterable,
};

/// Types implementing the `IterableCol` trait share the following key properties:
/// * It is a collection that owns its elements or [`Item`]s. Of course, the items themselves can be
///   references, in which case the references are owned by the iterable collection.
/// * Its [`iter`] method creates an iterator yielding references to elements of the collection;
///   i.e., the iterator yields `&Item`.
/// * Its [`iter_mut`] method creates an iterator yielding mutable references to elements of the collection;
///   i.e., the iterator yields `&mut Item`.
/// * Since it is an iterable, rather than an iterator, both `iter` and `iter_mut` methods can
///   be called any number of times to create new iterators.
///
/// [`Item`]: crate::IterableCol::Item
/// [`iter`]: crate::IterableCol::iter
/// [`iter_mut`]: crate::IterableCol::iter_mut
///
/// # Relation among Iterables
///
/// Main similarity and difference between [`Iterable`] and [`IterableCol`] traits are as follows:
/// * Both `Iterable` and `IterableCol` implements the `iter` method.
/// * However, only `IterableCol` implements `iter_mut`. This is natural as `Iterable` promises to create an
///   iterator yielding elements of type `Item`; however, does not promise to own them. On the other hand,
///   `IterableCol` is a special case which promises to own memory of the elements that it yields.
///
/// Practically, these definitions correspond to the following relations:
/// * if a collection `X` implements [`IterableCol<Item = T>`], then `&X` implements [`Iterable<Item = &T>`];
/// * on the other hand, a type implementing [`Iterable`] may not be a collection at all, such as [`Range<usize>`],
///   and hence, does not necessarily implement [`IterableCol`].
///
/// [`Range<usize>`]: core::ops::Range
///
/// # Types that auto-implement IterableCol
///
/// Consider a collection `X` which implements the following traits:
/// * `X: IntoIterator` => it can be consumed and turned into an iterator.
///   * if our collection instance is `x`, we can convert it into an iterator by `x.into_iter()`.
/// * `&X: IntoIterator<Item = &<X as IntoIterator>::Item>` => a reference to the collection can be converted to
///   an iterator yielding references to elements of the collection.
///   * for our collection `x`, we can use `(&x).into_iter()` to get an iterator over references of elements;
///   * however, almost always `x.iter()` is conventionally available to create the same iterator.
/// * `&mut X: IntoIterator<Item = &mut <X as IntoIterator>::Item>` => a mutable reference to the collection can be
///   converted to an iterator yielding mutable references to the elements.
///   * this means that we can use `(&mut x).into_iter()`;
///   * however, again `x.iter_mut()` is almost always available.
///
/// Some examples from the standard library are `Vec<T>`, `[T; N]`, `VecDeque<T>`, `LinkedList<T>`.
///
/// Or if you are using a collection outside std which satisfies the three requirements above, they will be
/// automatically implementing `IterableCol`. Some examples are `SmallVec`, `ArrayVec` or `SplitVec`.
pub trait IterableCol: Sized {
    /// Type of elements stored by the collection.
    type Item;

    /// Related type implementing `Iterable` trait that the `as_iterable` method returns.
    /// If the type of the `IterableCol` is `X`, the corresponding `Iterable` type is almost
    /// always `&X` due to the following relation among the both traits.
    ///
    /// Practically, these definitions correspond to the following relations:
    /// * if a collection `X` implements [`IterableCol<Item = T>`], then `&X` implements [`Iterable<Item = &T>`];
    /// * on the other hand, a type implementing [`Iterable`] may not be a collection at all, such as [`Range<usize>`],
    ///   and hence, does not necessarily implement [`IterableCol`].
    ///
    /// [`Range<usize>`]: core::ops::Range
    type Iterable<'i>: Iterable<Item = &'i Self::Item>
    where
        Self: 'i;

    /// Type of the iterator yielding mutable references created by the [`iter_mut`] method.
    ///
    /// [`iter_mut`]: crate::IterableCol::iter_mut
    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    /// Creates a new iterator yielding references to the elements of the collection; i.e.,
    /// type of elements is `&IterableCol::Item`.
    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        self.as_iterable().iter()
    }

    /// Creates a new iterator yielding mutable references to the elements of the collection; i.e.,
    /// type of elements is `&mut IterableCol::Item`.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

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
        I: IterableCol<Item = Self::Item>,
        for<'a> &'a I: Iterable<Item = &'a Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

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
        I: IterableCol<Item = Self::Item>,
        for<'b> &'b I: Iterable<Item = &'b Self::Item>,
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
        P: Fn(&Self::Item) -> bool + Copy,
    {
        FilteredCol {
            it: self,
            filter,
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
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
        for<'i> &'i mut Self::Item: IntoIterator<Item = &'i mut <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
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
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
        for<'i> &'i mut Self::Item: IntoIterator<Item = &'i mut <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Consumes this collection and creates an iterable collection which is a fused version of this collection.
    ///
    /// See [`core::iter::Fuse`] for details on fused iterators.
    fn into_fused(self) -> FusedCol<Self, Self> {
        FusedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    /// Creates an iterable collection view which is a fused version of this collection from its mutable reference.
    ///
    /// See [`core::iter::Fuse`] for details on fused iterators.
    fn fused_mut(&mut self) -> FusedCol<Self, &mut Self> {
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
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
        for<'b> Self::IterMut<'b>: DoubleEndedIterator,
    {
        ReversedCol {
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
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
        for<'b> Self::IterMut<'b>: DoubleEndedIterator,
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
    fn into_skipped(self, n: usize) -> SkippedCol<Self, Self> {
        SkippedCol {
            it: self,
            n,
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
    fn skipped_mut(&mut self, n: usize) -> SkippedCol<Self, &mut Self> {
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
        P: Fn(&Self::Item) -> bool + Copy,
    {
        SkippedWhileCol {
            it: self,
            skip_while,
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
    fn into_stepped_by(self, step: usize) -> SteppedByCol<Self, Self> {
        SteppedByCol {
            it: self,
            step,
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
    fn stepped_by_mut(&mut self, step: usize) -> SteppedByCol<Self, &mut Self> {
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
    fn into_taken(self, n: usize) -> TakenCol<Self, Self> {
        TakenCol {
            it: self,
            n,
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
    fn taken_mut(&mut self, n: usize) -> TakenCol<Self, &mut Self> {
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
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhileCol {
            it: self,
            take_while,
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
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhileCol {
            it: self,
            take_while,
            phantom: Default::default(),
        }
    }
}

impl<X> IterableCol for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    type Iterable<'i> = &'i X
    where
        Self: 'i;

    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        <&X as IntoIterator>::into_iter(self)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut X as IntoIterator>::into_iter(self)
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

use crate::*;

struct Stats {
    count: usize,
    mean: i64,
    std_dev: i64,
}

fn statistics(numbers: impl Iterable<Item = i64>) -> Stats {
    let count = numbers.iter().count() as i64;
    let sum = numbers.iter().sum::<i64>();
    let mean = sum / count;
    let sum_sq_errors: i64 = numbers.iter().map(|x| (x - mean) * (x - mean)).sum();
    let std_dev = f64::sqrt(sum_sq_errors as f64 / (count - 1) as f64) as i64;
    Stats {
        count: count as usize,
        mean,
        std_dev,
    }
}
