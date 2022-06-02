use super::error::ParseError;
use super::inline;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{BlockParagraph, Inline};
use chumsky::prelude::*;
use std::hash::Hash;

impl BlockParagraph {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Inline) -> Self {
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Inline) -> Self {
        let hash = content.hashing();
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for BlockParagraph {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, BlockParagraph, Error = ParseError> {
    inline::parser().map_with_span(|content, span| BlockParagraph::new(span, content))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
