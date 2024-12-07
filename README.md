# orx-iterable

[![orx-iterable crate](https://img.shields.io/crates/v/orx-iterable.svg)](https://crates.io/crates/orx-iterable)
[![orx-iterable documentation](https://docs.rs/orx-iterable/badge.svg)](https://docs.rs/orx-iterable)

Iterable and IterableCol traits to generalize types which can be iterated over multiple times.

## Motivation

There exist numerous situations where we need to iterate over an abstract type multiple times.

For a very simple example, consider a method that computes certain statistics of an iterable of numbers. The `Iterable` trait defines the shared iterable behavior so that the following abstraction is available.

```rust
use orx_iterable::*;
use std::collections::{VecDeque, LinkedList};

struct Stats {
    count: usize,
    mean: i64,
    std_dev: i64,
}

/// we need multiple iterations over numbers to compute the stats
/// * we can compute count & sum in one go
/// * but we need the second iteration at least for std_dev
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

let numbers = vec![3, 5, 7, 10, 2, 11];
statistics(numbers.copied());

let evens = numbers.filtered(|x| *x % 2 == 0);
statistics(evens.copied());

let doubles = numbers.mapped(|x| 2 * x);
statistics(doubles);

let numbers: LinkedList<i64> = [3, 5, 7].into_iter().collect();
statistics(numbers.copied());

let numbers = 7..20i64;
statistics(numbers);

let numbers = (0..4)
    .into_iter()
    .flat_map(|x| [-2 * x, x, 2 * x + 1])
    .into_iterable();
statistics(numbers);
```

Furthermore, a more restrictive and stronger `IterableCol` trait is defined to additionally allow for mutable iterations using `iter_mut`.

```rust
use orx_iterable::*;
use std::collections::{LinkedList, VecDeque};

/// first computes sum, and then adds it to each of the elements
fn increment_by_sum(numbers: &mut impl IterableCol<Item = i32>) {
    let sum: i32 = numbers.iter().sum();

    for x in numbers.iter_mut() {
        *x += sum;
    }
}

let mut vec = vec![1, 2, 3];
increment_by_sum(&mut vec);
assert_eq!(vec, [7, 8, 9]);

let mut vec_deq = VecDeque::from_iter([1, 2, 3]);
increment_by_sum(&mut vec_deq);
assert_eq!(vec_deq, VecDeque::from_iter([7, 8, 9]));

let mut list = LinkedList::from_iter([1, 2, 3]);
increment_by_sum(&mut list);
assert_eq!(list, LinkedList::from_iter([7, 8, 9]));
```

## Iterable Traits

Currently, the standard library is lacking the iterable traits.

This create introduces `Iterable` and IterableCol` traits. Thanks to the powerful rust type system ❤️🦀, these traits are automatically implemented for relevant types that are defined and to be defined.

* `Iterable` => `iter`
  * an iterable can be any type that can create iterators repeatedly.
  * lightweight & more inclusive.
* `IterableCol` => `iter + iter_mut`
  * an iterable collection which stores the elements that it yields.
  * more restrictive & more powerful.

### A. Iterable

> An `Iterable` is any type which can return a new iterator that yields elements of the associated type `Item` every time `iter` method is called.

Note that this is the core and least restrictive iterable definition which represents a wide range of types. We can investigate types implementing Iterable in three categories:

* collections
* cloneable iterators
* element producing iterables

#### A.1. Collections as Iterable

Let `X` be a collection type storing elements of type `T`, such as a vector, a set or a linked list. Then, `&X` implements `Iterable<Item = &T>`.

The auto implementation requirement is as follows:

```rust ignore
&X: IntoIterator<Item = &T> =====> &X: Iterable<Item = &T>
```

Note that this implication is straightforward: provided that `&self` can be converted into an iterator, then we can define `iter(&self)` method returning the same iterator.

This condition is satisfied by std collections, as well as, many collections outside the standard library, such as `SmallVec`, `ArrayVec` or `SplitVec` to name a few. You may see some examples below.

TODO: link new collection

```rust
use orx_iterable::*;
use std::collections::{HashSet, LinkedList, VecDeque};

/// fn requiring multiple immutable iterations over names
fn process_names<'a>(names: impl Iterable<Item = &'a String>) { }

// we can call it with (presumably) all collections

let names = [String::from("xox"), String::from("oxo")];
process_names(&names);

let names = vec![String::from("xox"), String::from("oxo")];
process_names(&names);

let names: HashSet<_> = [String::from("xox")].into_iter().collect();
process_names(&names);

let names: LinkedList<_> = [String::from("xox")].into_iter().collect();
process_names(&names);

let names: VecDeque<_> = [String::from("xox")].into_iter().collect();
process_names(&names);
```

#### A.2. Cloneable Iterators

An iterator is not limited to visiting elements of a collection. Thanks to chainable methods transforming one iterator to another, such as `filter` or `map`, iterators often hold a definition of a computation over some data.

It would be awesome if we could use such an iterator multiple times.

This is also conveniently possible.

Any iterator that can be cloned (`I: Iterator + Clone`) can be converted into an iterable by simply calling the `into_iterable` method.

Consider the generic `process_names` function in the example above. This time we want to call it using a collection of names; however, we want to filter the names to be processed. One way to achieve this is to define the filtered iterator and convert it into an iterable as demonstrated below.

```rust
use orx_iterable::*;

/// fn requiring multiple immutable iterations over names
fn process_names<'a>(names: impl Iterable<Item = &'a String>) { }

// the source data
let names = vec![String::from("xox"), String::from("oxo")];

// the iterator that we want to use multiple times
let iter = names.iter().filter(|x| x.starts_with('x'));

// so we convert it into an iterable
let filtered_names = iter.into_iterable();

process_names(filtered_names);
```

#### A.3. Element Producing Iterables

Some iterators yield elements which are created on the fly, rather than being read from a memory location.

Types creating such iterators also share the common behavior of the `Iterable` trait.

A common example is the range. Consider, for instance, the range `3..7`. Although it looks like a collection, it does not hold elements (3, 4, 5, 6) anywhere in memory. These elements are produced on the fly during the iteration.

```rust
use orx_iterable::*;

let range = 3..7usize;

assert_eq!(range.iter().max(), Some(6));
assert_eq!(range.iter().sum::<usize>(), 18);
assert_eq!(range.iter().product::<usize>(), 360);
```

As a second example, consider the custom iterator `FibUntilIter` which produces Fibonacci numbers until an upper bound. `FibUntil` struct can create this iterator any time `iter` is called, and hence, it is an Iterable.

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

### B. IterableCol

> An `IterableCol` is a collection that is able to produce iterators yielding shared and mutable references to its elements.

IterableCol has more restrictive requirements than Iterable. However, in addition to `iter`, iterable collections also allow multiple mutable iterations through the `iter_mut` method.

Any collection type `X` having elements of type `T` that satisfies the following conditions automatically implements `IterableCol`:
* `X: IntoIterator<Item = T>`
* `&X: IntoIterator<Item = &T>`
* `&mut X: IntoIterator<Item = &mut T>`

These conditions are satisfied by std collections, as well as, many collections outside the standard library, such as `SmallVec`, `ArrayVec` or `SplitVec` to name a few.

## Custom Iterables

In the previous section, we mentioned the wide range of types that already implement `Iterable` and `IterableCol` traits. This has been possible thanks to the joyful rust type system, consistent use of `IntoIterator` trait in the standard library and almost all collection crates that follow this nice design pattern.

What about the new types? We can discuss this in three cases: (i) immutable collections, (ii) mutable collections and (iii) others.

### Custom Immutable Collection

For custom collections, we should not need to implement iterable traits. Better approach is to provide the `IntoIterator` implementations, and iterable traits will be automatically implemented.

For an immutable collection, it suffices to implement `IntoIterator` for `&X` where `X` is our immutable collection.

```rust

```

### Custom Collection

For custom collections, we should not need to implement iterable traits. Better approach is to provide the `IntoIterator` implementations, and iterable traits will be automatically implemented.

Consider for instance a custom collection of numbers which yields evens first, odds later.

```rust
use orx_iterable::*;

pub struct EvensThenOdds {
    pub evens: Vec<usize>,
    pub odds: Vec<usize>,
}

impl IntoIterator for EvensThenOdds {
    type Item = usize;

    type IntoIter = core::iter::Chain<std::vec::IntoIter<usize>, std::vec::IntoIter<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.into_iter().chain(self.odds.into_iter())
    }
}

impl<'a> IntoIterator for &'a EvensThenOdds {
    type Item = &'a usize;

    type IntoIter = core::iter::Chain<core::slice::Iter<'a, usize>, core::slice::Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter().chain(self.odds.iter())
    }
}

impl<'a> IntoIterator for &'a mut EvensThenOdds {
    type Item = &'a mut usize;

    type IntoIter =
        core::iter::Chain<core::slice::IterMut<'a, usize>, core::slice::IterMut<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter_mut().chain(self.odds.iter_mut())
    }
}
```



## Chainable Transformations

`Iterator` trait provides a wide variety of methods which transforms one iterator into another, such as `filter`, `map` or `flat_map`. These transformations can nicely be chained to compose lazy computation definitions.

`Iterable` and `IterableCol` traits follow the same design and provide these chainable transformation methods.

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

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-iterable/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
