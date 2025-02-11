#![cfg(feature = "std")]

use orx_iterable::{obj_safe::*, IntoCloningIterable};
use std::{
    collections::{BTreeSet, LinkedList, VecDeque},
    rc::Rc,
};

// collection holder of trait objects
// pro: simple type without generic parameters
// con: use of dynamic dispatch
struct CollectionHolder {
    values: Box<dyn IterableObj<Item = u32>>,
    chars: Rc<dyn CollectionObj<Item = char>>,
    names: Box<dyn CollectionMutObj<Item = String>>,
}

impl CollectionHolder {
    fn sum_values(&self) -> u32 {
        self.values.boxed_iter().sum()
    }

    fn join_chars(&self) -> String {
        self.chars.boxed_iter().collect()
    }

    fn append_to_names(&mut self, c: char) {
        for x in self.names.boxed_iter_mut() {
            x.push(c)
        }
    }

    fn second_name(&self) -> Option<&str> {
        self.names.boxed_iter().nth(1).map(|x| x.as_str())
    }
}

// test with different concrete field types

#[test]
fn fields_of_iterable_objects() {
    let values = Box::new(vec![1, 2, 3].into_iter().into_iterable());
    let chars = Rc::new(['x', 'y', 'z']);
    let names = Box::new(VecDeque::from_iter(["john".to_string(), "doe".to_string()]));

    let mut col = CollectionHolder {
        values,
        chars,
        names,
    };

    assert_eq!(col.sum_values(), 6);
    assert_eq!(col.join_chars(), "xyz".to_string());
    col.append_to_names('!');
    assert_eq!(col.second_name(), Some("doe!"));

    // alternatively

    let values = Box::new(1..4u32);
    let chars: Rc<BTreeSet<_>> = Rc::new(['x', 'y', 'z'].into_iter().collect());
    let names = Box::new(LinkedList::from_iter([
        "john".to_string(),
        "doe".to_string(),
    ]));
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
