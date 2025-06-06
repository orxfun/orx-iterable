use orx_iterable::*;

fn test_it(values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iter().count(), count);
    assert_eq!(col.iter().sum::<usize>(), sum);
}

#[test]
fn flattened() {
    test_it(
        vec![1, 4, 7, 11, 3, 8],
        vec![vec![0, 3], vec![6], vec![10, 2, 7]]
            .mapped(|x| x.iter().map(|x| x + 1))
            .flattened(),
    );

    let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
    let indices = vec![0, 2, 3];
    assert_eq!(
        indices
            .mapped(|idx| &data[*idx])
            .flattened()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 4, 2, 8, 8, 3]
    );
    assert_eq!(
        indices
            .mapped(|idx| &data[*idx])
            .flattened()
            .copied()
            .iter()
            .collect::<Vec<_>>(),
        vec![1, 4, 2, 8, 8, 3]
    );
}

#[test]
fn into_flattened() {
    let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
    let mut col = data.into_flattened();
    for x in col.iter_mut() {
        *x -= 1;
    }
    assert_eq!(
        col.iter().copied().collect::<Vec<_>>(),
        vec![0, 332, 3, 1, 7, 7, 2, 999]
    );
}

#[test]
fn flattened_mut() {
    let mut data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
    let mut col = data.flattened_mut();
    for x in col.iter_mut() {
        *x -= 1;
    }
    assert_eq!(
        col.iter().copied().collect::<Vec<_>>(),
        vec![0, 332, 3, 1, 7, 7, 2, 999]
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
    fn obj_flattened() {
        obj_test_it(
            vec![1, 4, 7, 11, 3, 8],
            &vec![vec![0, 3], vec![6], vec![10, 2, 7]]
                .mapped(|x| x.iter().map(|x| x + 1))
                .flattened(),
        );

        let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
        let indices = vec![0, 2, 3];
        assert_eq!(
            indices
                .mapped(|idx| &data[*idx])
                .flattened()
                .boxed_iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![1, 4, 2, 8, 8, 3]
        );
        assert_eq!(
            indices
                .mapped(|idx| &data[*idx])
                .flattened()
                .copied()
                .boxed_iter()
                .collect::<Vec<_>>(),
            vec![1, 4, 2, 8, 8, 3]
        );
    }

    #[test]
    fn obj_into_flattened() {
        let data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
        let mut col = data.into_flattened();
        for x in col.boxed_iter_mut() {
            *x -= 1;
        }
        assert_eq!(
            col.boxed_iter().copied().collect::<Vec<_>>(),
            vec![0, 332, 3, 1, 7, 7, 2, 999]
        );
    }

    #[test]
    fn obj_flattened_mut() {
        let mut data = vec![vec![1], vec![333], vec![4, 2], vec![8, 8, 3], vec![1000]];
        let mut col = data.flattened_mut();
        for x in col.boxed_iter_mut() {
            *x -= 1;
        }
        assert_eq!(
            col.boxed_iter().copied().collect::<Vec<_>>(),
            vec![0, 332, 3, 1, 7, 7, 2, 999]
        );
    }
}
