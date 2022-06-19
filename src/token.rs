use std::ops::Range;

#[cfg(feature = "serde")]
use serde::Serialize;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct Ident {
    pub content: String,
    pub span: Span,
}
