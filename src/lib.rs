pub mod util {
    use std::iter;

    /// Iterator adapter to easily check if a given
    /// element is the last one. Uses peekable.
    pub struct Iter<I>
        where I: Iterator
    {
        iterator: iter::Peekable<I>
    }

    pub trait IdentifyLast: Iterator + Sized {
        fn identify_last(self) -> Iter<Self>;
    }

    impl<I> IdentifyLast for I
        where I: Iterator
    {
        fn identify_last(self) -> Iter<Self> {
            Iter { iterator: self.peekable() }
        }
    }

    impl<I> Iterator for Iter<I>
        where I: Iterator
    {
        type Item = (bool, I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            self.iterator.next().map(|e| (self.iterator.peek().is_none(), e))
        }
    }
}
