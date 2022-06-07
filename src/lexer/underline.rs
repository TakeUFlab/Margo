use super::error::ParseError;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{Inline, InlineUnderline};
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineUnderline {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Inline) -> Self {
        let content = Box::new(content);
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Inline) -> Self {
        let hash = ("underline", &content).hashing();
        let content = Box::new(content);
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineUnderline {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser<T>(r: T) -> impl Parser<char, InlineUnderline, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineUnderline::new(span, content))
        .delimited_by(just(" _"), just("_ "))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
