use super::error::ParseError;
use super::txt;
use crate::token::Span;
use crate::traits::Hashing;
use crate::types::{BlockHeading, Text};
use chumsky::prelude::*;
use std::hash::Hash;

impl BlockHeading {
    #[cfg(not(feature = "hashing"))]
    pub fn new(span: Span, level: usize, content: Text) -> Self {
        Self {
            span,
            level,
            content,
        }
    }

    #[cfg(feature = "hashing")]
    pub fn new(span: Span, level: usize, content: Text) -> Self {
        let hash = (&level, &content).hashing();
        Self {
            span,
            level,
            content,
            hash,
        }
    }
}

#[cfg(feature = "hashing")]
impl Hash for BlockHeading {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, BlockHeading, Error = ParseError> {
    let token = just('#').repeated().at_least(1).map(|c| c.len());

    token
        .padded()
        .then(txt::parser_until(text::newline().rewind()))
        .map_with_span(|(level, content), span| BlockHeading::new(span, level, content))
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn heading_parse() {
    //     assert_eq!(
    //         parser().parse("### Hello World\n\n").unwrap(),
    //         BlockHeading {
    //             span: 0..15,
    //             content: Text {
    //                 span: 4..15,
    //                 content: "Hello World".to_string(),
    //             },
    //             level: 3
    //         }
    //     )
    // }
}
