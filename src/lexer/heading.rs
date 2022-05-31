use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{BlockHeading, Text};

use super::error::ParseError;
use super::utils::{block_newline, is_newline};

pub fn parser() -> impl Parser<char, BlockHeading, Error = ParseError> {
    let token = just('#').repeated().at_least(1).map(|c| c.len());

    let content = filter(|c| !is_newline(c))
        .repeated()
        .at_least(1)
        .collect()
        .map_with_span(|content, span| Text { span, content });
    token
        .padded()
        .then(content)
        .map_with_span(|(level, content), span| BlockHeading {
            span,
            level,
            content,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_parse() {
        assert_eq!(
            parser().parse("### Hello World\n\n").unwrap(),
            BlockHeading {
                span: 0..15,
                content: Text {
                    span: 4..15,
                    content: "Hello World".to_string(),
                },
                level: 3
            }
        )
    }
}
