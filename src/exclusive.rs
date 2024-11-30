pub trait Exclusive<T> {
    fn get_ref(&self) -> &T;

    fn get_mut(&mut self) -> &mut T;
}

impl<T> Exclusive<T> for T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> Exclusive<T> for &mut T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        self
    }
}
