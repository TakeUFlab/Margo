use std::ops::Range;

pub type Span = Range<usize>;

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1.clone()
    }
}
