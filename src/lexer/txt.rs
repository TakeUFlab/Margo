use std::hash::Hash;

use chumsky::prelude::*;

use crate::token::Span;
use crate::traits::Hashing;
use crate::types::Text;

use super::error::{ParseError, ParseLabel};

impl Text {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, content: String) -> Self {
        Self { span, content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, content: String) -> Self {
        let hash = content.hashing();
        Self {
            span,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for Text {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser_until<T, K>(t: T) -> impl Parser<char, Text, Error = ParseError>
where
    T: Parser<char, K, Error = ParseError>,
{
    take_until(t)
        .try_map(|(chars, _), span| {
            (!chars.is_empty())
                .then(|| chars)
                .ok_or_else(|| ParseError::new(span).with_label(ParseLabel::CannotEmpty))
        })
        .collect()
        .map_with_span(|content, span| Text::new(span, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        dbg!(any::<char, ParseError>().parse("aaaa").unwrap());
    }
}
