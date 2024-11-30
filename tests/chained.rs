use orx_iterable::*;

// IterableRef & IterableMut

fn check_sum(iter: &impl IterableRef<Item = usize>, sum: usize) {
    assert_eq!(iter.iter().sum::<usize>(), sum);
    assert_eq!(iter.iter().sum::<usize>(), sum);
}

fn add_3_check_sum(iter: &mut impl IterableMut<Item = usize>, sum: usize) {
    let count = iter.iter().count();
    let sum = sum + 3 * count;

    for x in iter.iter_mut() {
        *x += 1;
    }
    for x in iter.iter_mut() {
        *x += 2;
    }

    assert_eq!(iter.iter().sum::<usize>(), sum);
    assert_eq!(iter.iter().sum::<usize>(), sum);
}

#[test]
fn chained() {
    let mut a = vec![3, 2];
    let mut b = vec![7, 1, 8, 6];
    let mut c = vec![10];

    check_sum(&a.chained(&b), 27);
    check_sum(&a.chained(&b).chained(&c), 37);

    add_3_check_sum(&mut a.chained_mut(&mut b).chained_mut(&mut c), 37);
    assert_eq!(a, vec![6, 5]);
    assert_eq!(b, vec![10, 4, 11, 9]);
    assert_eq!(c, vec![13]);
}

// Iterable

fn check_sum_it<'a>(iter: impl Iterable<ItemVal = &'a usize>, sum: usize) {
    assert_eq!(iter.iter_val().sum::<usize>(), sum);
    assert_eq!(iter.iter_val().sum::<usize>(), sum);
}

fn add_3_check_sum_it<'a>(mut iter: impl Iterable<ItemVal = &'a mut usize>, sum: usize) {
    let count = iter.iter_val().count();
    let sum = sum + 3 * count;

    for x in iter.iter_val() {
        *x += 1;
    }
    for x in iter.iter_val() {
        *x += 2;
    }

    assert_eq!(iter.iter_val().map(|x| *x).sum::<usize>(), sum);
    assert_eq!(iter.iter_val().map(|x| *x).sum::<usize>(), sum);
}

#[test]
fn chained_it() {
    let mut a = vec![3, 2];
    let mut b = vec![7, 1, 8, 6];
    let mut c = vec![10];

    check_sum_it(&a, 5);
    check_sum_it(&b, 22);
    check_sum_it(&c, 10);
}
