use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn mapped() {
    let values = || vec![1, 3, 7];

    test_it(values(), vec![2u32, 6, 14].mapped(|x| *x as usize / 2));

    test_it(values(), [2, 6, 14].copied().mapped(|x| x / 2));

    test_it(
        values(),
        [2, 6, 14].copied().mapped(|x| x * 2).mapped(|x| x / 4),
    );
}

#[cfg(feature = "std")]
mod object_safe {
    use orx_iterable::{obj_safe::*, *};

    fn obj_test_it(values: Vec<usize>, col: &dyn IterableObj<Item = usize>) {
        let sum = values.iter().sum::<usize>();
        let count = values.len();

        // tests
        assert_eq!(col.boxed_iter().count(), count);
        assert_eq!(col.boxed_iter().sum::<usize>(), sum);
    }

    #[test]
    fn obj_mapped() {
        let values = || vec![1, 3, 7];

        obj_test_it(values(), &vec![2u32, 6, 14].mapped(|x| *x as usize / 2));

        obj_test_it(values(), &[2, 6, 14].copied().mapped(|x| x / 2));

        obj_test_it(
            values(),
            &[2, 6, 14].copied().mapped(|x| x * 2).mapped(|x| x / 4),
        );
    }
}
