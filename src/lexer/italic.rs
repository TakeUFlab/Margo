use chumsky::prelude::*;

use crate::types::{Inline, InlineItalic};

use super::error::ParseError;

pub fn parser<T>(r: T) -> impl Parser<char, InlineItalic, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineItalic {
        span,
        content: Box::new(content),
    })
    .delimited_by(just("/"), just("/"))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
