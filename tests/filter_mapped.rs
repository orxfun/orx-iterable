use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn filter_mapped() {
    test_it(
        vec![1, 4, 7, 11],
        vec![1, 15, 4, 7, 33, 11].filter_mapped(|x| (*x < 12).then_some(*x)),
    );

    assert_eq!(
        vec![4.to_string()],
        [1, 4, 7, 11]
            .filter_mapped(|x| (*x % 2 == 0).then_some(x.to_string()))
            .iter()
            .collect::<Vec<_>>(),
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
    fn obj_filter_mapped() {
        obj_test_it(
            vec![1, 4, 7, 11],
            &vec![1, 15, 4, 7, 33, 11].filter_mapped(|x| (*x < 12).then_some(*x)),
        );

        assert_eq!(
            vec![4.to_string()],
            [1, 4, 7, 11]
                .filter_mapped(|x| (*x % 2 == 0).then_some(x.to_string()))
                .iter()
                .collect::<Vec<_>>(),
        );
    }
}
