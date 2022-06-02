use super::error::ParseError;
use super::{ident, txt};
use crate::token::{Ident, Span};
use crate::traits::Hashing;
use crate::types::{BlockCode, Text};
use chumsky::prelude::*;
use std::hash::Hash;

impl BlockCode {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: Text) -> Self {
        Self {
            span,
            content,
            lang,
        }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: Text, lang: Option<Ident>) -> Self {
        let hash = content.hashing();
        Self {
            span,
            content,
            lang,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for BlockCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, BlockCode, Error = ParseError> {
    let tag = just("```");
    tag.ignore_then(ident::parser().or_not())
        .then_ignore(text::newline())
        .then(txt::parser_until(tag))
        .map_with_span(|(lang, content), span| BlockCode::new(span, content, lang))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        dbg!(parser().parse_recovery_verbose("```\nHi\n```"));
        dbg!(parser().parse_recovery_verbose("```rust\nHi\n```"));
    }
}
