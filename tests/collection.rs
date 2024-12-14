mod common_testers;
mod custom_iterables;
use common_testers::{obj_test_col, test_col};
use std::collections::{LinkedList, VecDeque};

#[test]
fn std_collections() {
    let values = || vec![1, 3, 7];

    test_col(values(), [1, 3, 7]);
    test_col(values(), vec![1, 3, 7]);
    test_col(values(), VecDeque::from_iter([1, 3, 7].into_iter()));
    test_col(values(), LinkedList::from_iter([1, 3, 7].into_iter()));

    test_col(vec![10], Some(10));
    test_col(vec![], None);

    test_col(vec![10], Result::<_, String>::Ok(10));
    test_col(vec![], Result::<_, String>::Err("error".to_string()));
}

#[test]
fn custom_collection() {
    let col = custom_iterables::EvensThenOddsCol {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    test_col(vec![4, 12, 8, 2, 1, 7], col);
}

// obj

#[test]
fn obj_std_collections() {
    let values = || vec![1, 3, 7];

    obj_test_col(values(), &[1, 3, 7]);
    obj_test_col(values(), &vec![1, 3, 7]);
    obj_test_col(values(), &VecDeque::from_iter([1, 3, 7].into_iter()));
    obj_test_col(values(), &LinkedList::from_iter([1, 3, 7].into_iter()));
}

#[test]
fn obj_custom_collection() {
    let col = custom_iterables::EvensThenOddsCol {
        evens: vec![4, 12, 8, 2],
        odds: vec![1, 7],
    };

    obj_test_col(vec![4, 12, 8, 2, 1, 7], &col);

    obj_test_col(vec![10], &Some(10));
    obj_test_col(vec![], &None);

    obj_test_col(vec![10], &Result::<_, String>::Ok(10));
    obj_test_col(vec![], &Result::<_, String>::Err("error".to_string()));
}
