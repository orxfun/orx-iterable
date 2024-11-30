// implicitly implements IteratorCol<usize> + Iterator<&usize>

pub struct EvensThenOddsCol {
    pub evens: Vec<usize>,
    pub odds: Vec<usize>,
}

impl IntoIterator for EvensThenOddsCol {
    type Item = usize;

    type IntoIter = core::iter::Chain<std::vec::IntoIter<usize>, std::vec::IntoIter<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.into_iter().chain(self.odds.into_iter())
    }
}

impl<'a> IntoIterator for &'a EvensThenOddsCol {
    type Item = &'a usize;

    type IntoIter = core::iter::Chain<core::slice::Iter<'a, usize>, core::slice::Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter().chain(self.odds.iter())
    }
}

impl<'a> IntoIterator for &'a mut EvensThenOddsCol {
    type Item = &'a mut usize;

    type IntoIter =
        core::iter::Chain<core::slice::IterMut<'a, usize>, core::slice::IterMut<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter_mut().chain(self.odds.iter_mut())
    }
}

// implicitly implements Iterator<&usize>

pub struct EvensThenOdds {
    pub evens: Vec<usize>,
    pub odds: Vec<usize>,
}

impl<'a> IntoIterator for &'a EvensThenOdds {
    type Item = &'a usize;

    type IntoIter = core::iter::Chain<core::slice::Iter<'a, usize>, core::slice::Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.evens.iter().chain(self.odds.iter())
    }
}
