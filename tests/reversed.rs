use orx_iterable::*;

fn test_it(mut values: Vec<usize>, col: impl Iterable<Item = usize>) {
    let sum = values.iter().sum::<usize>();
    let count = values.len();

    // tests
    assert_eq!(col.iterate().count(), count);
    assert_eq!(col.iterate().sum::<usize>(), sum);

    values.reverse();
    assert_eq!(values, col.iterate().collect::<Vec<_>>());
}

#[test]
fn reversed() {
    let a = vec![1, 3, 7, 2, 8];
    test_it(vec![1, 3, 7, 2, 8], a.reversed().copied());
}

#[test]
fn reversed_mut() {
    let mut a = vec![1, 3, 7, 2, 8];
    for x in a.reversed_mut().iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.reversed().copied());
}

#[test]
fn into_reversed() {
    let a = vec![1, 3, 7, 2, 8];
    let mut a = a.into_reversed();
    for x in a.iter_mut() {
        *x += 10;
    }
    test_it(vec![11, 13, 17, 12, 18], a.copied());
}

#[test]
fn abc() {
    use orx_iterable::*;

    #[derive(Default)]
    pub struct Top4 {
        numbers: Vec<i32>,
    }

    impl Top4 {
        fn push(&mut self, number: i32) {
            match self.numbers.len() < 4 {
                true => self.numbers.push(number),
                false => {
                    let (i, &min) = self
                        .numbers
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, x)| *x)
                        .unwrap();
                    if min < number {
                        self.numbers.remove(i);
                        self.numbers.push(number);
                    }
                }
            }
        }
    }

    impl IntoIterator for Top4 {
        type Item = i32;
        type IntoIter = std::vec::IntoIter<i32>;
        fn into_iter(self) -> Self::IntoIter {
            self.numbers.into_iter()
        }
    }

    impl<'a> IntoIterator for &'a Top4 {
        type Item = &'a i32;
        type IntoIter = core::slice::Iter<'a, i32>;
        fn into_iter(self) -> Self::IntoIter {
            self.numbers.iter()
        }
    }

    impl<'a> IntoIterator for &'a mut Top4 {
        type Item = &'a mut i32;
        type IntoIter = core::slice::IterMut<'a, i32>;
        fn into_iter(self) -> Self::IntoIter {
            self.numbers.iter_mut()
        }
    }

    let mut numbers = Top4::default();
    numbers.push(4);
    numbers.push(7);
    numbers.push(1);
    numbers.push(3);
    numbers.push(2);

    assert_eq!(numbers.iter().collect::<Vec<_>>(), [&4, &7, &3, &2]);

    for x in numbers.iter_mut() {
        *x += 1;
    }

    assert_eq!(numbers.iter().collect::<Vec<_>>(), [&5, &8, &4, &3]);
}

#[test]
fn def() {
    use orx_iterable::*;

    #[derive(Default)]
    pub struct EvensThenOdds {
        pub evens: Vec<i32>,
        pub odds: Vec<i32>,
    }

    impl EvensThenOdds {
        fn push(&mut self, number: i32) {
            match number % 2 == 0 {
                true => self.evens.push(number),
                false => self.odds.push(number),
            }
        }
    }

    impl<'a> IntoIterator for &'a EvensThenOdds {
        type Item = &'a i32;
        type IntoIter = core::iter::Chain<core::slice::Iter<'a, i32>, core::slice::Iter<'a, i32>>;
        fn into_iter(self) -> Self::IntoIter {
            self.evens.iter().chain(self.odds.iter())
        }
    }

    let mut numbers = EvensThenOdds::default();
    numbers.push(4);
    numbers.push(7);
    numbers.push(2);

    // numbers.iter_it();

    assert_eq!(numbers.iter2().collect::<Vec<_>>(), [&4, &2, &7]);
    assert_eq!((&numbers).iterate().collect::<Vec<_>>(), [&4, &2, &7]);
}
