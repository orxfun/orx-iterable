pub type BoxedIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

struct CheckObjectSafety {
    obj: Box<dyn IterableObj<Item = usize>>,
}

pub trait IterableObj {
    type Item;

    fn boxed_iterator(&self) -> BoxedIter<&Self::Item>;

    // provided
    fn filtered<F>(self, filter: F) -> Filtered<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            filter,
            source: self,
        }
    }
}

impl<X: IterableObj> IterableObj for &X {
    type Item = X::Item;

    fn boxed_iterator(&self) -> BoxedIter<&Self::Item> {
        <X as IterableObj>::boxed_iterator(self)
    }
}

impl<X: IterableObj> IterableObj for &mut X {
    type Item = X::Item;

    fn boxed_iterator(&self) -> BoxedIter<&Self::Item> {
        <X as IterableObj>::boxed_iterator(self)
    }
}

impl<'a, T> IterableObj for std::slice::Iter<'a, T> {
    type Item = T;

    fn boxed_iterator(&self) -> BoxedIter<&Self::Item> {
        Box::new(self.clone())
    }
}

pub struct Filtered<I, F>
where
    I: IterableObj,
    F: Fn(&I::Item) -> bool,
{
    source: I,
    // filter: fn(&I::Item) -> bool,
    filter: F,
}

impl<I, F> IterableObj for Filtered<I, F>
where
    I: IterableObj,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn boxed_iterator(&self) -> BoxedIter<&Self::Item> {
        Box::new(self.source.boxed_iterator().filter(|x| (self.filter)(x)))
    }
}
