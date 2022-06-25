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
        let hash = ("text", &content).hashing();
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
    take_until(t.labelled(ParseLabel::MissingEnd))
        .map_with_span(|(content, _), span| Text::new(span, String::from_iter(content)))
        .try_map(|t, span| {
            if t.content.is_empty() {
                Err(ParseError::new(span, ParseLabel::CannotEmpty))
            } else {
                Ok(t)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        dbg!(any::<char, ParseError>().parse("aaaa").unwrap());
    }
}
