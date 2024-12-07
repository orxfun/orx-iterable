# orx-iterable

[![orx-iterable crate](https://img.shields.io/crates/v/orx-iterable.svg)](https://crates.io/crates/orx-iterable)
[![orx-iterable documentation](https://docs.rs/orx-iterable/badge.svg)](https://docs.rs/orx-iterable)

Defines and implements Iterable, Collection and CollectionMut traits to represent types that can be iterated over multiple times.

There exist numerous situations where we need to iterate over an abstract type multiple times. Currently most collections allow this by `iter` and `iter_mut` methods; however, this is a convention rather than a shared behavior defined by a trait method.

> `Collection` and `CollectionMut` traits are defined and automatically implemented for all collections enabling abstraction over repeatedly iterable collection types.

However, not all iterables are collections storing elements. For instance, a *Range* is iterable which cannot return a reference to its elements since the elements are not stored. Similarly, consider an iterator mapping elements of a collection to new values. As these values are not stored in memory and computed on the fly during iteration, they do not fit in the definition of collections.

> More general immutable iterable trait `Iterable` is defined and implemented for most common relevant types.

## A. Collection and CollectionMut

The core method of the Collection trait is `iter(&self)` which returns an iterator yielding shared references; i.e., `&Item`.

CollectionMut trait extends Collection with the `iter_mut(&mut self)` method that creates an iterator yielding mutable references; i.e., `&mut Item`.

Definitions of the collection traits are based on the IntoIterator trait as follows. Consider a collection of type `X` which implements `IntoIterator<Item = T>`. Then, the following conditions automatically hold.

```rust ignore
&X: IntoIterator<Item = &T>           ===>   X: Collection
&mut X: IntoIterator<Item = &mut T>   ===>   X: CollectionMut
```

Note that the implications are straightforward. Provided that `&X` can be converted into an iterator yielding references to elements, then we can automatically implement `iter(&self)`. The same follows for the mutable extension.

Collections in the standard library satisfy these conditions (*see below the note for maps*). Further, most collection crates follow the IntoIterator pattern, and hence, automatically implement collection traits. Some examples are SmallVec, ArrayVec, StackVec, SmallOrdSet, etc.

<br />
<details>
<summary style="font-weight:bold;"><code>Collection</code> Example</summary>

Consider, for instance, a method which creates statistics from a collection of numbers. In order to be able to compute the required values, it needs at least two iterations over the data. In the following example, we use the Collection trait to define this requirement.

```rust
use orx_iterable::*;
use arrayvec::ArrayVec;
use smallvec::{smallvec, SmallVec};
use std::collections::{BinaryHeap, BTreeSet, HashSet, LinkedList, VecDeque};

struct Stats {
    count: usize,
    mean: i64,
    std_dev: i64,
}

/// we need multiple iterations over numbers to compute the stats
fn statistics(numbers: &impl Collection<Item = i64>) -> Stats {
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

// example collections that automatically implement Collection

statistics(&[3, 5, 7]);
statistics(&vec![3, 5, 7]);
statistics(&LinkedList::from_iter([3, 5, 7]));
statistics(&VecDeque::from_iter([3, 5, 7]));
statistics(&HashSet::<_>::from_iter([3, 5, 7]));
statistics(&BTreeSet::<_>::from_iter([3, 5, 7]));
statistics(&BinaryHeap::<_>::from_iter([3, 5, 7]));

let x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
statistics(&x);

let mut x = ArrayVec::<_, 16>::new();
x.extend([3, 5, 7]);
statistics(&x);
```
</details>
<br />

<br />
<details>
<summary style="font-weight:bold;"><code>CollectionMut</code> Example</summary>

The `increment_by_sum` method below first computes the sum of all elements and then increments each element by this sum. Therefore, we require both the iter and iter_mut methods, which can be represented by the `CollectionMut` trait.

```rust
use orx_iterable::*;
use arrayvec::ArrayVec;
use smallvec::{smallvec, SmallVec};
use std::collections::{LinkedList, VecDeque};

/// first computes sum, and then adds it to each of the elements
fn increment_by_sum(numbers: &mut impl CollectionMut<Item = i32>) {
    let sum: i32 = numbers.iter().sum();

    for x in numbers.iter_mut() {
        *x += sum;
    }
}

// example collections that automatically implement CollectionMut

let mut x = [1, 2, 3];
increment_by_sum(&mut x);
assert_eq!(x, [7, 8, 9]);

let mut x = vec![1, 2, 3];
increment_by_sum(&mut x);

let mut x = LinkedList::from_iter([1, 2, 3]);
increment_by_sum(&mut x);

let mut x = VecDeque::from_iter([1, 2, 3]);
increment_by_sum(&mut x);

let mut x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
increment_by_sum(&mut x);

let mut x = ArrayVec::<_, 16>::new();
x.extend([3, 5, 7]);
increment_by_sum(&mut x);
```
</details>
<br />

*Maps do not follow the definition above as their iterators behave slightly differently, and hence, they deserve their own trait.*

## B. Iterable

Collection traits are useful; however, they do not cover all iterables. By definition, they are bound to yield shared or mutable references to their elements. However, some iterators produce elements on the fly during iteration. Therefore, they cannot return a reference to the temporarily computed values. Further, mutable references is irrelevant. Therefore, we require a more general definition for immutable iterables.

> An `Iterable` is any type which can return a new iterator that yields elements of the associated type `Item` every time `iter` method is called.

Three categories of types implement the Iterable trait:

* references of collections
* cloneable iterators
* lazy generators

<br />
<details>
<summary style="font-weight:bold;"><code>Iterable</code> Example</summary>

In the following example, we relax the `Collection` requirement on numbers to `Iterable`. The example demonstrates the flexibility of the Iterable trait abstracting the input over the three categories of implementing types listed above.

```rust
use orx_iterable::*;
use arrayvec::ArrayVec;
use smallvec::{smallvec, SmallVec};
use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};

struct Stats {
    count: usize,
    mean: i64,
    std_dev: i64,
}

/// we need multiple iterations over numbers to compute the stats
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

// collections as Iterable

let x = [3, 5, 7];
statistics(x.copied()); // see section C for details of copied()

let x = vec![3, 5, 7];
statistics(x.copied());

let x = LinkedList::from_iter([3, 5, 7]);
statistics(x.copied());

let x = VecDeque::from_iter([3, 5, 7]);
statistics(x.copied());

let x = HashSet::<_>::from_iter([3, 5, 7]);
statistics(x.copied());

let x = BTreeSet::from_iter([3, 5, 7]);
statistics(x.copied());

let x = BinaryHeap::from_iter([3, 5, 7]);
statistics(x.copied());

let x: SmallVec<[_; 128]> = smallvec![3, 5, 7];
statistics(x.copied());

let mut x = ArrayVec::<_, 16>::new();
x.extend([3, 5, 7]);
statistics(x.copied());

// cloneable iterators as Iterable

let x = (0..10).map(|x| x * 2).into_iterable();
statistics(x);

let x = vec![1, 2, 3];
let y = x
    .iter()
    .copied()
    .filter(|x| x % 2 == 1)
    .flat_map(|x| [-x, x])
    .into_iterable();
statistics(y);

// lazy generators as Iterable

statistics(7..21i64);

// also see FibUntil example in section B3
```
</details>
<br />

### B.1. Collections as Iterable

References of collections automatically implement Iterable, again based on the IntoIterator implementation.

```rust ignore
&X: IntoIterator<Item = &T>           ===>   &X: Iterable
// equivalently
X: Collection<Item = T>               ===>   &X: Iterable
```

Note that the implication is straightforward. If we can implement `iter(&self)` for the collection itself, we can implement it for its reference too. This implementation is useful in allowing to create the more general Iterable's from Collection's; we can simply use a reference of a collection when we require an Iterable.

### B.2. Cloneable Iterators

An iterator is not limited to visiting elements of a collection. Thanks to chainable methods transforming one iterator to another, such as `filter` or `map`, iterators are capable of carrying definition of a computation over some data.

The trouble is, iterators are not repeatedly iterable. However, the conversion is conveniently possible.

Any iterator that can be cloned (`Iterator + Clone`) can be converted into an iterable by calling the `into_iterable` method.

### B.3. Lazy Generators

Some iterators yield elements which are computed and created each time its *next* method is called. These types can be represented as an Iterable; however, there is no generic implementation for them.

A common example from core library is range. Consider, for instance, the range 3..7. Although it looks like a collection, it does not hold elements (3, 4, 5, 6) anywhere in memory. These elements are produced on the fly during the iteration. `Iterable` trait implementations for the ranges are provided in this crate.

For similar custom types, the trait needs to be implemented explicitly.

<br />
<details>
<summary style="font-weight:bold;">Custom lazy generator as <code>Iterable</code></summary>

In the following example, `FibUntilIter` is an iterator which computes its elements on the fly. `FibUntil` knows how to create this iterator repeatedly. In other words, it is an iterable, and hence, explicitly implements `Iterable`.

```rust
use orx_iterable::*;

struct FibUntilIter {
    curr: u32,
    next: u32,
    until: u32,
}

impl Iterator for FibUntilIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        match current > self.until {
            false => Some(current),
            true => None,
        }
    }
}

struct FibUntil(u32);

impl Iterable for FibUntil {
    type Item = u32;

    type Iter = FibUntilIter;

    fn iter(&self) -> Self::Iter {
        FibUntilIter { curr: 0, next: 1, until: self.0 }
    }
}

let fib = FibUntil(10); // Iterable

assert_eq!(fib.iter().count(), 7);
assert_eq!(fib.iter().max(), Some(8));
assert_eq!(fib.iter().collect::<Vec<_>>(), [0, 1, 1, 2, 3, 5, 8]);
```
</details>
<br />

## C. Chainable Transformations

`Iterator` trait provides a wide variety of methods which transforms one iterator into another, such as `filter`, `map` or `flat_map`. These transformations can nicely be chained to compose computations.

`Iterable`, `Collection` and `CollectionMut` traits follow the same design and provide these chainable transformation methods.

```rust
use orx_iterable::*;

let a = vec![3, 7, 1];
let b = vec![8];
let c = [true, false, false, true];

let it = a
    .chained(&b)                // [&3, &7, &1, &8]
    .zipped(&c)                 // [(&3, &t), (&7, &f), (&1, &f), (&8, &t)]
    .filtered(|(_, b)| **b)     // [(&3, &t), (&8, &t)]
    .mapped(|(a, _)| a)         // [&3, &8]
    .copied()                   // [3, 8]
    .flat_mapped(|x| [x, -x]);  // [3, -3, 8, -8]

assert_eq!(it.iter().count(), 4);
assert_eq!(it.iter().sum::<i32>(), 0);
```

Also see consuming and mutable variants such as `into_filtered` or `filtered_mut`.

## D. Defining New Custom Collections

This crate aims to bring in the missing iterable and collection traits while keeping manual implementations as few as possible. Rust's joyful type system ❤️🦀 and the consistent usage of the IntoIterator trait in standard library and collection crates allow us to achieve this almost effortlessly.

When developing a new collection type, iterable and collection traits and corresponding iter and iter_mut methods will be readily available once we provide the relevant IntoIterator implementations.

### D.1. Custom Collection

Assume that our collection `X` does not allow for iter_mut such as the `HashSet`. In this case, once we provide the following implementations:

* `X: IntoIterator`
* `&X: IntoIterator<Item = &<X as IntoIterator>::Item>`

then our collection will automatically implement `Collection`; and hence, its reference will implement `Iterable`.

<br />
<details>
<summary style="font-weight:bold;">Example Custom Collection</summary>

Consider the following collection of numbers, iterators of which yield first even numbers then odds. We implement `IntoIterator` on the type and on its reference. This qualifies `EvenThenOdds` as a `Collection` and provides the `iter` method.

```rust
use orx_iterable::*;

#[derive(Default)]
pub struct EvensThenOdds {
    pub evens: Vec<i32>,
    pub odds: Vec<i32>,
}

impl EvensThenOdds {
    fn push(&mut self, number: i32) {
        match number % 2 == 0 {
            true => self.evens.push(number),
            false => self.odds.push(number),
        }
    }
}

impl IntoIterator for EvensThenOdds {
    type Item = i32;
    type IntoIter = core::iter::Chain<std::vec::IntoIter<i32>, std::vec::IntoIter<i32>>;
    fn into_iter(self) -> Self::IntoIter {
        self.evens.into_iter().chain(self.odds.into_iter())
    }
}

impl<'a> IntoIterator for &'a EvensThenOdds {
    type Item = &'a i32;
    type IntoIter = core::iter::Chain<core::slice::Iter<'a, i32>, core::slice::Iter<'a, i32>>;
    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter().chain(self.odds.iter())
    }
}

let mut numbers = EvensThenOdds::default();
numbers.push(4);
numbers.push(7);
numbers.push(2);

assert_eq!(numbers.iter().collect::<Vec<_>>(), [&4, &2, &7]);
```
</details>
<br />

### D.2. Custom CollectionMut

If our new collection allows both immutable and mutable iterations, we need to additionally provide the following implementation:

* `&mut X: IntoIterator<Item = &mut <X as IntoIterator>::Item>`

Then our collection will automatically implement `CollectionMut`.

<br />
<details>
<summary style="font-weight:bold;">Example Custom CollectionMut</summary>

Consider the following custom collection which keeps at most four largest numbers. In this case, we provide three of the `IntoIterator` implementations and `Top4` automatically implements `Collection` and `CollectionMut`.

```rust
use orx_iterable::*;

#[derive(Default)]
pub struct Top4 {
    numbers: Vec<i32>,
}

impl Top4 {
    fn push(&mut self, number: i32) {
        match self.numbers.len() < 4 {
            true => self.numbers.push(number),
            false => {
                let (i, &min) = self
                    .numbers
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, x)| *x)
                    .unwrap();
                if min < number {
                    self.numbers.remove(i);
                    self.numbers.push(number);
                }
            }
        }
    }
}

impl IntoIterator for Top4 {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Top4 {
    type Item = &'a i32;
    type IntoIter = core::slice::Iter<'a, i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.numbers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Top4 {
    type Item = &'a mut i32;
    type IntoIter = core::slice::IterMut<'a, i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.numbers.iter_mut()
    }
}

let mut numbers = Top4::default();
numbers.push(4);
numbers.push(7);
numbers.push(1);
numbers.push(3);
numbers.push(2);

assert_eq!(numbers.iter().collect::<Vec<_>>(), [&4, &7, &3, &2]);

for x in numbers.iter_mut() {
    *x += 1;
}

assert_eq!(numbers.iter().collect::<Vec<_>>(), [&5, &8, &4, &3]);
```
</details>
<br />

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-iterable/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
