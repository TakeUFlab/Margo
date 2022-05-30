use std::ops::Range;

#[cfg(feature = "serde")]
use serde::Serialize;

pub type Span = Range<usize>;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Token {
    HeadingTag(String, Span),
    InlinePair(String, Span),
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Token::HeadingTag(_, span) | Token::InlinePair(_, span) => span.clone(),
        }
    }
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1.clone()
    }
}
