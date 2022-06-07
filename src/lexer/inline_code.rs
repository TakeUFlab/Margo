use super::{error::ParseError, txt};
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::InlineCode;
use crate::types::Text;
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineCode {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Text) -> Self {
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Text) -> Self {
        let hash = ("inline code", &content).hashing();
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, InlineCode, Error = ParseError> {
    just(" `")
        .ignore_then(txt::parser_until(just("` ")))
        .map_with_span(|content, span| InlineCode::new(span, content))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
