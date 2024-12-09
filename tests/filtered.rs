mod common_testers;
use common_testers::{obj_test_col, obj_test_it, test_col, test_it};
use orx_iterable::obj_safe::*;
use orx_iterable::*;

#[test]
fn filtered() {
    let a = vec![1, 3, 4, 8, 10];

    test_it(vec![1, 3, 4, 8, 10], a.filtered(|x| **x < 100));
    test_it(vec![1, 3, 4], a.filtered(|x| **x < 5));
    test_it(vec![3, 4], a.filtered(|x| **x < 5 && **x > 1));
}

#[test]
fn into_filtered() {
    let a = vec![1, 3, 4, 8, 10];
    test_col(vec![1, 3, 4, 8, 10], a.into_filtered(|x| *x < 100));

    let a = vec![1, 3, 4, 8, 10];
    test_col(vec![1, 3, 4], a.into_filtered(|x| *x < 5));

    let a = vec![1, 3, 4, 8, 10];
    test_col(vec![3, 4], a.into_filtered(|x| *x < 5 && *x > 1));
}

#[test]
fn filtered_mut() {
    let mut a = vec![1, 3, 4, 8, 10];

    test_col(vec![1, 3, 4, 8, 10], a.filtered_mut(|x| *x < 100));
    test_col(vec![1, 3, 4], a.filtered_mut(|x| *x < 5));
    test_col(vec![3, 4], a.filtered_mut(|x| *x < 5 && *x > 1));

    for x in a.filtered_mut(|x| *x < 5 && *x > 1).iter_mut() {
        *x += 10;
    }
    for x in a.filtered_mut(|x| *x < 5 && *x > 1).iter_mut() {
        // none matches in the second run!
        *x += 10;
    }

    assert_eq!(a, vec![1, 13, 14, 8, 10]);
    test_it(vec![1, 13, 14, 8, 10], &a);
}

// obj

#[test]
fn obj_filtered() {
    let a = vec![1, 3, 4, 8, 10];

    obj_test_it(vec![1, 3, 4, 8, 10], &a.filtered(|x| **x < 100));
    obj_test_it(vec![1, 3, 4], &a.filtered(|x| **x < 5));
    obj_test_it(vec![3, 4], &a.filtered(|x| **x < 5 && **x > 1));
}

#[test]
fn obj_into_filtered() {
    let a = vec![1, 3, 4, 8, 10];
    obj_test_col(vec![1, 3, 4, 8, 10], &a.into_filtered(|x| *x < 100));

    let a = vec![1, 3, 4, 8, 10];
    obj_test_col(vec![1, 3, 4], &a.into_filtered(|x| *x < 5));

    let a = vec![1, 3, 4, 8, 10];
    obj_test_col(vec![3, 4], &a.into_filtered(|x| *x < 5 && *x > 1));
}

#[test]
fn obj_filtered_mut() {
    let mut a = vec![1, 3, 4, 8, 10];

    obj_test_col(vec![1, 3, 4, 8, 10], &a.filtered_mut(|x| *x < 100));
    obj_test_col(vec![1, 3, 4], &a.filtered_mut(|x| *x < 5));
    obj_test_col(vec![3, 4], &a.filtered_mut(|x| *x < 5 && *x > 1));

    for x in a.filtered_mut(|x| *x < 5 && *x > 1).boxed_iter_mut() {
        *x += 10;
    }
    for x in a.filtered_mut(|x| *x < 5 && *x > 1).boxed_iter_mut() {
        // none matches in the second run!
        *x += 10;
    }

    assert_eq!(a, vec![1, 13, 14, 8, 10]);
    obj_test_it(vec![1, 13, 14, 8, 10], &&a);
}
