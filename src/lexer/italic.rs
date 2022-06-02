use super::error::ParseError;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{Inline, InlineItalic};
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineItalic {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Inline) -> Self {
        let content = Box::new(content);
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Inline) -> Self {
        let hash = content.hashing();
        let content = Box::new(content);
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineItalic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser<T>(r: T) -> impl Parser<char, InlineItalic, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineItalic::new(span, content))
        .delimited_by(just(" /"), just("/ "))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
