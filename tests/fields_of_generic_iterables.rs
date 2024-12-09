use orx_iterable::*;
use std::collections::{BTreeSet, LinkedList, VecDeque};

// collection holder of generic iterables and collections
// pro: no vtable, static dispatch
// con: complicated type with generic parameters
struct CollectionHolder<V, C, N>
where
    V: Iterable<Item = u32>,
    C: Collection<Item = char>,
    N: CollectionMut<Item = String>,
{
    values: V,
    chars: C,
    names: N,
}

impl<V, C, N> CollectionHolder<V, C, N>
where
    V: Iterable<Item = u32>,
    C: Collection<Item = char>,
    N: CollectionMut<Item = String>,
{
    fn sum_values(&self) -> u32 {
        self.values.iter().sum()
    }

    fn join_chars(&self) -> String {
        self.chars.iter().collect()
    }

    fn append_to_names(&mut self, c: char) {
        for x in self.names.iter_mut() {
            x.push(c)
        }
    }

    fn second_name(&self) -> Option<&str> {
        self.names.iter().nth(1).map(|x| x.as_str())
    }
}

// test with different concrete field types

#[test]
fn fields_of_generic_iterables() {
    let values = vec![1, 2, 3];
    let chars = ['x', 'y', 'z'];
    let names = VecDeque::from_iter(["john".to_string(), "doe".to_string()]);

    let mut col = CollectionHolder {
        values: values.copied(),
        chars,
        names,
    };

    assert_eq!(col.sum_values(), 6);
    assert_eq!(col.join_chars(), "xyz".to_string());
    col.append_to_names('!');
    assert_eq!(col.second_name(), Some("doe!"));

    // alternatively

    let values = 1..4u32;
    let chars: BTreeSet<_> = ['x', 'y', 'z'].into_iter().collect();
    let names = LinkedList::from_iter(["john".to_string(), "doe".to_string()]);
    let mut col = CollectionHolder {
        values,
        chars,
        names,
    };

    assert_eq!(col.sum_values(), 6);
    assert_eq!(col.join_chars(), "xyz".to_string());
    col.append_to_names('!');
    assert_eq!(col.second_name(), Some("doe!"));
}
