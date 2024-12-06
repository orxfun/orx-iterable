# orx-iterable

[![orx-iterable crate](https://img.shields.io/crates/v/orx-iterable.svg)](https://crates.io/crates/orx-iterable)
[![orx-iterable documentation](https://docs.rs/orx-iterable/badge.svg)](https://docs.rs/orx-iterable)

Iterable and IterableCol traits to define types which can be iterated over multiple times.

## Motivation

There exists many situations where we need to iterate over an abstract type multiple times. For a very simple example, consider the following `statistics` method.

```rust
struct Stats {
    count: usize,
    mean: i64,
    std_dev: i64,
}

fn statistics(numbers: &[i64]) -> Stats {
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
```

Here, we used the slice as the way to abstract over the actual data type. However, this is too limiting. For instance, the following relevant and actually fitting concrete types cannot be used with this method:
* Collections that cannot be represented as a single contagious block of memory, such as `HashSet`, `VecDeque` or `LinkedList` or `SplitVec`.
* `Range<i64>`
* `vec.iter().filter(|x| *x < 100)` where `vec` is `Vec<i64>`

This crate defines iterable traits so that the following abstraction is possible.

```rust
use orx_iterable::*;
use std::collections::{VecDeque, LinkedList};

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
```

## Missing Traits

Iterable traits seem to be from the standard library.

First, we have the `Iterator` trait which builds extremely useful functionalities around its core method `next`. We can iterate over elements of an iterator once.

```rust ignore
fn next(&mut self) -> Option<Self::Item>`;
```

Second, we have the `IntoIterator` trait which transforms an instance of its type into an iterator through the trait method `into_iter`. This method consumes the instance of the type to create the corresponding iterator, which again can be iterated over once.

```rust ignore
fn into_iter(self) -> Self::IntoIter;
```

**How can we iterate over a type multiple times?**

We call `iter(&self)` or `iter_mut(&mut self)` as many times as we want. 

However, these are not trait methods. This is nothing but a nice **convention** that the standard library follows. A much stronger design would be through defining this common behavior through corresponding traits.

Luckily, rust type system is powerful ❤️🦀 to introduce these missing traits for types existed and to ever exist.

This create defines two traits:

* `Iterable`: an iterable type implements the `iter` method allowing to create an immutable iterator from a shared reference.
  * If `X` implements `Iterable<Item = T>` then,
  * `iter` returns `impl Iterator<Item = T>`.
* `IterableCol`: an iterable collection owns its elements and implements both the `iter` and `iter_mut` methods to create immutable and mutable iterators.
  * If `X` implements `Iterable<Item = T>` then,
  * `iter` returns `impl Iterator<Item = &T>`, and
  * `iter_mut` returns `impl Iterator<Item = &mut T>`.

## Iterable

Iterables implement the `iter` method which returns a new iterator every time it is called. Created iterators yield elements of type `Item` which can be a reference to a stored element or a value created during iteration.

We can investigate the iterables in three categories.

### A. Collections as Iterable

Let `X` be a collection storing every one of its elements of type `T`, such as a vector, a set or a linked list. Then, `&X` implements `Iterable<Item = &T>`.

The formal requirement for collections to implement Iterable is as simple as the following:

```rust ignore
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
```

Therefore, if we are implementing a new collection `X` and if we implement `IntoIterator` on `&X`, the `iter` method will readily be available for our collection.

This condition is satisfied by std collections, as well as, many collections outside the standard library, such as `SmallVec`, `ArrayVec` or `SplitVec` to name a few.

This generic implementation allows for passing collections to polymorphic functions accepting any iterable.

```rust
use orx_iterable::*;
use std::collections::{HashSet, LinkedList, VecDeque};

fn process_names<'a>(names: impl Iterable<Item = &'a String>) { /* ... */ }

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

### B. Cloneable Iterators

An iterator is not limited to visiting all elements of a collection. Due to ergonomic chaining methods such as `filter` or `map` that transforms one iterator to another, iterators often hold a definition of a computation.

It would be awesome if we could use such an iterator multiple times. This is also simply possible in rust. We define `CloningIterable` which is a wrapper around an iterator, and returns a clone of this iterator whenever `iter` is called. We can transform any cloneable iterator into an `Iterable` by calling `into_iterable` method.

```rust ignore
impl<I> IntoCloningIterable for I
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    fn into_iterable(self) -> CloningIterable<<Self as IntoIterator>::IntoIter> {
        CloningIterable(self.into_iter())
    }
}
```

Consider the `process_names` function in the example above, and assume we have a collection of names; however, we want to filter the names to be processed. One way to achieve this is to define the filtered iterator and convert it into an iterable as demonstrated below.

```rust
use orx_iterable::*;

fn process_names<'a>(names: impl Iterable<Item = &'a String>) { /* ... */ }

let names = vec![String::from("xox"), String::from("oxo")];

// names.iter().filter(|x| x.starts_with('x'))
// is the iterator, the definition, that we want to create multiple times
// so we convert it into an iterable
let filtered_names = names.iter().filter(|x| x.starts_with('x')).into_iterable();

process_names(filtered_names);
```

### C. Element Producing Iterables

Some iterators yield elements which are created on the fly, rather than being read from a memory location. A very common example is the range, `Range<usize>` for instance. Consider the range `3..7`. Although it seems like a collection, it does not hold elements (3, 4, 5, 6) anywhere in memory. These elements are produced on the fly during the iteration. Such types need to explicitly implement the `Iterable` trait.

```rust
use orx_iterable::*;

let range = 3..7usize;

assert_eq!(range.iter().max(), Some(6));
assert_eq!(range.iter().sum::<usize>(), 18);
assert_eq!(range.iter().product::<usize>(), 360);
```

## IterableCol

IterableCol has a more restrictive definition:
* An iterable collection is a collection that stores each of its elements and is able to produce shared and mutable references for them.
  * Notice that element producing iterables such as the `Range<usize>` does not satisfy this requirement.
  * Furthermore, cloneable iterators that produce elements on the fly, such as `numbers.iter().map(|x| x + 1)`, does not satisfy it either.
* Type of its elements is `Item`:
  * the `iter` method yields `&Item`, and
  * the `iter_mut` method yields `&mut Item`.

Any collection type `X` satisfying the following conditions automatically implements `IterableCol`:
* `X: IntoIterator`
* `&X: IntoIterator<Item = &<X as IntoIterator>::Item>`
* `&mut X: IntoIterator<Item = &mut <X as IntoIterator>::Item>`

Therefore, if we are implementing a new collection X and if we implement IntoIterator on X, &X and &mut X, the `iter` and `iter_mut` methods will readily be available.

These conditions are satisfied by std collections, as well as, many collections outside the standard library, such as `SmallVec`, `ArrayVec` or `SplitVec` to name a few.

Notice that the second condition is almost identical to the required condition for `Iterable`. Therefore, relation among these two traits is as follows.
* If `X` implements `IterableCol<Item = T>`, then `&X` implements `Iterable<Item = &T>`.
* However, an Iterable type is not necessarily a collection and does not need to implement `IterableCol`.

## Chainable Transformations

The standard `Iterator` trait provides a wide variety of methods which transforms one iterator into another, such as `filter`, `map` or `flat_map`. Furthermore, these transformations can ergonomically be chained to create views defined by lazy computations.

`Iterable` and `IterableCol` traits follow the same design and provide these chainable transformation methods.

```rust
use orx_iterable::*;

let data = vec![3, 7, 11, 8, 2, 5];

let it = data
    .copied()
    .filtered(|x| x % 2 == 1)
    .flat_mapped(|x| [x, -x]);

assert_eq!(it.iter().count(), 8);
assert_eq!(it.iter().sum::<i32>(), 0);
```
