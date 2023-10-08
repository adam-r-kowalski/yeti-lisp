pub struct PeekingTakeWhile<'a, I, P>
where
    I: Iterator,
{
    iter: &'a mut core::iter::Peekable<I>,
    predicate: P,
}

impl<I, P> Iterator for PeekingTakeWhile<'_, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(&mut self.predicate)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }

    #[inline]
    fn fold<B, F>(mut self, mut accum: B, mut f: F) -> B
    where
        F: FnMut(B, I::Item) -> B,
    {
        while let Some(x) = self.iter.next_if(&mut self.predicate) {
            accum = f(accum, x);
        }
        accum
    }
}

pub trait PeekableExt<I>: Iterator
where
    I: Iterator,
{
    fn peeking_take_while<P>(&mut self, predicate: P) -> PeekingTakeWhile<'_, I, P>
    where
        P: FnMut(&Self::Item) -> bool;
}

impl<I: Iterator> PeekableExt<I> for std::iter::Peekable<I> {
    #[inline]
    fn peeking_take_while<P>(&mut self, predicate: P) -> PeekingTakeWhile<'_, I, P>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        PeekingTakeWhile {
            iter: self,
            predicate,
        }
    }
}

