use super::error::ParseError;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{Inline, InlineBold};
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineBold {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Inline) -> Self {
        let content = Box::new(content);
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Inline) -> Self {
        let hash = ("bold", &content).hashing();
        let content = Box::new(content);
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineBold {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser<T>(r: T) -> impl Parser<char, InlineBold, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.delimited_by(just(" *"), just("* "))
        .map_with_span(|content, span| InlineBold::new(span, content))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
