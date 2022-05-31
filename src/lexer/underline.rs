use chumsky::prelude::*;

use crate::types::{Inline, InlineUnderline};

use super::error::ParseError;

pub fn parser<T>(r: T) -> impl Parser<char, InlineUnderline, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineUnderline {
        span,
        content: Box::new(content),
    })
    .delimited_by(just("_"), just("_"))
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn block_parse() {}
}
