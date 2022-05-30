use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{BlockHeading, Text};

use super::error::ParseError;
use super::utils::block_newline;

pub fn parser() -> impl Parser<char, BlockHeading, Error = ParseError> {
    let token = just('#')
        .repeated()
        .collect()
        .map_with_span(Token::HeadingTag)
        .map(Box::new);
    let content = take_until(block_newline())
        .map(|(chars, _)| chars)
        .collect()
        .map_with_span(|content, span| Text { span, content });
    token
        .padded()
        .then(content)
        .map_with_span(|(token, content), span| BlockHeading {
            span,
            token,
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
                span: 0..17,
                token: Box::new(Token::HeadingTag("###".to_string(), 0..3,)),
                content: Text {
                    span: 4..17,
                    content: "Hello World".to_string(),
                },
            }
        )
    }
}
