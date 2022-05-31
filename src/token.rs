use std::ops::Range;

#[cfg(feature = "serde")]
use serde::Serialize;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct Ident {
    pub content: String,
    pub span: Span,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1.clone()
    }
}
