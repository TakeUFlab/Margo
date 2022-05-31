use chumsky::prelude::*;

use crate::types::{Inline, InlineLinethrough};

use super::error::ParseError;

pub fn parser<T>(r: T) -> impl Parser<char, InlineLinethrough, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.map_with_span(|content, span| InlineLinethrough {
        span,
        content: Box::new(content),
    })
    .delimited_by(just("~"), just("~"))
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn block_parse() {}
}
