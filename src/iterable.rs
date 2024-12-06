use crate::transformations::{
    Chained, Cloned, Copied, Enumerated, FilterMapped, Filtered, FlatMapped, Flattened, Fused,
    Mapped, MappedWhile, Reversed, Skipped, SkippedWhile, SteppedBy, Taken, TakenWhile, Zipped,
};

/// A type implementing the `Iterable` trait is characterized by the following characteristics:
/// * Its [`iter`] method creates an iterator yielding elements of type [`Item`].
/// * Since it is an iterable, rather than an iterator, `iter` method can
///   be called any number of times to create new iterators.
/// * It does not necessarily own the data; see [`IterableCol`] for iterable collections.
///   A straightforward example is the range `0..4` which is an iterable creating iterators that
///   would yield values 0, 1 and 2. However, these elements are not actually stored in memory;
///   in this sense, range is not a collection.
/// * On the other hand, collections owning the data can and will most likely implement `Iterable`.
///   See the **Relation among Iterables** section for details.
///
/// [`iter`]: crate::Iterable::iter
/// [`Item`]: crate::Iterable::Item
/// [`IterableCol`]: crate::IterableCol
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
/// # Types that implement Iterable
///
/// Consider a type, most likely a collection, `X` that satisfies the following:
/// * `&X: IntoIterator` => a reference to the collection can be converted to an iterator.
///   * for our instance `x`, we can use `(&x).into_iter()` to get an iterator over references of elements;
///   * however, almost always `x.iter()` is conventionally available to create the same iterator.
///
/// Then, `X` auto-implements `Iterable`.
///
/// Other than collections, any source that can create an iterator can be an `Iterable`.
/// For instance `Range<usize>` implements `Iterable<Item = usize>`.
///
/// Also consider an `Iterator` which is cheap to `Clone`.
/// For instance, let `x` be an instance of a vector and `let iter = x.iter().map(|x| 2 * x)` be our iterator.
/// Notice that `iter` holds a reference to the vector and a function pointer; and hence, a cheap to clone type.
/// Now, consider that we want to consume `iter` more than once; i.e., we want an `Iterable` rather than an `Iterator`.
/// Unfortunately, `iter` cannot directly implement `Iterable`.
/// However, it can easily be transformed into one by calling `let it = iter.into_iterable()`.
/// Now, `it` carries the definition of the lazy computation and we can call `it.iter()` as many times as we require.
///
/// # Examples
///
/// The following example illustrates the main functionality of the trait
/// allowing to create multiple iterators.
///
/// ```
/// use orx_iterable::*;
/// use std::collections::HashSet;
///
/// fn sum_and_count<'a>(numbers: impl Iterable<Item = &'a u32>) -> (u32, usize) {
///     let count = numbers.iter().count();
///     let sum: u32 = numbers.iter().sum();
///     (sum, count)
/// }
///
/// // collections as Iterable
///
/// let array = [1, 4, 7];
/// assert_eq!(sum_and_count(&array), (12, 3));
///
/// let vec = vec![1, 4, 7];
/// assert_eq!(sum_and_count(&vec), (12, 3));
///
/// let set: HashSet<_> = [1, 4, 7].into_iter().collect();
/// assert_eq!(sum_and_count(&set), (12, 3));
///
/// // Iterator's as Iterable
/// let iter = vec.iter().filter(|x| **x < 5);
/// assert_eq!(sum_and_count(iter.into_iterable()), (5, 2));
/// ```
///
/// The second example demonstrates chaining iterables.
///
/// ```
/// use orx_iterable::*;
///
/// fn sum_and_count(numbers: impl Iterable<Item = u32>) -> (u32, usize) {
///     let count = numbers.iter().count();
///     let sum: u32 = numbers.iter().sum();
///     (sum, count)
/// }
///
/// let vec = vec![1, 5, 3, 9, 8, 3, 7, 6];
///
/// let iter = vec
///     .skipped(1)                 // [5, 3, 9, 8, 3, 7, 6]
///     .taken_while(|x| **x != 7)  // [5, 3, 9, 8, 3]
///     .filtered(|x| **x % 2 == 1) // [5, 3, 9, 3]
///     .mapped(|x| x * 2);         // [10, 6, 18, 6]
///
/// assert_eq!(sum_and_count(iter), (40, 4));
/// ```
pub trait Iterable: Sized {
    /// Type of the item that the iterators created by the [`iter`] method yields.
    ///
    /// [`iter`]: crate::Iterable::iter
    type Item;

    /// Type of the iterator created by the [`iter`] method.
    ///
    /// [`iter`]: crate::Iterable::iter
    type Iter: Iterator<Item = Self::Item>;

    /// Creates a new iterator from this iterable yielding elements of type `Iterable::Item`.
    fn iter(&self) -> Self::Iter;

    // provided

    /// Takes two iterables and creates a new iterable over both in sequence.
    ///
    /// In other words, it links two iterators together, in a chain.
    ///
    /// [`once`] is commonly used to adapt a single value into a chain of other kinds of iteration.
    ///
    /// [`once`]: crate::once
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = vec!['a', 'b'];
    /// let b = ['c', 'd', 'e'];
    ///
    /// let it = a.chained(&b).copied();
    /// assert_eq!(it.iter().count(), 5);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), vec!['a', 'b', 'c', 'd', 'e']);
    /// ```
    fn chained<I>(self, other: I) -> Chained<Self, I>
    where
        I: Iterable<Item = Self::Item>,
    {
        Chained {
            it1: self,
            it2: other,
        }
    }

    /// Creates an iterable, iterators of which clone all of its elements.
    ///
    /// This is useful when you have an iterable over &T, but you need an iterable over T.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// fn count_and_sum(data: impl Iterable<Item = i32>) -> (usize, i32) {
    ///     (data.iter().count(), data.iter().sum())
    /// }
    ///
    /// let a = vec![1, 3, 7, 15];
    ///
    /// assert_eq!((4, 26), count_and_sum(a.cloned()));
    ///
    /// assert_eq!((3, 11), count_and_sum(a.filtered(|x| **x < 10).cloned()));
    /// ```
    fn cloned<'a, T>(self) -> Cloned<'a, T, Self>
    where
        T: Clone,
        Self: Iterable<Item = &'a T>,
    {
        Cloned { it: self }
    }

    /// Creates an iterable, iterators of which copy all of its elements.
    ///
    /// This is useful when you have an iterable over &T, but you need an iterable over T.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// fn count_and_sum(data: impl Iterable<Item = i32>) -> (usize, i32) {
    ///     (data.iter().count(), data.iter().sum())
    /// }
    ///
    /// let a = vec![1, 3, 7, 15];
    ///
    /// assert_eq!((4, 26), count_and_sum(a.copied()));
    ///
    /// assert_eq!((3, 11), count_and_sum(a.filtered(|x| **x < 10).copied()));
    /// ```
    fn copied<'a, T>(self) -> Copied<'a, T, Self>
    where
        T: Copy,
        Self: Iterable<Item = &'a T>,
    {
        Copied { it: self }
    }

    /// Creates an iterable which gives the current iteration count as well as the next value.
    ///
    /// The iterators created by enumerated iterable yields pairs `(i, val)`,
    /// where `i` is the current index of iteration and `val` is the value returned by the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = ['a', 'b', 'c'];
    /// let it = a.enumerated();
    ///
    /// assert_eq!(it.iter().count(), 3);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), vec![(0, &'a'), (1, &'b'), (2, &'c')]);
    /// ```
    fn enumerated(self) -> Enumerated<Self> {
        Enumerated { it: self }
    }

    /// Creates an iterable that both filters and maps.
    ///
    /// Iterators of the returned iterable yields only the values for which the supplied closure returns `Some(value)`.
    ///
    /// `filter_mapped` can be used to make chains of `filtered` and `mapped` more concise.
    /// The example below shows how a `mapped().filtered().mapped()` can be shortened to a single call
    /// to `filter_mapped`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = ["1", "two", "NaN", "four", "5"];
    ///
    /// let it = a.filter_mapped(|s| s.parse::<u32>().ok());
    ///
    /// assert_eq!(it.iter().count(), 2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), vec![1, 5]);
    /// ```
    fn filter_mapped<M, U>(self, filter_map: M) -> FilterMapped<Self, M, U>
    where
        M: Fn(Self::Item) -> Option<U> + Copy,
    {
        FilterMapped {
            it: self,
            filter_map,
        }
    }

    /// Creates an iterable which uses a closure to determine if an element should be yielded.
    ///
    /// Given an element the closure must return true or false. Iterators of the returned iterable
    /// will yield only the elements for which the closure returns true.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [0i32, 1, 2];
    ///
    /// let it = a.filtered(|x| x.is_positive());
    ///
    /// assert_eq!(it.iter().count(), 2);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2]);
    /// ```
    fn filtered<P>(self, filter: P) -> Filtered<Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        Filtered { it: self, filter }
    }

    /// Creates an iterable that works like map, but flattens nested structure.
    ///
    /// You can think of `flat_mapped(f)` as the semantic equivalent of mapping,
    /// and then flattening as in `mapped(f).flattened()`.
    ///
    /// Another way of thinking about `flat_mapped()`:
    ///
    /// * `mapped`’s closure returns one item for each element, and
    /// * `flat_map()`’s closure returns an iterator for each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let words = ["al", "p", "ha"];
    ///
    /// let it = words.flat_mapped(|s| s.chars());
    ///
    /// assert_eq!(it.iter().count(), 5);
    /// assert_eq!(it.iter().collect::<String>().as_str(), "alpha");
    /// ```
    fn flat_mapped<M, U>(self, flat_map: M) -> FlatMapped<Self, M, U>
    where
        U: IntoIterator,
        M: Fn(Self::Item) -> U + Copy,
    {
        FlatMapped { it: self, flat_map }
    }

    /// Creates an iterable that flattens nested structure.
    ///
    /// This is useful when you have an iterable of iterators or an iterable of things that can be
    /// turned into iterators and you want to remove one level of indirection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    ///
    /// let it = data.flattened();
    ///
    /// assert_eq!(it.iter().count(), 6);
    /// assert_eq!(it.iter().sum::<u32>(), 21);
    /// ```
    fn flattened(self) -> Flattened<Self>
    where
        Self::Item: IntoIterator,
    {
        Flattened { it: self }
    }

    /// Creates an iterable which ends after the first `None`.
    ///
    /// After an iterator returns `None`, future calls may or may not yield `Some(T)` again.
    /// fuse() adapts an iterator, ensuring that after a `None` is given, it will always return `None` forever.
    ///
    /// Note that the Fuse wrapper is a no-op on iterators that implement the FusedIterator trait.
    /// fuse() may therefore behave incorrectly if the FusedIterator trait is improperly implemented.
    fn fused(self) -> Fused<Self> {
        Fused { it: self }
    }

    /// Creates an iterable that both yields elements based on a predicate and maps.
    ///
    /// `map_while()` takes a closure as an argument. It will call this closure on each element
    /// of the iterator, and yield elements while it returns `Some(_)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [0, 1, 2, -3, 4, 5, -6];
    ///
    /// let it = a.mapped_while(|x| u32::try_from(*x).ok());
    ///
    /// assert_eq!(it.iter().count(), 3);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [0, 1, 2]);
    /// ```
    fn mapped_while<M, U>(self, map_while: M) -> MappedWhile<Self, M, U>
    where
        M: Fn(Self::Item) -> Option<U> + Copy,
    {
        MappedWhile {
            it: self,
            map_while,
        }
    }

    /// Takes a closure and creates an iterable which calls that closure on each element.
    ///
    /// map() transforms one iterator into another, by means of its argument `map`.
    /// It produces a new iterable, iterators of which calls this closure on each element of
    /// the original iterable.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 3, 6];
    ///
    /// let it = a.mapped(|x| 2 * x);
    ///
    /// assert_eq!(it.iter().sum::<i32>(), 20);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [2, 6, 12]);
    /// ```
    fn mapped<M, U>(self, map: M) -> Mapped<Self, M, U>
    where
        M: Fn(Self::Item) -> U + Copy,
    {
        Mapped { it: self, map }
    }

    /// Creates an iterable iterators of which reverses the traversal direction.
    ///
    /// This is only possible if the iterable's iterator type has an end,
    /// so `reversed()` only works when `Iterable::Iter` is a `DoubleEndedIterator`.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 2, 3];
    ///
    /// let it = a.reversed();
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&3, &2, &1]);
    ///
    /// let it = it.reversed();
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    /// ```
    fn reversed(self) -> Reversed<Self>
    where
        Self::Iter: DoubleEndedIterator,
    {
        Reversed { it: self }
    }

    /// Creates an iterable, iterators of which skip the first `n` elements.
    ///
    /// Created iterators skip elements until n elements are skipped or the end of the iterator
    /// is reached (whichever happens first).
    /// After that, all the remaining elements are yielded. In particular, if the original iterator
    /// is too short, then the returned iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 2, 3];
    ///
    /// let it = a.skipped(2);
    ///
    /// assert_eq!(it.iter().count(), 1);
    /// assert_eq!(it.iter().next(), Some(&3));
    /// ```
    fn skipped(self, n: usize) -> Skipped<Self> {
        Skipped { it: self, n }
    }

    /// Creates an iterable, iterators of which skip elements based on a predicate.
    ///
    /// `skipped_while()` takes a closure as an argument. It will call this closure on each element
    /// of the iterator, and ignore elements until it returns false.
    ///
    /// After false is returned, `skip_while`’s job is over, and the rest of the elements are yielded.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [-1i32, 0, 1];
    ///
    /// let it = a.skipped_while(|x| x.is_negative());
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&0, &1]);
    /// ```
    fn skipped_while<P>(self, skip_while: P) -> SkippedWhile<Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        SkippedWhile {
            it: self,
            skip_while,
        }
    }

    /// Creates an iterable starting at the same point, but stepping by the given amount at each iteration.
    ///
    /// The first element of the iterator will always be returned, regardless of the step given.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [0, 1, 2, 3, 4, 5];
    ///
    /// let it = a.stepped_by(2);
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&0, &2, &4]);
    /// ```
    fn stepped_by(self, step: usize) -> SteppedBy<Self> {
        SteppedBy { it: self, step }
    }

    /// Creates an iterable whose iterators yield the first `n` elements, or fewer if the underlying iterator ends sooner.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [1, 2, 3];
    ///
    /// let it = a.taken(2);
    ///
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [&1, &2]);
    /// ```
    fn taken(self, n: usize) -> Taken<Self> {
        Taken { it: self, n }
    }

    /// Creates an iterable, iterators of which yield elements based on a predicate.
    ///
    /// `taken_while()` takes a closure as an argument.
    /// It will call this closure on each element of the iterator, and yield elements while it returns true.
    ///
    /// After false is returned, the rest of the elements are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a = [-1i32, 0, 1];
    ///
    /// let it = a.taken_while(|x| x.is_negative());
    ///
    /// assert_eq!(it.iter().count(), 1);
    /// assert_eq!(it.iter().next(), Some(&-1));
    /// ```
    fn taken_while<P>(self, take_while: P) -> TakenWhile<Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhile {
            it: self,
            take_while,
        }
    }

    /// ‘Zips up’ two iterables into a single iterable of pairs.
    ///
    /// The zipped iterable creates zipped iterators.
    ///
    /// If either iterator returns None, next from the zipped iterator will return None.
    /// If the zipped iterator has no more elements to return then each further attempt to advance it will first try to
    /// advance the first iterator at most one time and if it still yielded an item try to advance the second iterator
    /// at most one time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_iterable::*;
    ///
    /// let a1 = [1, 2, 3];
    /// let b1 = [4, 5, 6, 7];
    ///
    /// let it = a1.zipped(&b1);
    ///
    /// assert_eq!(it.iter().count(), 3);
    /// assert_eq!(it.iter().collect::<Vec<_>>(), [(&1, &4), (&2, &5), (&3, &6)]);
    /// ```
    fn zipped<I>(self, other: I) -> Zipped<Self, I>
    where
        I: Iterable,
    {
        Zipped {
            it1: self,
            it2: other,
        }
    }
}

// impl

impl<'a, X> Iterable for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    type Iter = <&'a X as IntoIterator>::IntoIter;

    fn iter(&self) -> Self::Iter {
        self.into_iter()
    }
}
