mod common_testers;
use common_testers::{obj_test_col, obj_test_it, test_col, test_it};
use orx_iterable::*;
use std::collections::VecDeque;

#[test]
fn chained() {
    let a = vec![1, 3, 4];
    let b = [8, 10];
    let c = VecDeque::from_iter([2, 7].into_iter());

    test_it(vec![1, 3, 4, 8, 10], a.chained(&b));
    test_it(vec![1, 3, 4, 8, 10, 2, 7], a.chained(&b).chained(&c));
    test_it(vec![1, 3, 4, 8, 10, 2, 7], a.chained(b.chained(&c)));
}

#[test]
fn into_chained() {
    let a = vec![1, 3, 4];
    let b = [8, 10];
    test_col(vec![1, 3, 4, 8, 10], a.into_chained(b));

    let a = vec![1, 3, 4];
    let b = [8, 10];
    let c = VecDeque::from_iter([2, 7].into_iter());
    test_col(
        vec![1, 3, 4, 8, 10, 2, 7],
        a.into_chained(b).into_chained(c),
    );
}

#[test]
fn chained_mut() {
    let mut a = vec![1, 3, 4];
    let mut b = [8, 10];
    test_col(vec![1, 3, 4, 8, 10], a.chained_mut(&mut b));

    let mut a = vec![1, 3, 4];
    let mut b = [8, 10];
    let mut c = VecDeque::from_iter([2, 7].into_iter());
    test_col(
        vec![1, 3, 4, 8, 10, 2, 7],
        a.chained_mut(&mut b).chained_mut(&mut c),
    );
}

// obj

#[test]
fn obj_chained() {
    let a = vec![1, 3, 4];
    let b = [8, 10];
    let c = VecDeque::from_iter([2, 7].into_iter());

    obj_test_it(vec![1, 3, 4, 8, 10], &a.chained(&b));
    obj_test_it(vec![1, 3, 4, 8, 10, 2, 7], &a.chained(&b).chained(&c));
    obj_test_it(vec![1, 3, 4, 8, 10, 2, 7], &a.chained(b.chained(&c)));
}

#[test]
fn obj_into_chained() {
    let a = vec![1, 3, 4];
    let b = [8, 10];
    obj_test_col(vec![1, 3, 4, 8, 10], &a.into_chained(b));

    let a = vec![1, 3, 4];
    let b = [8, 10];
    let c = VecDeque::from_iter([2, 7].into_iter());
    obj_test_col(
        vec![1, 3, 4, 8, 10, 2, 7],
        &a.into_chained(b).into_chained(c),
    );
}

#[test]
fn obj_chained_mut() {
    let mut a = vec![1, 3, 4];
    let mut b = [8, 10];
    obj_test_col(vec![1, 3, 4, 8, 10], &a.chained_mut(&mut b));

    let mut a = vec![1, 3, 4];
    let mut b = [8, 10];
    let mut c = VecDeque::from_iter([2, 7].into_iter());
    obj_test_col(
        vec![1, 3, 4, 8, 10, 2, 7],
        &a.chained_mut(&mut b).chained_mut(&mut c),
    );
}
