use super::error::ParseError;
use super::txt;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{InlineMath, Text};
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineMath {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Text) -> Self {
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Text) -> Self {
        let hash = ("inline maths", &content).hashing();
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineMath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, InlineMath, Error = ParseError> {
    just(" $")
        .ignore_then(txt::parser_until(just("$ ")))
        .map_with_span(|content, span| InlineMath::new(span, content))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
