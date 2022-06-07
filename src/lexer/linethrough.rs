use super::error::ParseError;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{Inline, InlineLinethrough};
use chumsky::prelude::*;
use std::hash::Hash;

impl InlineLinethrough {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Inline) -> Self {
        let content = Box::new(content);
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Inline) -> Self {
        let hash = ("linethrough", &content).hashing();
        let content = Box::new(content);
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for InlineLinethrough {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser<T>(r: T) -> impl Parser<char, InlineLinethrough, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineLinethrough::new(span, content))
        .delimited_by(just(" ~"), just("~ "))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
