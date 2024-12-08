use crate::transformations::{
    Chained, Cloned, Copied, Enumerated, FilterMapped, Filtered, FlatMapped, Flattened, Fused,
    Mapped, MappedWhile, Reversed, Skipped, SkippedWhile, SteppedBy, Taken, TakenWhile, Zipped,
};

/// An `Iterable` is any type which can return a new iterator that yields elements of the associated type [`Item`] every time [`iter`] method is called.
///
/// [`Item`]: crate::Iterable::Item
/// [`iter`]: crate::Iterable::iter
///
/// Notice that this is the least restrictive and most general iterable definition.
///
/// Three categories of types implement the Iterable trait:
///
/// * references of collections
/// * cloneable iterators
/// * lazy generators
///
/// # Auto Implementations
///
/// ## References of collections
///
/// First, consider a collection type `X` storing elements of type `T`.
/// Provided that the following implementation is provided:
///
/// * `&X: IntoIterator<Item = &T>`
///
/// Then, `&X` implements `Iterable<Item = &T>`.
///
/// In other words, a reference of a collection is an `Iterable`.
///
/// ## Cloneable iterators
///
/// Second, consider an iterator that can be cloned; i.e., `Iterator + Clone`.
/// This iterator can be converted into an `Iterable` which can be iterated over
/// repeatedly by calling `into_iterable` method.
///
/// ## Lazy Generators
///
/// Third, consider types iterators of which create values on the fly during the
/// iteration. One such example is the range.
/// Consider, for instance, the range 3..7.
/// Although it looks like a collection, it does not hold elements (3, 4, 5, 6) anywhere in memory. These elements are produced on the fly during the iteration.
/// `Iterable` trait implementations for the ranges are provided in this crate.
///
/// For similar custom types, the trait needs to be implemented explicitly.
///
/// # Examples
///
/// ```
/// use orx_iterable::*;
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
/// fn statistics(numbers: impl Iterable<Item = i64>) -> Stats {
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
/// // collections as Iterable
///
/// let x = [3, 5, 7];
/// statistics(x.copied()); // see Iterable's transformation methods such as copied, mapped, etc.
///
/// let x = vec![3, 5, 7];
/// statistics(x.copied());
///
/// let x = LinkedList::from_iter([3, 5, 7]);
/// statistics(x.copied());
///
/// let x = VecDeque::from_iter([3, 5, 7]);
/// statistics(x.copied());
///
/// let x = HashSet::<_>::from_iter([3, 5, 7]);
/// statistics(x.copied());
///
/// let x = BTreeSet::from_iter([3, 5, 7]);
/// statistics(x.copied());
///
/// let x = BinaryHeap::from_iter([3, 5, 7]);
/// statistics(x.copied());
///
/// let x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
/// statistics(x.copied());
///
/// let mut x = ArrayVec::<_, 16>::new();
/// x.extend([3, 5, 7]);
/// statistics(x.copied());
///
/// // cloneable iterators as Iterable
///
/// let x = (0..10).map(|x| x * 2).into_iterable();
/// statistics(x);
///
/// let x = vec![1, 2, 3];
/// let y = x
///     .iter()
///     .copied()
///     .filter(|x| x % 2 == 1)
///     .flat_map(|x| [-x, x])
///     .into_iterable();
/// statistics(y);
///
/// // lazy generators as Iterable
///
/// statistics(7..21i64);
/// ```
///
/// The following example represents an explicit implementation of the Iterable
/// trait for a lazy generator, which generates a sequence of Fibonacci numbers
/// up to a set bound.
///
/// ```
/// use orx_iterable::*;
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
/// impl Iterable for FibUntil {
///     type Item = u32;
///
///     type Iter = FibUntilIter;
///
///     fn iter(&self) -> Self::Iter {
///         FibUntilIter { curr: 0, next: 1, until: self.0 }
///     }
/// }
///
/// let fib = FibUntil(10); // Iterable
///
/// assert_eq!(fib.iter().count(), 7);
/// assert_eq!(fib.iter().max(), Some(8));
/// assert_eq!(fib.iter().collect::<Vec<_>>(), [0, 1, 1, 2, 3, 5, 8]);
/// ```
pub trait Iterable {
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
        Self: Sized,
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
        Self: Sized + Iterable<Item = &'a T>,
        T: Clone,
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
        Self: Sized + Iterable<Item = &'a T>,
        T: Copy,
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
    fn enumerated(self) -> Enumerated<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
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
        Self: Sized,
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
        Self: Sized,
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
        Self: Sized,
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
    fn fused(self) -> Fused<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
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
        Self: Sized,
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
        Self: Sized,
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
    fn skipped(self, n: usize) -> Skipped<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
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
    fn stepped_by(self, step: usize) -> SteppedBy<Self>
    where
        Self: Sized,
    {
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
    fn taken(self, n: usize) -> Taken<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
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
        Self: Sized,
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

impl<'a, X> Iterable for &'a [X] {
    type Item = &'a X;

    type Iter = core::slice::Iter<'a, X>;

    fn iter(&self) -> Self::Iter {
        <[X]>::iter(self)
    }
}
