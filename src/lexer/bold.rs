use chumsky::prelude::*;

use crate::types::{Inline, InlineBold};

use super::error::ParseError;

pub fn parser<T>(r: T) -> impl Parser<char, InlineBold, Error = ParseError>
where
    T: Parser<char, Inline, Error = ParseError>,
{
    r.delimited_by(just("*"), just("*"))
        .map_with_span(|content, span| InlineBold {
            span,
            content: Box::new(content),
        })
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn block_parse() {
    }
}
